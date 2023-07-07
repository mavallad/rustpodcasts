use std::fmt;
use sqlx::PgPool;
use tera::Tera;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub tera: Tera
}

#[derive(Debug, Clone)]
pub struct QueryError {
    pub sql: String,
    pub error: String
}

impl QueryError {
    pub fn new(sql: String, error: String) -> QueryError {
        QueryError { sql, error }
    }
}
impl std::error::Error for QueryError {}

impl fmt::Display for QueryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error executing query {}.\nError: {}", self.sql, self.error)
    }
}
