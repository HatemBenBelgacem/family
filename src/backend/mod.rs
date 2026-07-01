pub mod models;
pub mod server_fns;

// Das Repository wird Client-seitig (WASM) gar nicht erst kompiliert, 
// da es sqlx enthält, was im Browser nicht funktioniert.
#[cfg(feature = "server")]
pub mod repository;