#[macro_use]
extern crate yew;

extern crate pulldown_cmark;

extern crate stdweb;

mod markdown;
pub mod model;
mod slides;

use markdown::render_markdown;
use model::{Model, Msg, TransitionType};
use yew::prelude::*;

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Model::new(link)
    }

    fn update(&mut self, cmd: Self::Message) -> ShouldRender {
        match cmd {
            Msg::Transition(t_type, next_slide) => self.handle_transition(t_type, next_slide),
            Msg::GotKeyPress(event) => match event.key().as_str() {
                "ArrowLeft" | "Backspace" => {
                    if self.slide_idx > 0 {
                        let next_slide = self.slide_idx - 1;
                        if let Some(mut task) = self.handler.take() {
                            task.cancel();
                        }
                        self.transition(TransitionType::Fade, next_slide);
                        true
                    } else {
                        false
                    }
                }
                "ArrowRight" | "Enter" => {
                    if self.slide_idx < self.slides.len() - 1 {
                        let next_slide = self.slide_idx + 1;
                        if let Some(mut task) = self.handler.take() {
                            task.cancel();
                        }

                        self.transition(TransitionType::Fade, next_slide);
                        true
                    } else {
                        false
                    }
                }
                _ => false,
            },
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div style={format!("opacity: {}", self.opacity)},
            tabindex="-1",
            onkeydown=|e| Msg::GotKeyPress(e),
            >
            {render_markdown(self.slides[self.slide_idx])}
            </div>
        }
    }
}
