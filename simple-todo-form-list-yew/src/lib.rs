mod todo;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::todo::ToDo;

//fn main() {
//    yew::start_app::<ToDo>();
//}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<ToDo>::new().mount_to_body();
}
