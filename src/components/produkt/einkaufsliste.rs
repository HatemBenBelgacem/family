use dioxus::prelude::*;
use crate::backend::server_functions::produkt_fns::alle_produkte;
use crate::components::produkt::delete::Delete;
// use crate::Route; // Nur importieren, wenn es genutzt wird

#[component]
pub fn Einkaufsliste() -> Element {
    // Die Resource startet den asynchronen Aufruf
    let mut reload_trigger = consume_context::<Signal<usize>>();
    
    let produktliste = use_resource(move || async move { 
        let _ = reload_trigger.read();
        alle_produkte().await.unwrap_or_default()
        
    });
    
    rsx! {
        {
            match &*produktliste.read() {
                Some(produkte) => {
                    rsx! {
                        for produkt in produkte {
                            div { class: "card", key: "{produkt.id}",
                                div { class: "card-body",
                                    input { class: "form-check-input me-3", r#type: "checkbox" }
                                    "{produkt.bezeichnung}"
                                    Delete { produkt: produktliste, id: produkt.id.clone() }

                                }

                            }
                        }
                    }
                }
                None => rsx! {
                    p { "Daten laden" }
                },
            }
        }
    }
}