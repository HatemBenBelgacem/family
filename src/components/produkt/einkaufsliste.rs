use dioxus::prelude::*;
use crate::backend::server_functions::produkt_fns::alle_produkte;
use crate::components::produkt::delete::Delete;
use crate::backend::models::produkt::Produkt;


#[component]
pub fn Einkaufsliste() -> Element {
    
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
                            ProduktEintrag { key: "{produkt.id}", produkt: produkt.clone(), produktliste }
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

#[component]
fn ProduktEintrag(produkt: Produkt, mut produktliste: Resource<Vec<Produkt>>) -> Element {
    // 1. Lokaler Zustand: Ist die Checkbox abgehakt?
    // Wir starten mit dem Wert aus der Datenbank (produkt.eingekauft)
    let mut ist_abgehakt = use_signal(|| produkt.eingekauft);

    // 2. Wir definieren dynamisch den CSS-Stil
    // Wenn abgehakt, nehmen wir Bootstrap-Klassen für "durchgestrichen" und "grau"
    let text_stil = if ist_abgehakt() {
        "text-decoration-line-through text-muted" 
    } else {
        "" 
    };

    rsx! {
        div { class: "card mb-1",
            // d-flex und justify-content-between schiebt den Text nach links und den Löschen-Button nach rechts!
            div { class: "card-body d-flex align-items-center justify-content-between",

                div {
                    // Die Checkbox
                    input {
                        class: "form-check-input me-3",
                        r#type: "checkbox",
                        checked: "{ist_abgehakt}", // Mit unserem Zustand verbinden

                        onchange: move |_| {
                            ist_abgehakt.set(!ist_abgehakt());

                        },
                    }
                    span { class: "{text_stil}", "{produkt.bezeichnung}" }
                }

                Delete { produkt: produktliste, id: produkt.id.clone() }
            }
        }
    }
}