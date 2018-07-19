use yew::prelude::*;

use playlist_entry::PlaylistEntry;

pub struct Playlist {
    entries: Vec<String>,
    current_entry: String,
}

pub enum Msg {
    AddEntry,
    UpdateCurrentEntry(String),
    EntryClicked(usize),
    ClearPlaylist,
    NoOp,
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
            current_entry: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddEntry => self.entries.push(self.current_entry.clone()),
            Msg::UpdateCurrentEntry(val) => self.current_entry = val,
            Msg::EntryClicked(val) => println!("{}", val),
            Msg::ClearPlaylist => self.entries.clear(),
            Msg::NoOp => {},
        }
        true
    }
}

impl Renderable<Playlist> for Playlist {
    fn view(&self) -> Html<Self> {
        let playlist_entry = |(i, val): (usize, &String)| html! {
            <PlaylistEntry: title=val, onsignal=|_| Msg::EntryClicked(i.clone()), />
        };
        html! {
            { for self.entries.iter().enumerate().map(playlist_entry) }
            <input value=&self.current_entry,
                   type="text",
                   oninput=|e| Msg::UpdateCurrentEntry(e.value),
                   onkeypress=|e| {
                       if e.key() == "Enter" { Msg::AddEntry } else { Msg::NoOp }
                   },
            />
        }
    }
}
