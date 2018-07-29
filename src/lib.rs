#[macro_use]
extern crate yew;
extern crate pulldown_cmark;

#[macro_use]
mod markdown;
mod slides;

use std::time::Duration;
use yew::prelude::*;
use yew::services::{TimeoutService, Task};
use markdown::render_markdown;
use slides::SLIDE_MARKDOWN;

pub struct Model {
    slides: Vec<& 'static str>,
    slide_idx: usize,
    timeout: TimeoutService,
    link: ComponentLink<Model>,
    transition: f32,
    job: Option<Box<Task>>,
}

pub enum Msg {
    GotKeyPress(KeyDownEvent),
    Transition(f32),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut timeout = TimeoutService::new();
        let send_msg = link.send_back(|_| Msg::Transition(0.0));
        let handle = timeout.spawn(Duration::from_millis(100), send_msg);
        Model {
            link,
            timeout,
            transition: 0.0,
            slides: SLIDE_MARKDOWN
                .split("---\n")
                .map(|s|s.trim())
                .filter(|s|s != &"")
                .collect(),
            slide_idx: 0,
            job: Some(Box::new(handle)),
        }
    }

    fn update(&mut self, cmd: Self::Message) -> ShouldRender {
        match cmd {
            Msg::Transition(value) => {
                if value >= 1.0 {
                    self.job = None;
                    false
                } else {
                    if let Some(mut task) = self.job.take() {
                        task.cancel();
                    }
                    self.transition = value;
                    let send_msg = self.link.send_back(move|_| Msg::Transition(value + 0.1));
                    let handle = self.timeout.spawn(Duration::from_millis(100), send_msg);
                    self.job = Some(Box::new(handle));
                    true
                }
            }
            Msg::GotKeyPress(event) => {
                match event.key().as_str() {
                    "ArrowLeft" | "Backspace" => {
                        if self.slide_idx > 0 {
                            self.slide_idx -= 1;
                            self.transition = 0.0;
                            if let Some(mut task) = self.job.take() {
                                task.cancel();
                            }
                            let send_msg = self.link.send_back(|_| Msg::Transition(0.1));
                            let handle = self.timeout.spawn(Duration::from_millis(100), send_msg);
                            self.job = Some(Box::new(handle));
                            true
                        } else {
                            false
                        }
                    },
                    "ArrowRight" | "Enter" => {
                        if self.slide_idx < self.slides.len() -1{
                            self.slide_idx += 1;
                            self.transition = 0.0;
                            if let Some(mut task) = self.job.take() {
                                task.cancel();
                            }
                            let send_msg = self.link.send_back(|_| Msg::Transition(0.1));
                            let handle = self.timeout.spawn(Duration::from_millis(100), send_msg);
                            self.job = Some(Box::new(handle));
                            true
                        } else {
                            false
                        }
                    },
                    _ => {
                        false
                    }
                }
            },
        }
    }
}

impl Renderable<Model> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div style={format!("opacity: {}", self.transition)},
                tabindex="-1",
            onkeydown=|e| Msg::GotKeyPress(e),
            >
            {render_markdown(self.slides[self.slide_idx])}
            </div>
        }
    }
}
