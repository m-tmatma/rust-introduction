use std::fs::File;
use std::io::{Read, Write, BufReader, BufWriter};
use std::env;
// https://stackoverflow.com/questions/63648649/type-stdresultresultu8-stdioerror-cannot-be-dereferenced

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() <= 1 {
        let reader = BufReader::new(std::io::stdin());
        do_print(reader);
    } else {
        let file = File::open(&args[1]).expect("file not found.");
        let reader = BufReader::new(file);
        do_print(reader);
    }
}

fn do_print<R>(reader: BufReader<R>) where R: std::io::Read {
    let mut writer = BufWriter::new(std::io::stdout());
    for it in reader.bytes() {
        writer.write(&[it.unwrap()]);
    }
}
