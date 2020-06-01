use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, InputData};
use uuid::Uuid;
use serde_derive::{Deserialize, Serialize};
use rand::prelude::*;


pub struct Table {
    rows: Vec<Row>,
    current: Row,
    link: ComponentLink<Self>,
}


#[derive(Clone)]
struct Row {
    integer: i64,
//    float: f64,
//    string: String,
//    boolean: bool,
//    uuid: Option<Uuid>,
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

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut rows = Vec::new();
        let current = Row {
            integer: 1,//random::<i64>(),
//            float: random::<f64>(),
//            string: random::<char>().to_string(),
//            boolean: random::<bool>(),
//            uuid: Uuid::new_v4(),
        };
        rows.push(current.clone());
        Table {
            rows: rows,
            current: current,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.rows.push( self.current.clone() );
                self.current = Row {
            integer: 2,//random::<i64>(),
//            float: random::<f64>(),
//            string: random::<char>().to_string(),
//            boolean: random::<bool>(),
//            uuid: Uuid:: new_v4(),
        };
                true // Indicate that the Component should re-render
            }
            Msg::SetCurrent(val) => {
                true
            }
            Msg::ToggleIsDone(index) => {
//                let mut Row = self.rows.get_mut(index).unwrap();
//                Row.is_done = !Row.is_done;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="container">
                <div>{ "Row Manager" }</div>
                <div>
                    { self.view_add_form() }
                    { self.view_rows_list() }
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

    fn view_row(&self, index: usize, row: &Row) -> Html {
        html! {
            <li class="Row-row">
                <div class="col col-2">
                    { &row.integer}
                </div>
                <div class="col col-2">
                    { "&row.string" }
                </div>
                <div class="col col-2">
                    { "&row.boolean" }
                </div>
                <div class="col col-2">
                    { "&row.uuid" }
                </div>
            </li>
        }
    }    
}

