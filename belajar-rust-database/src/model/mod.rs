use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, sqlx::FromRow)]
pub struct Brand {
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
