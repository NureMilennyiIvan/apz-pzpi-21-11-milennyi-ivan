pub(crate) enum ServiceError {
    DatabaseError(sqlx::Error),
    NotFound,
}