use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx!{
        nav { class: "navbar bg-body-tertiary",
            div { class: "container-fluid",
                h2 { "family-app" }
                a {
                    Link { to: "/login",
                        i { class: "bi bi-box-arrow-in-left" }
                    }
                }
            
            }
        
        }
    }
}