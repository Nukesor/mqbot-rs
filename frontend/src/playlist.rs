use yew::prelude::*;

use playlist_entry::PlaylistEntry;

pub struct Playlist {
    entries: Vec<String>,
}

pub enum Msg {
    PlaylistEntryClicked,
}

#[derive(PartialEq, Clone)]
pub struct Props {
}

impl Default for Props {
    fn default() -> Self {
        Props {}
    }
}

impl Component for Playlist {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Playlist {
            entries: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::PlaylistEntryClicked => self.entries.clear()
        }
        true
    }
}

impl Renderable<Playlist> for Playlist {
    fn view(&self) -> Html<Self> {
        let playlist_entry = |x| html! {
            <PlaylistEntry: title=x, onsignal=|_| Msg::PlaylistEntryClicked, />
        };
        html! {
            { for self.entries.iter().map(playlist_entry) }
        }
    }
}
