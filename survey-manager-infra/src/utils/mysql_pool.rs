use r2d2_mysql::MysqlConnectionManager;
use mysql::{OptsBuilder, Opts};

/// Pool type is a simple wrapper over r2d2::Pool<MysqlConnectionManager> -> use it to pass around your
/// pool.
pub type Pool = r2d2::Pool<MysqlConnectionManager>;

pub fn create_pool(db_url: &str) -> Pool {
    let opts = Opts::from_url(&db_url).unwrap();
    let builder = OptsBuilder::from_opts(opts);
    let manager = MysqlConnectionManager::new(builder);
    r2d2::Pool::builder().max_size(32).build(manager).unwrap()
}
