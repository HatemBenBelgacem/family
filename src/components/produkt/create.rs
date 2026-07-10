use dioxus::prelude::*;
use crate::backend::server_functions::produkt_fns::create_produkt;


#[component]
pub fn Create() -> Element {
    let mut bezeichnung = use_signal(|| String::new());
    let mut eingekauft = use_signal(|| false);
    let mut error_msg = use_signal(|| String::new());
    let mut reload_trigger = consume_context::<Signal<usize>>();

    let register_action = move |_| {
        spawn(async move {
            let result = create_produkt(bezeichnung(), eingekauft()).await;
            
            match result {
                Ok(_) => {
                    error_msg.set(format!("Willkommen, {}!", bezeichnung()));
                    reload_trigger += 1;
                    bezeichnung.set(String::new());
                },
                Err(e) => {
                    error_msg.set(e.to_string());
                }
            }
        });
    };

    rsx! {
        div { class: " bg-light",
            div { class: "bg-white", style: "width: 100%;",
                div {
                    h2 { "Einkaufsliste" }

                    div { class: "mb-3",
                        input {
                            class: "form-control",
                            r#type: "text",
                            placeholder: "Bezeichnung",
                            value: "{bezeichnung}",
                            oninput: move |e| bezeichnung.set(e.value()),
                        }
                    }
                    div { class: "mb-3",
                        button {
                            class: "btn btn-outline-secondary",
                            onclick: register_action,
                            "Speichern"
                        }
                    }
                
                }
            }
        }

    }
}