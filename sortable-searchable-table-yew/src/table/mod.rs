use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, InputData};

use serde_derive::{Deserialize, Serialize};

pub struct Table {
    Rows: Vec<Row>,
    current: Row,
    link: ComponentLink<Self>,
}


#[derive(Clone)]
struct Row {
    description: String,
    is_done: bool,
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
        let current = Row {
            description: "".into(),
            is_done: false,
        };
        Table {
            Rows: Vec::new(),
            current: current,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.Rows.push( self.current.clone() );
                self.current = Row {
                    description: "".into(),
                    is_done: false,
                };
                true // Indicate that the Component should re-render
            }
            Msg::SetCurrent(val) => {
                println!("Input: {}", val);
                self.current.description = val;
                true
            }
            Msg::ToggleIsDone(index) => {
                let mut Row = self.Rows.get_mut(index).unwrap();
                Row.is_done = !Row.is_done;
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
                    { self.view_Rows_list() }
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
                <input placeholder="Row description"
                    value=&self.current.description
                    oninput=self.link.callback(|e: InputData| Msg::SetCurrent(e.value) ) />
                <button onclick=self.link.callback(|_: MouseEvent| Msg::Add ) >{ "Add Row" }</button>
            </div>
        }
    }

    fn view_Rows_list(&self) -> Html {
        html! {
            <ul>
                { for self.Rows.iter().enumerate()
                    .map(|(index, Row)| self.view_Row_row(index, &Row) ) }
            </ul>
        }
    }

    fn view_Row_row(&self, index: usize, Row: &Row) -> Html {
        html! {
            <li class="Row-row">
                <div class="col col-1">
                <input placeholder="Is done?" name="is_done" type="checkbox"
                    value=index
                    checked=Row.is_done
                    oninput=self.link.callback(move |_: InputData| { Msg::ToggleIsDone(index) }) />
                    // to force the closure to take ownership of `index` 
                    // (and any other referenced variables), use the `move` keyword
                </div>
                <div class="col col-10">
                { &Row.description }
                </div>
                <div class="col col-1">
                { "123" }
                </div>
            </li>
        }
    }    
}

