mod flash;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Html,
    routing::{get, get_service, post},
    Router,
};
use service::{
    sea_orm::{Database, DatabaseConnection},
    Mutation as MutationCore, Query as QueryCore,
};
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer}
};
use entity::anime;
use flash::get_flash_cookie;
use migration::{Migrator, MigratorTrait};
use serde::{Deserialize, Serialize};
use std::env;
use tera::Tera;
use tower_cookies::{CookieManagerLayer, Cookies};
use tower_http::decompression::DecompressionLayer;
use tracing::{info, Level};
use axum_typed_multipart::{TryFromMultipart, TypedMultipart};
use regex::Regex;

#[tokio::main]
async fn start() -> anyhow::Result<()> {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt().with_target(false).json().init();

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");

    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let templates = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"))
        .expect("Tera initialization failed");

    let state = AppState { templates, conn };

    let webhook_route = Router::new()
        .route("/events", post(new_plex_event));

    let form_routes = Router::new()
        .route("/", get(list_anime))
        .route("/:id", get(edit_anime))
        .nest_service(
            "/static",
            get_service(ServeDir::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/static"
            )))
            .handle_error(|error| async move {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Unhandled internal error: {error}"),
                )
            }),
        );

    let app = Router::new()
        .nest("/plex", webhook_route)
        .nest("/", form_routes)
        .layer(CookieManagerLayer::new())
        .layer(DecompressionLayer::new())
        .layer(TraceLayer::new_for_http()
                   .make_span_with(trace::DefaultMakeSpan::new()
                       .level(Level::INFO))
                   .on_response(trace::DefaultOnResponse::new()
                       .level(Level::INFO)),)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    info!("listening on: {}", listener.local_addr()?);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[derive(TryFromMultipart, Deserialize, Debug)]
struct PlexEventRequest {
    payload: String
}

#[derive(Deserialize, Debug)]
struct PlexEventPayload {
    event: String,
    #[serde(rename="Metadata")]
    metadata: PlexEventPayloadMetadata
}

#[derive(Deserialize, Debug)]
struct PlexEventPayloadMetadata {
    year: i32,
    guid: String,
    #[serde(rename="grandparentTitle")]
    grantparent_title: String,
    index: i32,
    #[serde(rename="parentIndex")]
    parent_index: i32
}

async fn new_plex_event(
    state: State<AppState>,
    data: TypedMultipart<PlexEventRequest>,
) -> Result<(), (StatusCode, &'static str)> {
    let p: PlexEventPayload = serde_json::from_str(&*data.payload).unwrap();
    println!("Event: {:?}", p.event);

    if p.event == "media.scrobble" {
        println!("Event: {:?}", p.metadata);
        let agent_source = p.metadata.guid.as_str();
        let re = Regex::new(r#"-([0-9]+)"#).unwrap();
        let agent_id = match agent_source {
            s if s.contains("anidb") => 1,
            s if s.contains("tvdb") => 2,
            _ => 0,
        };

        if let Some(number) = re.captures(agent_source).and_then(|captures| captures.get(1)) {
            let external_id = number.as_str().parse::<i32>().unwrap();
            println!("{}", external_id);

            let anime = MutationCore::find_or_create_anime(&state.conn, p.metadata.grantparent_title, agent_id, external_id)
                .await
                .expect("could not find or create anime");

            MutationCore::create_event(&state.conn, p.metadata.index, p.metadata.parent_index, anime.id)
                .await
                .expect("could not insert webhook event");
        }
    }

    Ok(())
}

#[derive(Clone)]
struct AppState {
    templates: Tera,
    conn: DatabaseConnection,
}

#[derive(Deserialize)]
struct Params {
    page: Option<u64>,
    anime_per_page: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct FlashData {
    kind: String,
    message: String,
}

async fn list_anime(
    state: State<AppState>,
    Query(params): Query<Params>,
    cookies: Cookies,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let page = params.page.unwrap_or(1);
    let anime_per_page = params.anime_per_page.unwrap_or(5);

    let (anime, num_pages) = QueryCore::find_anime_in_page(&state.conn, page, anime_per_page)
        .await
        .expect("Cannot find anime in page");

    let mut ctx = tera::Context::new();
    ctx.insert("anime", &anime);
    ctx.insert("page", &page);
    ctx.insert("anime_per_page", &anime_per_page);
    ctx.insert("num_pages", &num_pages);

    if let Some(value) = get_flash_cookie::<FlashData>(&cookies) {
        ctx.insert("flash", &value);
    }

    let body = state
        .templates
        .render("index.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

async fn edit_anime(
    state: State<AppState>,
    Path(id): Path<i32>,
) -> Result<Html<String>, (StatusCode, &'static str)> {
    let anime: anime::Model = QueryCore::find_anime_by_id(&state.conn, id)
        .await
        .expect("could not find anime")
        .unwrap_or_else(|| panic!("could not find anime with id {id}"));

    let mut ctx = tera::Context::new();
    ctx.insert("anime", &anime);

    let body = state
        .templates
        .render("edit.html.tera", &ctx)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Template error"))?;

    Ok(Html(body))
}

pub fn main() {
    let result = start();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}