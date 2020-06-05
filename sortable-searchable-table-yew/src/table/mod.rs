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
    SortBy(String),
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
            Msg::SortBy(key) => {
                self.sort_by(key);
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
            <table>
                { self.view_tab_head() }
                { for self.rows.iter().enumerate()
                    .map(|(index, row)| self.view_row(index, &row) ) }
            </table>
        }
    }

    fn view_row(&self, _index: usize, row: &Row) -> Html {
        html! {
            <tr class="row">
                <td class="col">
                    { &row.sequence}
                </td>
                <td class="col">
                    { &row.integer}
                </td>
                <td class="col">
                    { &row.natural}
                </td>
                <td class="col">
                    { &row.char}
                </td>
            </tr>
        }
    }

    fn view_tab_head(&self) -> Html {
        html!{
            <tr>
                { self.view_tab_head_col( "seq".into() ) }
                { self.view_tab_head_col( "integer".into() ) }
                { self.view_tab_head_col( "natural".into() ) }
                { self.view_tab_head_col( "char".into() ) }
            </tr>
        }
    }

    fn view_tab_head_col(&self, key: String) -> Html {
        let key_cp = key.clone();
        html!{
            <th onclick=self.link.callback(move |_: MouseEvent| Msg::SortBy( key.clone() ) )>
                    { key_cp } <i class="arrow down"></i>
            </th>
        }
    }

    fn sort_by(&mut self, key: String) {
        match &key[..] {
            "seq" => self.rows.sort_by_key(|row| row.sequence),
            "integer" => self.rows.sort_by_key(|row| row.integer),
            "natural" => self.rows.sort_by_key(|row| row.natural),
            "char" => self.rows.sort_by_key(|row| row.char),
            _ => self.rows.sort_by_key(|row| row.sequence),
        };
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

