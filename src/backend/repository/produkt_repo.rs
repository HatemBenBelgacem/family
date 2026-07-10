#[cfg(feature = "server")]
use crate::backend::models::produkt::Produkt;
#[cfg(feature = "server")]
use crate::backend::db::get_db;


#[cfg(feature = "server")]
pub async fn get_produkt_by_produktname(bezeichnung: &str) -> Result<Option<Produkt>, sqlx::Error> {
    let pool = get_db().await;
    
    let produkt = sqlx::query_as::<_, Produkt>(
        "SELECT * FROM produkt WHERE benutzername = $1"
    )
    .bind(bezeichnung)
    .fetch_optional(pool) // fetch_optional, da der Benutzer evtl. nicht existiert
    .await?;

    Ok(produkt)
}

// Diese Funktion existiert nur auf dem Server
#[cfg(feature = "server")]
pub async fn create_produkt(produkt: &Produkt) -> Result<(), sqlx::Error> {
    let pool = get_db().await;
    
    sqlx::query(
        r#"
        INSERT INTO produkt (id, bezeichnung, eingekauft)
        VALUES ($1, $2, $3)
        "#
    )
    .bind(&produkt.id)
    .bind(&produkt.bezeichnung)
    .bind(&produkt.eingekauft)
    .execute(pool)
    .await?;

    Ok(())
}

#[cfg(feature = "server")]
pub async fn delete_produkt(id:&str) -> Result<Produkt, sqlx::Error> {
    let pool = get_db().await;

    let produkt = sqlx::query_as::<_, Produkt>(
        "DELETE FROM produkt WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(produkt)
}

#[cfg(feature = "server")]
pub async fn get_produkt_by_id(id: &str) -> Result<Produkt, sqlx::Error> {
    let pool = get_db().await;
    
    let produkt = sqlx::query_as::<_, Produkt>(
        "SELECT * FROM produkt WHERE id = $1"
    )
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(produkt)
}

#[cfg(feature = "server")]
pub async fn produktliste() -> Result<Vec<Produkt>, sqlx::Error>{
    let pool = get_db().await;

    let produktliste = sqlx::query_as::<_, Produkt>(
        "SELECT * FROM produkt"
    )
    .fetch_all(pool)
    .await?;

    Ok(produktliste)
}