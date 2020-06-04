mod table;

use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::table::Table;



#[wasm_bindgen(start)]
pub async fn run_app() {

    let _result = table::fetch::fetch("https://8000-e1ce7c57-cf46-4cec-bebb-2185a34276d3.ws-us02.gitpod.io/teste.json".into()).await;

    yew::initialize();
    let document = yew::utils::document();
    let tags = document.get_elements_by_tag_name("Target");
    for i in 0..tags.length() {
        let tag = tags.item(i).unwrap();
        let _app = App::<Table>::new().mount( tag ); 
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    pub hello: String,
}