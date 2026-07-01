// src/backend/server_fns/benutzer_fns.rs

use dioxus::prelude::*;
use crate::backend::models::benutzer::Benutzer;


#[server]
pub async fn fetch_benutzer(id: String) -> Result<Benutzer, ServerFnError> {
    let user = crate::backend::repository::benutzer_repo::get_benutzer_by_id(&id)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Benutzer nicht gefunden: {}", e)))?;
        
    Ok(user)
}


#[server]
pub async fn register_benutzer(mut benutzer: Benutzer) -> Result<(), ServerFnError> {
    if benutzer.benutzername.is_empty() || benutzer.passwort.is_empty() {
        return Err(ServerFnError::ServerError("Benutzername und Passwort dürfen nicht leer sein".into()));
    }

    let hashed_password = bcrypt::hash(&benutzer.passwort, bcrypt::DEFAULT_COST)
        .map_err(|_| ServerFnError::ServerError("Fehler beim Hashen des Passworts".into()))?;    
    
    benutzer.passwort = hashed_password;

    crate::backend::repository::benutzer_repo::create_benutzer(&benutzer)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Datenbankfehler: {}", e)))?;

    Ok(())
}

#[server]
pub async fn login_benutzer(benutzername: String, passwort_klartext: String) -> Result<Benutzer, ServerFnError> {
    // 1. Benutzer aus der DB holen
    let benutzer_opt = crate::backend::repository::benutzer_repo::get_benutzer_by_benutzername(&benutzername)
        .await
        .map_err(|e| ServerFnError::ServerError(format!("Datenbankfehler: {}", e)))?;

    if let Some(benutzer) = benutzer_opt {
        // 2. Passwort überprüfen
        let is_valid = bcrypt::verify(&passwort_klartext, &benutzer.passwort)
            .unwrap_or(false);

        if is_valid {
            // Login erfolgreich! (In einer echten App würdest du hier z.B. einen Session-Cookie setzen)
            Ok(benutzer)
        } else {
            Err(ServerFnError::ServerError("Falsches Passwort".into()))
        }
    } else {
        Err(ServerFnError::ServerError("Benutzer nicht gefunden".into()))
    }
}