// Note on vector sorting...
// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html

use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};

use std::fs::File;
use std::io::{Error, Write};

fn get_set_keys(key: String) -> RedisResult<Vec<u32>> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.smembers(key)
}

fn main() -> Result<(), Error> {
    let mut keys = get_set_keys("set-hnfav".to_string()).unwrap();
    keys.sort();
    /*
        let manager = RedisConnectionManager::new("redis://localhost").unwrap();
        let pool = r2d2::Pool::builder().build(manager).unwrap();

        let pool = pool.clone();
        let mut con = pool.get().unwrap();
    */
    let path = "lines.json";
    let mut output = File::create(path)?;

    let mut ids = vec![];

    for key in &keys {
        ids.push(key.to_string());
    }
    write!(output, "{:?}", ids);
    output.sync_all()?;
    println!("Number of keys = {}", keys.len());
    Ok(())
}
