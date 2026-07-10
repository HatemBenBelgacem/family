use dioxus::prelude::*;
use crate::backend::models::produkt::Produkt;




#[server]
pub async fn fetch_produkt(id: String) -> Result<Produkt, ServerFnError> {
    let produkt = crate::backend::repository::produkt_repo::get_produkt_by_id(&id)
        .await
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("Datenbankfehler: {}", e)) })?;
        
    Ok(produkt)
}

#[server]
pub async fn alle_produkte() -> Result<Vec<Produkt>, ServerFnError> {
    let produktliste = crate::backend::repository::produkt_repo::produktliste()
        .await
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("Datenbankfehler: {}", e)) })?;
    
    Ok(produktliste)
}


#[server]
pub async fn create_produkt(bezeichnung: String, eingekauft: bool) -> Result<(), ServerFnError> {
    if bezeichnung.is_empty() {
        return Err(ServerFnError::ServerError("Benutzername und Passwort dürfen nicht leer sein".into()));
    }
    
    let produkt = Produkt {
        id: uuid::Uuid::new_v4(),
        bezeichnung,
        eingekauft,
    };

    crate::backend::repository::produkt_repo::create_produkt(&produkt)
        .await
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("Datenbankfehler: {}", e)) })?;

    Ok(())
}

#[server]
pub async fn delete_produkt(id: String) -> Result<(), ServerFnError> {
    crate::backend::repository::produkt_repo::delete_produkt(&id)
        .await
        .map_err(|e| -> ServerFnError { ServerFnError::ServerError(format!("Datenbankfehler: {}", e)) })?;

    Ok(())
}