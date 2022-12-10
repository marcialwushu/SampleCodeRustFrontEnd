struct TodoApp {
    link: ComponentLink<Self>,
    todos: Option<Vec<Todo>>,
    fetch_task: Option<FetchTask>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
    pub user_id: u64,
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

enum Msg {
    MakeReq,
    Resp(Result<Vec<Todo>, anyhow::Error>),
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            todos: None,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=classes!("todo")>
                ...
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<TodoApp>::new().mount_to_body();
}