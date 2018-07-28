extern crate rust_vs_go_slides;
extern crate yew;

use yew::prelude::*;
use rust_vs_go_slides::Model;

fn main() {
    yew::initialize();
    let app: App<Model> = App::new();
    app.mount_to_body();
    yew::run_loop();
}
