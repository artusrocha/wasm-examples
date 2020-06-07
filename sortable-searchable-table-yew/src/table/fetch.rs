use web_sys::{Request, RequestInit, RequestMode, Response};
use wasm_bindgen_futures::JsFuture;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

//#[derive(Serialize, Deserialize)]
pub async fn fetch(url: String) -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let req = Request::new_with_str_and_init(&url, &opts)?;
    req.headers().set("Accept", "application/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&req)).await?;

    let resp: Response = resp_value.dyn_into().unwrap();
    let resp_obj = JsFuture::from(resp.json()?).await?;
    Ok(resp_obj)

}
