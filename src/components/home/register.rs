use dioxus::prelude::*;
use crate::backend::server_functions::benutzer_fns::register_benutzer;

#[component]
pub fn Register() -> Element {
    let mut username = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| String::new());

    let register_action = move |_| {
        spawn(async move {
            let result = register_benutzer(username(), email(), password()).await;
            
            match result {
                Ok(benutzer) => {
                    error_msg.set(format!("Willkommen, {}!", benutzer.benutzername));
                },
                Err(e) => {
                    error_msg.set(e.to_string());
                }
            }
        });
    };

    rsx! {
        div {
            h2 { "Login" }

            if !error_msg().is_empty() {
                p { color: "red", "{error_msg}" }
            }

            input {
                r#type: "text",
                placeholder: "Benutzername",
                value: "{username}",
                oninput: move |e| username.set(e.value()),
            }
            input {
                r#type: "password",
                placeholder: "Passwort",
                value: "{password}",
                oninput: move |e| password.set(e.value()),
            }
            button { onclick: register_action, "Einloggen" }
        }
    }
}