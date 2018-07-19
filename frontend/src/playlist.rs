use yew::prelude::*;

use playlist_entry::PlaylistEntry;

pub struct Playlist {
    entries: Vec<String>,
    current_entry: String,
}

pub enum Msg {
    AddEntry,
    UpdateCurrentEntry(String),
    PlayEntry(usize),
    DeleteEntry(usize),
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

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Playlist {
            entries: vec!["https://bulma.io/".to_string()],
            current_entry: "".to_string(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddEntry => {
                self.entries.push(self.current_entry.clone());
                self.current_entry.clear();
            },
            Msg::UpdateCurrentEntry(val) => self.current_entry = val,
            Msg::PlayEntry(val) => println!("{}", val),
            Msg::DeleteEntry(val) => { self.entries.remove(val); },
            Msg::ClearPlaylist => self.entries.clear(),
            Msg::NoOp => {},
        }
        true
    }
}

impl Renderable<Playlist> for Playlist {
    fn view(&self) -> Html<Self> {
        let playlist_entry = |(i, val): (usize, &String)| html! {
            <li>
                <PlaylistEntry: title=val, onclick_entry=move |_| Msg::PlayEntry(i), onclick_delete=move |_| Msg::DeleteEntry(i), />
            </li>
        };
        html! {
            <div class="playlist",>
                <ul>
                    { for self.entries.iter().enumerate().map(playlist_entry) }
                </ul>
                <div class="field",>
                    <label class="label",>{ "Media URL" }</label>
                    <div class="control",>
                        <div class="field has-addons",>
                            <div class="control",>
                                <input value=&self.current_entry,
                                    class="input",
                                    type="text",
                                    placeholder="https://youtu.be/4dTvF53buTc",
                                    oninput=|e| Msg::UpdateCurrentEntry(e.value),
                                    onkeypress=|e| {
                                        if e.key() == "Enter" { Msg::AddEntry } else { Msg::NoOp }
                                    },
                                />
                            </div>
                            <div class="control",>
                                <a onclick=|_| Msg::AddEntry, class="button is-info",>{ "Add" }</a>
                            </div>
                        </div>
                    </div>
                    <p class="help",>{ "Try any public media URL" }</p>
                </div>
                <div class="control",>
                    <a class="button is-danger", onclick=|_| Msg::ClearPlaylist, >{ "Clear playlist" }</a>
                </div>
            </div>
        }
    }
}
