use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub translation: String,
}