use yew::prelude::*;

pub struct PlaylistEntry {
    title: String,
    onsignal: Option<Callback<()>>,
}

pub enum Msg {
    Clicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
    pub title: String,
    pub onsignal: Option<Callback<()>>,
}

impl Default for Props {
    fn default() -> Self {
        Props {
            title: "Unnamed Entry".into(),
            onsignal: None,
        }
    }
}

impl Component for PlaylistEntry {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        PlaylistEntry {
            title: props.title,
            onsignal: props.onsignal,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                if let Some(ref mut callback) = self.onsignal {
                    callback.emit(());
                }
            }
        }
        false
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.title = props.title;
        self.onsignal = props.onsignal;
        true
    }
}

impl Renderable<PlaylistEntry> for PlaylistEntry {
    fn view(&self) -> Html<Self> {
        html! {
            <div onclick=|_| Msg::Clicked, class="playlist_entry",>
                <p>{ format!("{}", self.title) }</p>
            </div>
        }
    }
}
