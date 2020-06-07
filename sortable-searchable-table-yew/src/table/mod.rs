pub mod fetch;

//use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, InputData};
use yew::{html, MouseEvent, Component, ComponentLink, Html, ShouldRender, InputData, KeyboardEvent};
use serde_derive::{Deserialize, Serialize};

use regex::Regex;

pub struct Table {
    rows: Vec<Row>,
    link: ComponentLink<Self>,
    sort_by: String,
    searchable_example: Row,
}


#[derive(Clone)]
struct Row {
    sequence: Option<usize>,
    natural: Option<u64>,
    integer: Option<i64>,
    float: Option<f64>,
    char: Option<char>,
    string: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub enum Msg {
    Add,
    SortBy(String),
    Search(String, String),
    Filter(String),
    Nope,
}

impl Component for Table {
    type Message = Msg;
    type Properties = ();

    fn create( _: Self::Properties, link: ComponentLink<Self>) -> Self {

        let mut rows = Vec::new();
        for n in 0..1000 {
            let new_row = Table::gen_random_row(n);
            rows.push(new_row);
        }

        Table {
            rows: rows,
            link,
            sort_by: "seq".into(),
            searchable_example: Row {
                sequence: None,
                natural: None,
                integer: None,
                float: None,
                char: None,
                string: None,
            }
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.rows.push( Table::gen_random_row( self.rows.len() ) );
                true // Indicate that the Component should re-render
            }
            Msg::SortBy(key) => {
                self.sort_by(key);
                true
            }
            Msg::Search(key, query) => {
                self.search(key, query);
                false
            }
            Msg::Filter(key) => {
                true
            }
            Msg::Nope => { false }
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
                { for self.rows.iter()
                    .filter(|row| self.do_filter(row))
                    .enumerate()
                    .map(|(index, row)| self.view_row(index, &row) ) }
            </table>
        }
    }

    fn do_filter(&self, row: &Row) -> bool {
        self.searchable_example.sequence
            .and_then(|val| Some( val.to_string() ) )
            .and_then(|val| Some( row.sequence.unwrap().to_string().contains(&val) ) )
            .unwrap_or(true)
        //true
    }

    fn view_row(&self, _index: usize, row: &Row) -> Html {
        let string = &row.string.clone().unwrap();
        html! {
            <tr class="row">
                <td class="col">
                    { &row.sequence.unwrap() }
                </td>
                <td class="col">
                    { &row.integer.unwrap() }
                </td>
                <td class="col">
                    { &row.natural.unwrap() }
                </td>
                <td class="col">
                    { &row.float.unwrap() }
                </td>
                <td class="col">
                    { &row.char.unwrap() }
                </td>
                 <td class="col">
                    { &string.clone() }
                </td>
            </tr>
        }
    }

    fn view_tab_head(&self) -> Html {
        html!{
            <tr>
                { self.view_tab_head_col( "seq".into()     ) }
                { self.view_tab_head_col( "integer".into() ) }
                { self.view_tab_head_col( "natural".into() ) }
                { self.view_tab_head_col( "float".into()   ) }
                { self.view_tab_head_col( "char".into()    ) }
                { self.view_tab_head_col( "string".into()  ) }
            </tr>
        }
    }

    fn view_tab_head_col(&self, key: String) -> Html {
        let key_cp = key.clone();
        html!{
            <th> 
                <div onclick=self.link.callback(move |_: MouseEvent| Msg::SortBy( key.clone() ) )>
                    { key_cp.clone() } <i class="arrow down"></i>
                </div>
                { self.view_search_form(key_cp) }
            </th>
        }
    }

    fn view_search_form(&self, key: String) -> Html {
        let key_cp = key.clone();
        html! {
            <input placeholder="filter" class="search"
                oninput=self.link.callback(move |q: InputData| Msg::Search(key.clone(), q.value) ) 
                onkeypress=self.link.callback(move |k: KeyboardEvent| 
                    if k.key() == "Enter" { Msg::Filter(key_cp.clone()) }
                    else { Msg::Nope } ) >
            </input>
        }
    }

    fn sort_by(&mut self, key: String) {
        if key == self.sort_by {
            self.rows.reverse();
        } else {
            match &key[..] {
                "seq" => self.rows.sort_by_cached_key(|row| row.sequence),
                "integer" => self.rows.sort_by_cached_key(|row| row.integer),
                "natural" => self.rows.sort_by_cached_key(|row| row.natural),
                "float" => self.rows.sort_by_cached_key(|row| ordered_float::OrderedFloat(row.float.unwrap()) ),
                "char" => self.rows.sort_by_cached_key(|row| row.char),
                "string" => self.rows.sort_by_cached_key(|row| row.string.clone()),
                _ => self.rows.sort_by_key(|row| row.sequence),
            };
        }
        self.sort_by = key;
    }

    fn search(&mut self, key: String, query: String) {
        match &key[..] {
            "seq" => self.searchable_example.sequence = Some( query.parse::<usize>().unwrap() ),
            "integer" => self.searchable_example.integer = Some( query.parse::<i64>().unwrap() ),
            "natural" => self.searchable_example.natural = Some( query.parse::<u64>().unwrap() ),
            "float" => self.searchable_example.float = Some( query.parse::<f64>().unwrap() ),
            "char" => self.searchable_example.char = Some( query.parse::<char>().unwrap() ),
            "string" => self.searchable_example.string = Some( query ),
            _ => { },
        }
    }

    fn gen_random_row(sequence: usize) -> Row {
        Row {
            sequence: Some( sequence ),
            natural: Some( rand::random::<u64>() ),
            integer: Some( rand::random::<i64>() ),
            float: Some( rand::random::<f64>() ),
            char: Some( rand::random::<char>() ),
            string: Some( base64::encode( format!("{}",rand::random::<i64>()) ) ),
        }
    }
  
}

