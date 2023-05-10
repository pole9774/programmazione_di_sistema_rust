use std::fs::File;
use std::io::Read;
use std::fmt::Display;
use std::fmt::Formatter;
use std::convert::TryFrom;

#[derive(Debug, Clone, Copy)]
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
        write!(f, "({}, {}, {})", self.type_, self.val, self.timestamp)
    }
}

fn main() {
    // Apriamo il file in lettura
    let mut file = File::open("src/data_sem.bin").expect("Impossibile aprire il file");

    // Leggiamo il contenuto del file in un vettore di byte
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    // Calcoliamo il numero di struct presenti nel file
    let num_structs = buffer.len() / std::mem::size_of::<ValueStruct>();

    println!("Num struct: {}", num_structs);

    // Convertiamo il vettore di byte in un vettore di struct
    let mut values = Vec::new();
    for i in 0..num_structs {
        let start_type = i * std::mem::size_of::<ValueStruct>();

        //let tmp_type = i32::from_le_bytes([buffer[start_type], buffer[start_type+1], buffer[start_type+2], buffer[start_type+3]]);
        let tmp_type = i32::from_le_bytes(TryFrom::try_from(buffer.get(start_type..=start_type+3).expect("Invalid Range")).unwrap());
        //let tmp_val = f32::from_le_bytes([buffer[start_type+4], buffer[start_type+5], buffer[start_type+6], buffer[start_type+7]]);
        let tmp_val = f32::from_le_bytes(TryFrom::try_from(buffer.get(start_type+4..=start_type+7).expect("Invalid Range")).unwrap());
        /*let tmp_timestamp = i64::from_le_bytes([buffer[start_type+8], buffer[start_type+9], buffer[start_type+10], buffer[start_type+11],
                                                buffer[start_type+12], buffer[start_type+13], buffer[start_type+14], buffer[start_type+15]]);*/
        let tmp_timestamp = i64::from_le_bytes(TryFrom::try_from(buffer.get(start_type+8..=start_type+15).expect("Invalid Range")).unwrap());

        let value = ValueStruct::new(tmp_type, tmp_val, tmp_timestamp);

        values.push(value);
    }

    // Stampiamo il contenuto del vettore a video
    for value in values.iter() {
        println!("{}", value);
    }
}
