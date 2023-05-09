use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::Display;
use std::fmt::Formatter;

struct ValueStruct {
    type_: i32,
    val: f32,
    timestamp: i64,
}

impl ValueStruct {
    fn new(type_: i32, val: f32, timestamp: i64) -> Self {
        ValueStruct{type_, val, timestamp}
    }
}

impl Display for ValueStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(type: {}, val: {}, timestamp: {})", self.type_, self.val, self.timestamp)
    }
}

fn main() {
    // Apriamo il file in lettura
    let file = File::open("src/valori.txt").expect("Impossibile aprire il file");

    // Creiamo un buffer per leggere il file riga per riga
    let reader = BufReader::new(file);

    // Creiamo un vettore per memorizzare i dati letti dal file
    let mut values = Vec::new();

    // Leggiamo il file riga per riga e memorizziamo i dati nel vettore
    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split(' ').collect();
        let type_ = fields[0].parse().unwrap();
        let val = fields[1].parse().unwrap();
        let timestamp = fields[2].parse().unwrap();
        let value = ValueStruct::new(type_, val, timestamp);
        values.push(value);
    }

    // Stampiamo il contenuto del vettore a video
    for value in values.iter() {
        println!("{}", value);
    }
}
