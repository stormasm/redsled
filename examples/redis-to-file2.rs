// Note on vector sorting...
// https://rust-lang-nursery.github.io/rust-cookbook/algorithms/sorting.html

// https://doc.rust-lang.org/std/io/struct.LineWriter.html

use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};

use std::fs::{self, File};
use std::io::prelude::*;
// use std::io::BufWriter;
use std::io::{BufRead, BufReader, Error, Write};
/*
fn write_line_to_file() -> std::io::Result<()> {


    let mut file = LineWriter::new(file);

    file.write_all(b"rick")?;
    file.write_all(b"\n")?;
    Ok(())
}
*/

fn get_hashmap_keys(key: String) -> RedisResult<Vec<u32>> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();
    con.hkeys(key)
}

fn main() -> Result<(), Error> {
    // fn main() {
    let mut keys = get_hashmap_keys("hn-story-19".to_string()).unwrap();
    keys.sort();

    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();

    //let file = File::create("poem.txt");
    //let mut writer = BufWriter::new(&file);

    let path = "lines.txt";

    let mut output = File::create(path)?;
    // write!(output, "Rust\n💖\nFun")?;

    for key in &keys {
        let value: RedisResult<String> = con.hget("hn-story-19".to_string(), key.to_string());
        let json = value.unwrap();
        println!("{}", key);
        println!("{}", json);

        write!(output, "{}", json.to_string())?;

        // write!(output, json.to_string())?;
        write!(output, "{}", "\n")?;

        // writer.write_all(b"test\n");
        //writer.write_all();
        //writer.write_all("\n");

        // write_line_to_file();
    }

    //writer.flush();

    println!("Number of keys = {}", keys.len());
    Ok(())
}
