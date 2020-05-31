use yew::{html, Callback, MouseEvent, Component, ComponentLink, Html, ShouldRender, InputData};

use serde_derive::{Deserialize, Serialize};

pub struct ToDo {
    tasks: Vec<Task>,
    current: Task,
    link: ComponentLink<Self>,
}


#[derive(Clone)]
struct Task {
    description: String,
    is_done: bool,
}

#[derive(Serialize, Deserialize)]
pub enum Msg {
    Add,
    SetCurrent(String),
    ToggleIsDone(usize),
}

impl Component for ToDo {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let current = Task {
            description: "".into(),
            is_done: false,
        };
        ToDo {
            tasks: Vec::new(),
            current: current,
            link,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Add => {
                self.tasks.push( self.current.clone() );
                self.current = Task {
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
                let mut task = self.tasks.get_mut(index).unwrap();
                task.is_done = !task.is_done;
                true
            }
        }
    }

    fn view(&self) -> Html {
        html! {
            <div id="container">
                <div>{ "Task Manager" }</div>
                <div>
                    { self.view_add_form() }
                    { self.view_tasks_list() }
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

impl ToDo {
    fn view_add_form(&self) -> Html{
        html! {
            <div>
                <input placeholder="Task description"
                    value=&self.current.description
                    oninput=self.link.callback(|e: InputData| Msg::SetCurrent(e.value) ) />
                <button onclick=self.link.callback(|_: MouseEvent| Msg::Add ) >{ "Add Task" }</button>
            </div>
        }
    }

    fn view_tasks_list(&self) -> Html {
        html! {
            <ul>
                { for self.tasks.iter().enumerate()
                    .map(|(index, task)| self.view_task_row(index, &task) ) }
            </ul>
        }
    }

    fn view_task_row(&self, index: usize, task: &Task) -> Html {
        html! {
            <li class="task-row">
                <div class="col col-1">
                <input placeholder="Is done?" name="is_done" type="checkbox"
                    value=index
                    checked=task.is_done
                    oninput=self.link.callback(move |_: InputData| { Msg::ToggleIsDone(index) }) />
                    // to force the closure to take ownership of `index` 
                    // (and any other referenced variables), use the `move` keyword
                </div>
                <div class="col col-10">
                { &task.description }
                </div>
                <div class="col col-1">
                { "123" }
                </div>
            </li>
        }
    }    
}

