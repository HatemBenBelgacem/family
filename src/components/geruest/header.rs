use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    rsx!{
        nav { class: "navbar bg-body-tertiary",
            div { class: "container-fluid pb-2 pt-2",
                h2 { "" }
                a {
                    Link { to: "/login",
                        i { class: "bi bi-box-arrow-in-left" }
                    }
                }
            
            }
        
        }
    }
}