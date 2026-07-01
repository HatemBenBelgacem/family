#[cfg(feature = "server")]
use crate::backend::models::benutzer::Benutzer;
#[cfg(feature = "server")]
use crate::backend::db::get_db;


#[cfg(feature = "server")]
pub async fn get_benutzer_by_benutzername(benutzername: &str) -> Result<Option<Benutzer>, sqlx::Error> {
    let pool = get_db().await;
    
    let benutzer = sqlx::query_as::<_, Benutzer>(
        "SELECT * FROM benutzer WHERE benutzername = $1::uuid"
    )
    .bind(benutzername)
    .fetch_optional(pool) // fetch_optional, da der Benutzer evtl. nicht existiert
    .await?;

    Ok(benutzer)
}

// Diese Funktion existiert nur auf dem Server
#[cfg(feature = "server")]
pub async fn create_benutzer(benutzer: &Benutzer) -> Result<(), sqlx::Error> {
    let pool = get_db().await;
    
    sqlx::query(
        r#"
        INSERT INTO benutzer (id, benutzername, email, passwort)
        VALUES ($1::uuid, $2, $3, $4)
        "#
    )
    .bind(&benutzer.id)
    .bind(&benutzer.benutzername)
    .bind(&benutzer.email)
    .bind(&benutzer.passwort) // Hinweis: In der Praxis natürlich hashen!
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn get_benutzer_by_id(id: &str) -> Result<Benutzer, sqlx::Error> {
    let pool = get_db().await;
    
    let benutzer = sqlx::query_as::<_, Benutzer>(
        "SELECT * FROM benutzer WHERE id = $1::uuid"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(benutzer)
}