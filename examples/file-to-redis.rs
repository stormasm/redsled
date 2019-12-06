use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::io;
use std::io::BufRead;
use std::io::BufWriter;
use std::io::Write;

/// use num::Integer::is_even;
// use num::num_integer::*;
use r2d2_redis::{r2d2, RedisConnectionManager};
// use redis::Commands;
use redis::{Commands, RedisResult};

#[allow(dead_code)]
fn write_json_to_redis(key: String, value: String) -> RedisResult<()> {
    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::builder().build(manager).unwrap();

    let pool = pool.clone();
    let mut con = pool.get().unwrap();

    let _x0 = redis::cmd("HSET")
        .arg("hn-story-19-bak")
        .arg(key)
        .arg(value)
        .query::<u64>(&mut *con)
        .unwrap();

    Ok(())
}

fn read_file_to_buffer2(filename: String) {
    let f = File::open(filename).unwrap();
    let file = BufReader::new(&f);

    let mut writer = BufWriter::new(io::stdout());
    for (num, line) in file.lines().enumerate() {
        // num.is_even();
        // 4.is_even();

        /*
                if num::Integer.is_even(num) {
                    writeln!(writer, "{0}\n", num).unwrap();
                }
        */
        let l = line.unwrap();
        writeln!(writer, "{0} {1}\n", num, l).unwrap();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a filename");
        process::exit(1);
    }
    let filename = &args[1];
    println!("In file {}", filename);

    let _contents = read_file_to_buffer2(filename.to_string());

    // let _ = write_json_to_redis(item_id.to_string(), item_json);
}
