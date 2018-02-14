use std::fs::File;
use std::error::Error;
use std::fs;
use std::env;
use std::io::{BufReader, Read};
use std::str;
use std::io::stdout;
use List::*;
use std::io::Write;

const BUFFER_SIZE: usize = 2048;

enum List {
    Cons(u32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List{
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 + tail.len(),

            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}

fn main() {
    let paths: Vec<String> = env::args().skip(1).collect();
    let mut text = String::new();

    std::io::stdout().write_all("\x1b[2J\x1b[1;1H".as_bytes()).unwrap();
    // clear
    // std::io::stdout().write_all("\x1b[31m\x1b[47m".as_bytes()).unwrap();

    if paths.is_empty() {
        panic!("file name not given");
    }
    // pathが空ならねーよってキレる

    let mut writer = stdout();
    for path in paths {
        file_read(&mut text, &path);
    }
    // pathの数だけdo_cat
}

fn file_read(text: &mut String, path: &str) {
    let mut f = File::open(path).unwrap();

    match f.read_to_string(text){
        Err(why) => panic!("couldn't read {}: {}", path,
                                                   Error::description(&why)),
        Ok(_) => println!("FileReadOK!")
    };
    println!("{}", text);

    
//    let file = File::open(path).unwrap();
//    // 流れてきたPathをOpen
//    let mut reader = BufReader::new(&file);
//    // リーダーを作成
//    let mut buf = [0; BUFFER_SIZE];
//    loop {
//        let n = reader.read(&mut buf).unwrap();
//        if n == 0 { break; }
//        writer.write_all(&buf[..n]).unwrap();
//    }
    // リーダーから受け取った内容を表示
}
