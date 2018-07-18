#[macro_use]
extern crate yew;
use yew::prelude::*;

mod playlist;
mod playlist_entry;
mod video_viewer;

use playlist::Playlist;
use video_viewer::VideoViewer;

struct Model { }

enum Msg { }

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <p>{ "outa" }</p>
            <div class="mqbot",>
                <div class="playlist",>
                    <Playlist: />
                </div>
                <div class="viewport",>
                    <VideoViewer: />
                </div>
            </div>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Model>::new().mount_to_body();
    yew::run_loop();
}
