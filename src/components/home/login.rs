use dioxus::prelude::*;
use crate::backend::server_functions::benutzer_fns::login_benutzer;

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| String::new());

    let login_action = move |_| {
        spawn(async move {
            let result = login_benutzer(username(), password()).await;
            
            match result {
                Ok(benutzer) => {
                    // Login erfolgreich! Hier könntest du den User-State global setzen 
                    // oder via Router zur Startseite navigieren.
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
            button { onclick: login_action, "Einloggen" }
        }
    }
}