use dioxus::prelude::*;
use crate::Route; // Wir importieren die Routen aus der main.rs

#[component]
pub fn Sideboard() -> Element {
    rsx! {
        div {
            class: "d-flex flex-column flex-shrink-0 p-3 bg-body-tertiary",
            style: "width: 280px; min-height: 100vh;",
            a {
                href: "/",
                class: "d-flex align-items-center mb-3 mb-md-0 me-md-auto link-body-emphasis text-decoration-none",
                i { class: "bi bi-bootstrap-fill fs-4 me-2" }
                span { class: "fs-4", "Family App" } // Name deiner App
            }
            hr {}
            // --- 2. Die Navigation (Menüpunkte) ---
            ul { class: "nav nav-pills flex-column mb-auto",
                // Zweiter Menüpunkt: Einkaufsliste
                li { class: "nav-item",
                    Link {
                        to: Route::Einkaufsliste {}, // Verknüpfung zur Einkaufsliste
                        class: "nav-link link-body-emphasis",
                        i { class: "bi bi-cart me-2" } // Einkaufswagen-Icon
                        "Einkaufsliste"
                    }
                }
            }

            hr {} // Eine weitere Trennlinie vor dem Benutzerprofil

            // --- 3. Der untere Bereich (Benutzerprofil / Dropdown) ---
            div { class: "dropdown",
                a {
                    href: "#",
                    class: "d-flex align-items-center link-body-emphasis text-decoration-none dropdown-toggle",
                    "data-bs-toggle": "dropdown", // Macht den Link aufklappbar
                    "aria-expanded": "false",
                    i { class: "bi bi-person-circle fs-4 me-2" } // Benutzer-Icon
                    strong { "Benutzer Menü" }
                }
                // Die Liste, die sich beim Klicken öffnet
                ul { class: "dropdown-menu text-small shadow",
                    li {
                        Link { to: Route::Login {}, class: "dropdown-item", "Login" }
                    }
                    li {
                        Link { to: Route::Register {}, class: "dropdown-item", "Registrieren" }
                    }
                }
            }
        }
    }
}