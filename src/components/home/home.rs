use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Home() -> Element {
    rsx! {
        div {
            h1 { "Willkommen zur Family App!" }
            p { "Bitte wähle eine Option:" }

            ul {
                li {
                    Link { to: Route::Login {}, "Zum Login" }
                }
                li {
                    Link { to: Route::Register {}, "Zur Registrierung" }
                }
            }
        }
    }
}