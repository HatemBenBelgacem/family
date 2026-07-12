use dioxus::prelude::*;
use crate::components::geruest::header::Header;
use crate::components::geruest::sideboard::Sideboard; // 1. Wir importieren das neue Sideboard
use crate::components::produkt::create::Create;

#[component]
pub fn AppLayout() -> Element {
    use_context_provider(|| Signal::new(0usize));
    
    rsx!{
        // Ein großer Container, der die volle Höhe (vh-100) nutzt und flexibel ist
        div { class: "d-flex vh-100",

            // 2. Die linke Spalte: Das Sideboard
            Sideboard {}

            // 3. Die rechte Spalte: Der restliche Platz für den Header und den Seiteninhalt
            div { class: "d-flex flex-column w-100", // w-100 sorgt dafür, dass der restliche Platz voll genutzt wird

                // Der Kopfbereich (Header) ganz oben rechts
                Header {}

                // Der Hauptbereich für den Inhalt
                div { class: "container-fluid p-4",

                    // 4. Das Outlet ist sehr wichtig!
                    // Hier fügt Dioxus automatisch die Seite ein, die im Router aufgerufen wird
                    // (z. B. die Home-Seite oder die Einkaufsliste).
                    Create {}
                    Outlet::<crate::Route> {}
                }
            }
        }
    }
}