#[component]
fn Home() -> Element {
    rsx! {
        div {
            h1 { "Willkommen zur Family App!" }
            p { "Bitte wähle eine Option:" }

            // Navigation mit der Link-Komponente
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