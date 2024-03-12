use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tower_cookies::{Cookies};

#[derive(Deserialize)]
struct ValuedMessage<T> {
    #[serde(rename = "_")]
    value: T,
}

#[derive(Serialize)]
struct ValuedMessageRef<'a, T> {
    #[serde(rename = "_")]
    value: &'a T,
}

const FLASH_COOKIE_NAME: &str = "_flash";

pub fn get_flash_cookie<T>(cookies: &Cookies) -> Option<T>
where
    T: DeserializeOwned,
{
    cookies.get(FLASH_COOKIE_NAME).and_then(|flash_cookie| {
        if let Ok(ValuedMessage::<T> { value }) = serde_json::from_str(flash_cookie.value()) {
            Some(value)
        } else {
            None
        }
    })
}
