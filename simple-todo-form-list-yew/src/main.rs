use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

//extern crate is_even;
use is_even::IsEven;

struct ToDo {
    clicked: bool,
    counter: i32,
    plus_row: Callback<ClickEvent>,
    minus_row: Callback<ClickEvent>,

}

enum Msg {
    Plus,
    Minus,
}

impl Component for ToDo {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ToDo {
            clicked: false,
            counter: 0,
            plus_row: link.callback(|_| Msg::Plus),
            minus_row: link.callback(|_| Msg::Minus),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Plus => {
                self.clicked = true;
                self.counter = self.counter + 1;
                true // Indicate that the Component should re-render
            }
            Msg::Minus => {
                self.clicked = true;
                self.counter = if self.counter > 0 { self.counter - 1 } else { self.counter };
                true // Indicate that the Component should re-render
            }
        }
    }

    fn view(&self) -> Html {
        let button_text = if self.counter > 0 { "add another row" } else { "add row" };
        html! {
            <div>
                <div>{ "Total rows: " }{ self.counter }</div>
                <div>
                    <button onclick=&self.plus_row>{ button_text }</button>
                    {   if self.counter > 0 {
                            html!{ <button onclick=&self.minus_row>{ "Remove row" }</button> }
                        } else { html!{ <span/> } }
                    }
                </div>
                { rows(self.counter) }
                { basic_style() }
            </div>
        }
    }

    
}

fn rows(counter: i32) -> Html {
    html! {
        <table>{
            for (1..=counter).map(|i| row(i) )
        }</table>
    }
    
    
}

fn row(row_index: i32) -> Html {
    html! {
        <tr>
            <td>{ row_index }</td>
            <td>{ "Hello World!" }</td>
            <td>{ if row_index.is_even() {"even"} else {"odd"} }</td>
        </tr>
    }
}

fn basic_style() -> Html{
    html! {
        <link rel="stylesheet" type="text/css" href="style.css"/>
    }
}

fn main() {
    yew::start_app::<ToDo>();
}
