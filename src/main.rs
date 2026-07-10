use dioxus::prelude::*;

mod backend;
mod components;

use components::home::home::Home;
use components::home::login::Login;
use components::home::register::Register;
use components::produkt::einkaufsliste::Einkaufsliste;


const MAIN_CSS: Asset = asset!("/assets/main.css");

#[derive(Clone, Default, PartialEq)]
pub struct BenutzerStatus {
    pub ist_eingeloggt: bool,
    pub benutzername: Option<String>,
}

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    use_context_provider(|| Signal::new(BenutzerStatus::default()));

    rsx! {

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

