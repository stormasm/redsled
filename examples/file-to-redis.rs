// https://doc.rust-lang.org/rust-by-example/scope/lifetime/methods.html

use std::env;
use std::process;
use std::string::String;

use std::fs::File;
use std::io::BufReader;

use std::convert::TryInto;
use std::io::BufRead;

use r2d2_redis::{r2d2, RedisConnectionManager};
use redis::{Commands, RedisResult};

#[derive(Debug)]
struct FileToVec<'a> {
    key: &'a mut Vec<u32>,
    value: &'a mut Vec<String>,
}

impl<'a> FileToVec<'a> {
    fn is_even(num: u32) -> bool {
        (num) & 1 == 0
    }

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

    fn readfile(&mut self, filename: String) {
        let f = File::open(filename).unwrap();
        let file = BufReader::new(&f);

        for (num, line) in file.lines().enumerate() {
            let xval = line.unwrap().clone();
            if FileToVec::is_even(num.try_into().unwrap()) {
                let xkey = xval.parse::<u32>().unwrap();
                self.key.push(xkey);
            }
            if !FileToVec::is_even(num.try_into().unwrap()) {
                self.value.push(xval);
            }
        }

        for i in 0..self.key.len() {
            println!("{} {}", self.key[i], self.value[i]);
            FileToVec::write_json_to_redis(self.key[i].to_string(), self.value[i]);
        }
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

    // Instantiate a FileToVec
    let mut ftv: FileToVec = FileToVec {
        key: &mut Vec::new(),
        value: &mut Vec::new(),
    };

    let _contents = FileToVec::readfile(&mut ftv, filename.to_string());
}
