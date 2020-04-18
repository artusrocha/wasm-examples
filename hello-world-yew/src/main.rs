use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender};

//extern crate is_even;
use is_even::IsEven;

struct HelloWorld {
    clicked: bool,
    counter: i32,
    plus_row: Callback<ClickEvent>,
    minus_row: Callback<ClickEvent>,

}

enum Msg {
    Plus,
    Minus,
}

impl Component for HelloWorld {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        HelloWorld {
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
        let button_text = if self.clicked { "add another row" } else { "add row" };
        html! {
            <div>
                <div>{ "Total rows: " }{ self.counter }</div>
                <div>
                    <button onclick=&self.plus_row>{ button_text }</button>
                    {   if self.counter > 0 { html!{ <button onclick=&self.minus_row>{ "Remove row" }</button> } }
                        else { html!{ <span/> } }
                    }
                </div>
                { rows(self.counter) }
            </div>
        }
    }

    
}

fn rows(counter: i32) -> Html {
    html! {
        <table>{
            for (1..=counter).map(|i| html! {
                <tr>
                    <td>{format!{"Hello World! :: {}", i}}</td>
                    <td>{ if i.is_even() {"even"} else {"odd"} }</td>


                </tr>
            })
        }</table>
    }
    
    
}


fn main() {
    yew::start_app::<HelloWorld>();
}
