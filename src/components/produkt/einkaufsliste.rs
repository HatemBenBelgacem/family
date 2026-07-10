use dioxus::prelude::*;
use crate::components::produkt::create::Create;
use crate::backend::server_functions::produkt_fns::alle_produkte;
// use crate::Route; // Nur importieren, wenn es genutzt wird

#[component]
pub fn Einkaufsliste() -> Element {
    // Die Resource startet den asynchronen Aufruf
    let produktliste = use_resource(move || async move { alle_produkte().await });
    
    rsx! {
        {
            match produktliste() {
                // Zustand 1: Die Daten laden noch
                None => rsx! {
                    div { "Produkte werden geladen..." }
                },

                // Zustand 2: Die Serverfunktion meldet einen Fehler
                Some(Err(fehler)) => rsx! {
                    div { style: "color: red;", "Fehler beim Laden: {fehler}" }
                },

                // Zustand 3: Erfolgreich! 'produkte' ist jetzt deine Liste (z.B. Vec<Produkt>)
                Some(Ok(produkte)) => rsx! {
                    for p in produkte {
                        div { class: "card", key: "{p.id}",
                            div { class: "card-body",
                                "{p.bezeichnung}"
                                input {
                                    class: "form-check-input float-end",
                                    r#type: "checkbox",
                                    value: "true",
                                    "{p.eingekauft}"
                                }

                            }

                        }
                    }
                },
            }
        }
    }
}