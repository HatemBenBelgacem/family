use dioxus::prelude::*;
use crate::backend::server_functions::produkt_fns::delete_produkt;
use crate::backend::models::produkt::Produkt;

#[component]
pub fn Delete(mut produkt: Resource<Vec<Produkt>>, id: String) -> Element {
    rsx!{
        // 1. Wir nutzen einen 'button' statt 'Link'
        button {
            onclick: move |_| {

                let id = id.clone();

                spawn(async move {
                    match delete_produkt(id).await {
                        Ok(_) => {
                            produkt.restart();
                        }
                        Err(e) => {
                            // Fehlerbehandlung
                            println!("Fehler beim Löschen: {:?}", e);
                        }
                    }
                }); // <-- Wichtig: spawn() schließen
            },
            i { class: "bi bi-trash3" }
        }
    }
}