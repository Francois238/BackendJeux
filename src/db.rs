use crate::api_error::ApiError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use std::env;



pub fn connection() -> Result<PgConnection , ApiError> {
   

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    Ok(PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url)))
}