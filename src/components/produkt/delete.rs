use dioxus::prelude::*;
use crate::backend::server_functions::produkt_fns::delete_produkt;
use crate::backend::models::produkt::Produkt;

#[component]
pub fn Delete(mut produkt: Resource<Vec<Produkt>>, id: uuid::Uuid) -> Element {
    rsx!{
        a {
            // Lässt es wie einen typischen Link aussehen (Mauszeiger wird zur Hand)
            href: "#",

            // WICHTIG: Verhindert, dass die Seite beim Klicken neu lädt oder nach oben springt!
            prevent_default: "onclick",

            // Ein bisschen Bootstrap-Styling (optional), damit es rot und wie ein Link aussieht
            class: "text-danger text-decoration-none float-end",

            onclick: move |_| {
                let id = id.clone();
                spawn(async move {
                    match delete_produkt(id).await {
                        Ok(_) => {
                            produkt.restart();
                        }
                        Err(e) => {
                            println!("Fehler beim Löschen: {:?}", e);
                        }
                    }
                });
            },

            // Das Mülleimer-Icon bleibt genau wie vorher
            i { class: "bi bi-trash3" }
        
        // Optional: Du könntest hier auch noch Text hinzufügen, z.B.
        // " Löschen"
        }
    }
}