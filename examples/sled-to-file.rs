// https://doc.rust-lang.org/rust-by-example/scope/lifetime/methods.html

use std::env;
use std::process;
use std::str::from_utf8;
use std::string::String;

use std::fs::File;
use std::io::{Error, Write};

use sled::Db;

#[derive(Debug)]
struct SledToVec<'a> {
    key: &'a mut Vec<String>,
    value: &'a mut Vec<String>,
}

impl<'a> SledToVec<'a> {
    fn read_sled_write_file(&mut self, dbname: String) -> Result<(), Error> {
        let tree = Db::open(dbname).unwrap();
        let keys = tree.iter().keys();

        for key in keys {
            let ckey = key.clone().unwrap();
            let xkey = from_utf8(&ckey).unwrap();
            self.key.push(xkey.to_string());
        }

        let path = "linesmin-sled.txt";
        let mut output = File::create(path)?;

        for i in 0..self.key.len() {
            let x = &self.key[i];
            // println!("{}", x);
            write!(output, "{}", x)?;
            write!(output, "{}", "\n")?;

            let y = tree.get(x).unwrap().unwrap();
            // println!("{:?}", from_utf8(&y).unwrap());
            write!(output, "{}", from_utf8(&y).unwrap())?;
            write!(output, "{}", "\n")?;
        }

        output.sync_all()?;
        tree.flush().unwrap();
        Ok(())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("You need to enter a db name");
        process::exit(1);
    }
    let dbname = &args[1];
    println!("In file {}", dbname);

    // Instantiate a SledToVec
    let mut ftv: SledToVec = SledToVec {
        key: &mut Vec::new(),
        value: &mut Vec::new(),
    };

    let _contents = SledToVec::read_sled_write_file(&mut ftv, dbname.to_string());
}
