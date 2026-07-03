use dioxus::prelude::*;

mod backend;
mod components;

use components::home::home::Home;
use components::home::login::Login;
use components::home::register::Register;
use components::produkt::einkaufsliste::Einkaufsliste;


const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home{},
    #[route("/register")]
    Register{},
    #[route("/login")]
    Login{},
    #[route("/einkauf")]
    Einkaufsliste{},
}

