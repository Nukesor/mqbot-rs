#[macro_use]
extern crate yew;
use yew::prelude::*;

mod playlist;
mod playlist_entry;
mod video_viewer;

use playlist::Playlist;
use video_viewer::VideoViewer;

struct Root { }

enum Msg { }

impl Component for Root {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Root { }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
        }
    }
}

impl Renderable<Root> for Root {
    fn view(&self) -> Html<Self> {
        html! {
            <section class="section",>
                <div class="container",>
                    <h1 class="title",>{ "mqbot" }</h1>
                    <p class="subtitle",>{ "hallo arne" }</p>
                    <div class="columns",>
                      <div class="column is-one-third playlist",><Playlist: /></div>
                      <div class="column viewport",><VideoViewer: /></div>
                    </div>
                </div>
            </section>
        }
    }
}

fn main() {
    yew::initialize();
    App::<Root>::new().mount_to_body();
    yew::run_loop();
}
