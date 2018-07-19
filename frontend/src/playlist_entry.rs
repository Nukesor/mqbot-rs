use yew::prelude::*;

pub struct PlaylistEntry {
    title: String,
    onclick_entry: Option<Callback<()>>,
    onclick_delete: Option<Callback<()>>,
}

pub enum Msg {
    ClickedEntry,
    ClickedDelete,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub onclick_entry: Option<Callback<()>>,
    pub onclick_delete: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "Unnamed Entry".into(),
            onclick_entry: None,
            onclick_delete: None,
        }
    }
}

impl Component for PlaylistEntry {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        PlaylistEntry {
            title: props.title,
            onclick_entry: props.onclick_entry,
            onclick_delete: props.onclick_delete,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ClickedEntry => {
                if let Some(ref mut callback) = self.onclick_entry {
                    callback.emit(());
                }
            }
            Msg::ClickedDelete => {
                if let Some(ref mut callback) = self.onclick_delete {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.onclick_entry = props.onclick_entry;
        self.onclick_delete = props.onclick_delete;
        true
    }
}

impl Renderable<PlaylistEntry> for PlaylistEntry {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="box playlist_entry",>
                { format!("{}", self.title) }
                <a class="button is-success is-small is-outlined", onclick=|_| Msg::ClickedEntry, >
                    <span class="icon is-small",>
                        <i class="fas fa-play",></i>
                    </span>
                    <span>{ "Play" }</span>
                </a>
                <a class="delete is-pulled-right", onclick=|_| Msg::ClickedDelete, ></a>
            </div>
        }
    }
}
