use dioxus::prelude::*;
use crate::backend::server_functions::benutzer_fns::login_benutzer;
use crate::Route;

#[component]
pub fn Login() -> Element {
    let mut username = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error_msg = use_signal(|| String::new());
    let nav = use_navigator();

    let login_action = move |_| {
        spawn(async move {
            let result = login_benutzer(username(), password()).await;
            
            match result {
                Ok(benutzer) => {
                    // Login erfolgreich! Hier könntest du den User-State global setzen 
                    // oder via Router zur Startseite navigieren.
                    error_msg.set(format!("Willkommen, {}!", benutzer.benutzername));
                    nav.replace(Route::Einkaufsliste {  });
                },
                Err(e) => {
                    error_msg.set(e.to_string());
                }
            }
        });
    };

    rsx! {
        div { class: "d-flex justify-content-center align-items-center vh-100 bg-light",
            div {
                style: "width: 100%; max-width: 400px;",
                class: "border rounded p-4 shadow-sm bg-white",
                h2 { "Login" }

                if !error_msg().is_empty() {
                    p { color: "red", "{error_msg}" }
                }
                div { class: "mb-3",
                    label { class: "form-label", "Benutzername" }
                    input {
                        class: "form-control",
                        r#type: "text",
                        value: "{username}",
                        oninput: move |e| username.set(e.value()),
                    }
                }
                div { class: "mb-3",
                    label { class: "form-label", "Passwort" }
                    input {
                        class: "form-control",
                        r#type: "password",
                        value: "{password}",
                        oninput: move |e| password.set(e.value()),
                    }
                }
                div { class: "mb-3",
                    button {
                        class: "btn btn-outline-secondary",
                        onclick: login_action,
                        "Einloggen"
                    }
                }
                div { class: "mb-3",
                    li {
                        Link { to: Route::Register {}, "Registrierung" }
                    }
                }
            }
        }
    }
}