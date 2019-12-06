// Note on vector sorting...
// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html

use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};

use std::fs::File;
use std::io::{Error, Write};

fn get_hashmap_keys(key: String) -> RedisResult<Vec<u32>> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.hkeys(key)
}

fn main() -> Result<(), Error> {
    let mut keys = get_hashmap_keys("hn-story-19".to_string()).unwrap();
    keys.sort();

    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();

    let path = "lines.txt";
    let mut output = File::create(path)?;

    for key in &keys {
        let value: RedisResult<String> = con.hget("hn-story-19".to_string(), key.to_string());
        let json = value.unwrap();

        // println!("{}", key);
        // println!("{}", json);

        write!(output, "{}", json.to_string())?;
        write!(output, "{}", "\n")?;
    }
    output.sync_all()?;
    println!("Number of keys = {}", keys.len());
    Ok(())
}
