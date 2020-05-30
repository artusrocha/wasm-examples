mod todo;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::todo::ToDo;

//fn main() {
//    yew::start_app::<ToDo>();
//}

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::initialize();
    let document = yew::utils::document();
    let tags = document.get_elements_by_tag_name("ToDo");
//    App::<ToDo>::new().mount(tag.unwrap());
    for i in 0..tags.length() {
        App::<ToDo>::new().mount( tags.item(i).unwrap() ); //. .mount_to_body(); 
    }
}
