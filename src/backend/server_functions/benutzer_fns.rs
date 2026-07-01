// src/backend/server_fns/benutzer_fns.rs

use dioxus::prelude::*;
use crate::backend::models::benutzer::Benutzer;

#[server]
pub async fn register_benutzer(benutzer: Benutzer) -> Result<(), ServerFnError> {
    // Serverseitige Validierung könnte hier stattfinden
    if benutzer.benutzername.is_empty() {
        return Err(ServerFnError::ServerError("Benutzername darf nicht leer sein".into()));
    }

    // Aufruf der reinen DB-Logik
    crate::backend::repository::benutzer_repo::create_benutzer(&benutzer)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Datenbankfehler: {}", e)))?;

    Ok(())
}

#[server]
pub async fn fetch_benutzer(id: String) -> Result<Benutzer, ServerFnError> {
    let user = crate::backend::repository::benutzer_repo::get_benutzer_by_id(&id)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Benutzer nicht gefunden: {}", e)))?;
        
    Ok(user)
}