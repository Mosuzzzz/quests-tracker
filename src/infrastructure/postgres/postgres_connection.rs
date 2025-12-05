use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, Pool},
};
use anyhow::Result;

pub type PgpoolSquad = Pool<ConnectionManager<PgConnection>>;

pub fn estabish_connection_pool(database_url: &str) -> Result<PgpoolSquad> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = Pool::builder().build(manager)?;
    Ok(pool)
}