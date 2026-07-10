use dioxus::prelude::*;
use crate::components::geruest::header::Header;
use crate::components::produkt::create::Create;
use crate::components::produkt::einkaufsliste::Einkaufsliste;

#[component]
pub fn AppLayout() -> Element {
    use_context_provider(|| Signal::new(0usize));
    rsx!{
        div { class: "container-fluid p-0",
            div { class: "", Header {} }
        
        }
        div { class: "container p-0",
            div { class: "", Create {} }
            div { class: "", Einkaufsliste {} }
        }
    }
}