mod todo;

//use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

use crate::todo::ToDo;

fn main() {
    yew::start_app::<ToDo>();
}
