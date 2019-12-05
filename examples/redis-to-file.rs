use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};

fn get_hashmap_keys(key: String) -> RedisResult<Vec<String>> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.hkeys(key)
}

fn main() {
    let keys = get_hashmap_keys("hn-story-19".to_string()).unwrap();

    for key in &keys {
        println!("{}", key);
    }
}
