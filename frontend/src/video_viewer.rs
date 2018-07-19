use yew::prelude::*;

pub struct VideoViewer {
}

pub enum Msg {
}

#[derive(PartialEq, Clone)]
pub struct Props {
}

impl Default for Props {
    fn default() -> Self {
        Props {
        }
    }
}

impl Component for VideoViewer {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _: ComponentLink<Self>) -> Self {
        VideoViewer {
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }
}

impl Renderable<VideoViewer> for VideoViewer {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="video-viewer",>
                <figure class="image is-4by3",>
                    <img src="https://bulma.io/images/placeholders/640x480.png",/>
                </figure>
            </div>
        }
    }
}
