
pub mod fetch;

//use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, InputData};
use yew::{html, MouseEvent, Component, ComponentLink, Html, ShouldRender, InputData, KeyboardEvent};
use serde_derive::{Deserialize, Serialize};

use rand::{random, thread_rng, Rng};
use rand::distributions::Alphanumeric;


pub struct Table {
    rows: Vec<Row>,
    link: ComponentLink<Self>,
    sort_by: String,
    searchable_example: RowSearcheableExample,
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

#[derive(Clone)]
struct RowSearcheableExample {
    sequence: Option<String>,
    natural: Option<String>,
    integer: Option<String>,
    float: Option<String>,
    char: Option<String>,
    string: Option<String>,
}


#[derive(Serialize, Deserialize)]
pub enum Msg {
    Add,
    SortBy(String),
    Search(String, String),
    Filter,
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
            searchable_example: RowSearcheableExample {
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
            Msg::Filter => {
                true
            }
            Msg::Nope => { false }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="main">
                <div id="container">
                    <div>
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
        Table::filter_field( self.searchable_example.sequence.as_deref(), row.sequence.unwrap().to_string() )
        && Table::filter_field( self.searchable_example.integer.as_deref(), row.integer.unwrap().to_string() )
        && Table::filter_field( self.searchable_example.natural.as_deref(), row.natural.unwrap().to_string() )
        && Table::filter_field( self.searchable_example.float.as_deref(), row.float.unwrap().to_string() )
        && Table::filter_field( self.searchable_example.char.as_deref(), row.char.unwrap().to_string() )
        && Table::filter_field( self.searchable_example.string.as_deref(), row.string.as_ref().unwrap().clone() )
    }

    fn filter_field(example: Option<&str>, field_str_val: String) -> bool {
        example 
            .and_then(|val| Some( field_str_val.contains(val)) ) 
            .unwrap_or(true) 
    }

    fn view_row(&self, _index: usize, row: &Row) -> Html {
        html! {
            <tr class="row">
                { self.view_col(0, &row.sequence.unwrap().to_string() ) }
                { self.view_col(1, &row.integer.unwrap().to_string()  ) }
                { self.view_col(2, &row.natural.unwrap().to_string()  ) }
                { self.view_col(3, &row.float.unwrap().to_string()    ) }
                { self.view_col(3, &row.char.unwrap().to_string()    ) }
                { self.view_col(4, &row.string.clone().unwrap()       ) }
                { self.view_control_col() }
            </tr>
        }
    }

    fn view_col(&self, _n: usize, val: &String) -> Html {
        html!{
            <td class="col">
                    { val }
            </td>
        }
    }

    fn view_control_col(&self) -> Html {
        html!{
            <td class="col control">
                <i class="fa fa-trash"></i>
            </td>
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
                <th class="col control"><br/><i class="fa fa-search"></i></th>
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
        html! {
            <input placeholder="filter" class="search"
                oninput=self.link.callback(move |q: InputData| Msg::Search(key.clone(), q.value) ) 
                onkeypress=self.link.callback(move |k: KeyboardEvent| 
                    if k.key() == "Enter" { Msg::Filter }
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
                _ => { },
            };
        }
        self.sort_by = key;
    }

    fn search(&mut self, key: String, query: String) {
        match &key[..] {
            "seq" => self.searchable_example.sequence = if query.is_empty() { None } else { Some(query) },
            "integer" => self.searchable_example.integer = if query.is_empty() { None } else { Some(query) },
            "natural" => self.searchable_example.natural = if query.is_empty() { None } else { Some(query) },
            "float" => self.searchable_example.float = if query.is_empty() { None } else { Some(query) },
            "char" => self.searchable_example.char = if query.is_empty() { None } else { Some(query) },
            "string" => self.searchable_example.string = if query.is_empty() { None } else { Some(query) },
            _ => { },
        }
    }

    fn gen_random_row(sequence: usize) -> Row {
        Row {
            sequence: Some( sequence ),
            natural: Some( random::<u64>() ),
            integer: Some( random::<i64>() ),
            float: Some( random::<f64>() ),
            char: Some( Table::gen_random_string(1).pop().unwrap() ),
            string: Some( Table::gen_random_string(20) ),
            
        }
    }

    fn gen_random_string(n: usize) -> String {
        thread_rng()
            .sample_iter(&Alphanumeric)
            .take(n)
            .collect()
    }
  
}

