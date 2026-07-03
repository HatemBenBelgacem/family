use dioxus::prelude::*;
use crate::backend::server_functions::benutzer_fns::register_benutzer;
use crate::Route;

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
                Ok(_) => {
                    error_msg.set(format!("Willkommen, {}!", username()));
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
                class: "border rounded p-4 shadow-sm bg-white",
                style: "width: 100%; max-width: 400px;",
                div {
                    h2 { "Registrieren" }

                    if !error_msg().is_empty() {
                        p { color: "red", "{error_msg}" }
                    }

                    div { class: "mb-3",
                        input {
                            class: "form-control",
                            r#type: "text",
                            placeholder: "Benutzername",
                            value: "{username}",
                            oninput: move |e| username.set(e.value()),
                        }
                    }

                    div { class: "mb-3",
                        input {
                            class: "form-control",
                            r#type: "email",
                            placeholder: "E-Mail Adresse",
                            value: "{email}",
                            oninput: move |e| email.set(e.value()),
                        }
                    }
                    div { class: "mb-3",
                        input {
                            class: "form-control",
                            r#type: "password",
                            placeholder: "Passwort",
                            value: "{password}",
                            oninput: move |e| password.set(e.value()),
                        }
                    }
                    div { class: "mb-3",
                        button {
                            class: "btn btn-outline-secondary",
                            onclick: register_action,
                            "Registrieren"
                        }
                    }
                    div { class: "mb-3",
                        li {
                            Link { to: Route::Login {}, "Login" }
                        }
                    }
                
                }
            }
        }

    }
}