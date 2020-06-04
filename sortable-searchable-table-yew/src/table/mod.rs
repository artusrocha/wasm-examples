pub mod fetch;

//use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, InputData};
use yew::{html, MouseEvent, Component, ComponentLink, Html, ShouldRender};
use serde_derive::{Deserialize, Serialize};
//use rand::prelude::*;
//use uuid::Uuid;

//use crate::table::fetch;

pub struct Table {
    rows: Vec<Row>,
    link: ComponentLink<Self>,
}


#[derive(Clone)]
struct Row {
    sequence: usize,
    natural: u64,
    integer: i64,
    char: char,
}

#[derive(Serialize, Deserialize)]
pub enum Msg {
    Add,
    SetCurrent(String),
    ToggleIsDone(usize),
}

impl Component for Table {
    type Message = Msg;
    type Properties = ();

    fn create( _: Self::Properties, link: ComponentLink<Self>) -> Self {

        let mut rows = Vec::new();
        for n in 0..100 {
            let new_row = Table::gen_random_row(n);
            rows.push(new_row);
        }

        Table {
            rows: rows,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.rows.push( Table::gen_random_row( self.rows.len() ) );
                true // Indicate that the Component should re-render
            }
            Msg::SetCurrent(_val) => {
                true
            }
            Msg::ToggleIsDone(_index) => {
//                let mut Row = self.rows.get_mut(index).unwrap();
//                Row.is_done = !Row.is_done;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="main">
                <div id="container">
                    <div>{ "Row Manager" }</div>
                    <div>
                        { self.view_add_form() }
                        { self.view_rows_list() }
                    </div>
                </div>
            </div>
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }
    
}

impl Table {
    fn view_add_form(&self) -> Html{
        html! {
            <div>
                <input placeholder="Row description" />
                <button onclick=self.link.callback(|_: MouseEvent| Msg::Add ) >{ "Add Row" }</button>
            </div>
        }
    }

    fn view_rows_list(&self) -> Html {
        html! {
            <ul>
                { for self.rows.iter().enumerate()
                    .map(|(index, row)| self.view_row(index, &row) ) }
            </ul>
        }
    }

    fn view_row(&self, _index: usize, row: &Row) -> Html {
        html! {
            <li class="row">
                <div class="col col-2">
                    { &row.sequence}
                </div>
                <div class="col col-2">
                    { &row.integer}
                </div>
                <div class="col col-2">
                    { &row.natural}
                </div>
                <div class="col col-2">
                    
                </div>
                <div class="col col-2">
                    
                </div>
                <div class="col col-2">
                    { &row.char}
                </div>
            </li>
        }
    }

    fn gen_random_row(sequence: usize) -> Row {
        Row {
            sequence: sequence,
            natural: rand::random::<u64>(),
            integer: rand::random::<i64>(),
            char: rand::random::<char>(),
        }
    }
  
}

