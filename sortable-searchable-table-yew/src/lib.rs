mod table;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::table::Table;


#[wasm_bindgen(start)]
pub fn run_app() {
    yew::initialize();
    let document = yew::utils::document();
    let tags = document.get_elements_by_tag_name("Target");
    for i in 0..tags.length() {
        App::<Table>::new().mount( tags.item(i).unwrap() ); 
    }
}
