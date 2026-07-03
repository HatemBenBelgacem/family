use serde::{Deserialize, Serialize};

#[cfg(feature = "server")]
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "server", derive(FromRow))]
pub struct Produkt {
    pub id: uuid::Uuid,
    pub bezeichnung: String,
    pub eingekauft: bool,
}