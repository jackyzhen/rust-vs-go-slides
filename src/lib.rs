#[macro_use]
extern crate yew;
extern crate pulldown_cmark;

#[macro_use]
mod markdown;
mod slides;

use yew::prelude::*;
use markdown::render_markdown;
use slides::SLIDE_MARKDOWN;

pub struct Model {
    slides: Vec<& 'static str>,
    slide_idx: usize
}

pub enum Msg {
    GotKeyPress(KeyDownEvent)
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Model {
            slides: SLIDE_MARKDOWN
                .split("---\n")
                .map(|s|s.trim())
                .filter(|s|s != &"")
                .collect(),
            slide_idx: 0,
        }
    }

    fn update(&mut self, cmd: Self::Message) -> ShouldRender {
        match cmd {
            Msg::GotKeyPress(event) => {
                match event.key().as_str() {
                    "ArrowLeft" | "Backspace" => {
                        if self.slide_idx > 0 {
                            self.slide_idx -= 1;
                            true
                        } else {
                            false
                        }
                    },
                    "ArrowRight" | "Enter" => {
                        if self.slide_idx < self.slides.len() -1{
                            self.slide_idx += 1;
                            true
                        } else {
                            false
                        }
                    },
                    _ => {
                        false
                    }
                }
            }
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div id="slide-area",
            tabindex="-1",
            onkeydown=|e| Msg::GotKeyPress(e),
            >
            {render_markdown(self.slides[self.slide_idx])}
            </div>
        }
    }
}
