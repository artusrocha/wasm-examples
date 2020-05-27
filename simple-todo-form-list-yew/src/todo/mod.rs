use yew::{html, Callback, ClickEvent, Component, ComponentLink, Html, ShouldRender, InputData};

use serde_derive::{Deserialize, Serialize};
//extern crate is_even;
//use is_even::IsEven;


pub struct ToDo {
    tasks: Vec<Task>,
    current: Task,
    add: Callback<ClickEvent>,
    link: ComponentLink<Self>,
}


#[derive(Clone)]
struct Task {
    description: String,
}

#[derive(Serialize, Deserialize)]
pub enum Msg {
    Add,
    SetCurrent(String),
}

impl Component for ToDo {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let current = Task {
            description: "".into(),
        };
        ToDo {
            tasks: Vec::new(),
            current: current,
            add: link.callback(|_| Msg::Add),
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.tasks.push( self.current.clone() );
                self.current = Task {
                    description: "".into(),
                };
                true // Indicate that the Component should re-render
            }
            Msg::SetCurrent(val) => {
                println!("Input: {}", val);
                self.current.description = val;
                true
            }
        }
    }

    fn view(&self) -> Html {
//        let button_text = if self.counter > 0 { "add another row" } else { "add row" };
        html! {
            <div>
                <div>{ "Task Manager" }</div>
                <div>
                    { self.view_add_form() }
                    { self.view_tasks_list() }
                </div>
            </div>
        }
    }   
}

impl ToDo {
    fn view_add_form(&self) -> Html{
        html! {
            <div>
                <input placeholder="Task description"
                    value=&self.current.description
                    oninput=self.link.callback(|e: InputData| Msg::SetCurrent(e.value) ) />
                <button onclick=&self.add>{ "Add Task" }</button>
            </div>
        }
    }

    fn view_tasks_list(&self) -> Html {
        html! {
            <ul>
                { for self.tasks.iter().map(|e| self.view_task_row(e) ) }
            </ul>
        }
    }

    fn view_task_row(&self, task: &Task) -> Html {
        html! {
            <li> { &task.description } </li>
        }
    }
    
}

