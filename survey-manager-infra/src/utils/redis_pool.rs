use mysql::{OptsBuilder, Opts};
use r2d2_redis::RedisConnectionManager;

/// Pool type is a simple wrapper over r2d2::Pool<RedisConnectionManager> -> use it to pass around your
/// pool.
pub type Pool = r2d2::Pool<RedisConnectionManager>;

pub type Conn = r2d2::PooledConnection<RedisConnectionManager>;

pub fn create_pool(db_url: &str) -> Pool {
    let manager = RedisConnectionManager::new(db_url).unwrap();

    r2d2::Pool::builder()
        .build(manager)
        .unwrap()
}
