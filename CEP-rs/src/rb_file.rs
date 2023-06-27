use roaring::RoaringBitmap;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::time::Instant;

pub fn create_rb_bytes(fname: &str, debug: bool) -> Vec<u8> {
    let start = match debug {
        true => Some(Instant::now()),
        false => None,
    };

    let mut rb = RoaringBitmap::new();

    let file = File::open(&fname).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut result = reader.read_line(&mut line).unwrap();

    let mut i = 0;

    while result != 0 {
        let cep_number = &line[..8]
            .parse::<u32>()
            .expect(format!("Erro ao converter o CEP {}", line).as_str());
        rb.insert(*cep_number);
        line.clear();
        result = reader.read_line(&mut line).unwrap();
        i += 1;
    }

    match start {
        Some(start) => {
            dbg!("Saving {} size", rb.serialized_size());
            let duration = start.elapsed();
            dbg!("{} lines read in {} seconds", i, duration);
        }
        None => (),
    }

    let mut bytes = vec![];
    rb.serialize_into(&mut bytes).unwrap();
    return bytes;
}

pub fn write_bytes(bytes: Vec<u8>, output: &str, debug: bool) {
    let start = match debug {
        true => Some(Instant::now()),
        false => None,
    };

    let mut file = File::create(&output).unwrap();
    file.write_all(&bytes).unwrap();
    file.flush().unwrap();

    match start {
        Some(start) => {
            let duration = start.elapsed();
            dbg!("File saved in {} seconds", duration);
        }
        None => (),
    }
}

pub fn read_bytes(input: &str, debug: bool) -> Vec<u8> {
    let start = match debug {
        true => Some(Instant::now()),
        false => None,
    };

    let mut file = File::open(&input).unwrap();
    let mut bytes = vec![];
    file.read_to_end(&mut bytes).unwrap();

    match start {
        Some(start) => {
            let duration = start.elapsed();
            dbg!("File read in {} seconds", duration);
        }
        None => (),
    };
    return bytes;
}

pub fn create_rb_from_bytes(bytes: Vec<u8>, debug: bool) -> RoaringBitmap {
    let start = match debug {
        true => Some(Instant::now()),
        false => None,
    };

    let rb = RoaringBitmap::deserialize_from(&mut bytes.as_slice()).unwrap();

    match start {
        Some(start) => {
            let duration = start.elapsed();
            dbg!("RB created in {} seconds", duration);
        }
        None => (),
    };

    return rb;
}
