use std::fs::File;
use std::io::Read;
use std::fmt::Display;
use std::fmt::{Debug, Formatter};
use std::env;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ValueStruct { // 16
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct MValueStruct { // 52
    type_: i32,
    val: [f32; 10],
    timestamp: i64,
}

impl MValueStruct {
    fn new(type_: i32, val: [f32; 10], timestamp: i64) -> Self {
        MValueStruct{type_, val, timestamp}
    }
}

impl Display for MValueStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:?}, {})", self.type_, self.val, self.timestamp)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct MessageStruct { // 25
    message_type: i32,
    message: [u8; 21],
}

impl MessageStruct {
    fn new(message_type: i32, message: &str) -> Self {
        let mut message_bytes = [0u8; 21];
        let message_len = message.len().min(20);
        let message_slice = message.as_bytes().get(..message_len).unwrap_or(&[]);
        message_bytes[..message_slice.len()].copy_from_slice(message_slice);

        MessageStruct {message_type, message: message_bytes}
    }
}

impl Display for MessageStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut length = self.message.len();
        while length > 0 && self.message[length - 1] == 0 {
            length -= 1;
        }
        write!(f, "({}, {:?})", self.message_type, std::str::from_utf8(&self.message[..length]).unwrap())
    }
}

#[repr(C)]
union ExportData{
    val: ValueStruct,
    mvals: MValueStruct,
    messages: MessageStruct
}

#[repr(C)]
struct CData{
    type_: i32,
    data: ExportData
}

impl CData{
    fn new(data_type: i32, buf: Vec<u8>) -> Self {
        match data_type {
            1 => {
                let tmp_type = i32::from_le_bytes(TryFrom::try_from(buf.get(0..=3).expect("Invalid Range")).unwrap());
                let tmp_val = f32::from_le_bytes(TryFrom::try_from(buf.get(4..=7).expect("Invalid Range")).unwrap());
                let tmp_timestamp = i64::from_le_bytes(TryFrom::try_from(buf.get(8..=15).expect("Invalid Range")).unwrap());
                CData { type_: data_type, data: ExportData { val: ValueStruct { type_: tmp_type, val: tmp_val, timestamp: tmp_timestamp }}}
            },
            2 => {
                let tmp_type = i32::from_le_bytes(TryFrom::try_from(buf.get(0..=3).expect("Invalid Range")).unwrap());
                let mut tmp_val = [0.0f32; 10];
                for i in 0..10 {
                    tmp_val[i] = f32::from_le_bytes(TryFrom::try_from(buf.get(4+(i*4)..=7+(i*4)).expect("Invalid Range")).unwrap());
                }
                let tmp_timestamp = i64::from_le_bytes(TryFrom::try_from(buf.get(44..=51).expect("Invalid Range")).unwrap());
                CData { type_: data_type, data: ExportData { mvals: MValueStruct { type_: tmp_type, val: tmp_val, timestamp: tmp_timestamp }}}
            },
            3 => {
                let tmp_type = i32::from_le_bytes(TryFrom::try_from(buf.get(0..=3).expect("Invalid Range")).unwrap());
                let mut tmp_message = [0u8; 21];
                for i in 0..21 {
                    tmp_message[i] = buf[4+i];
                }
                CData { type_: data_type, data: ExportData { messages: MessageStruct { message_type: tmp_type, message: tmp_message }}}
            },
            _ => CData { type_: data_type, data: ExportData { val: ValueStruct { type_: data_type, val: 0.0, timestamp: 0 }}}
        }
    }
}

impl Display for CData {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.type_ {
            1 => unsafe{write!(f, "({}, {})", self.type_, self.data.val)},
            2 => unsafe{write!(f, "({}, {})", self.type_, self.data.mvals)},
            3 => unsafe{write!(f, "({}, {})", self.type_, self.data.messages)},
            _ => unsafe{write!(f, "({}, {})", self.type_, self.data.val)},
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Devi fornire una stringa come argomento!");
        std::process::exit(1);
    }
    let file_name = &args[1];

    // Apriamo il file in lettura
    let mut file = File::open(file_name).expect("Impossibile aprire il file");

    // Leggiamo il contenuto del file in un vettore di byte
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).unwrap();

    let mut values = Vec::new();

    let mut start = 0;
    
    for _i in 0..100 {
        // Tipo del dato corrente
        let data_type = i32::from_le_bytes(TryFrom::try_from(buffer.get(start..=start+3).expect("Invalid Range")).unwrap());

        match data_type {
            1 => {
                let tmp_buf = buffer.iter().skip(start+4).take(16).cloned().collect::<Vec<u8>>();
                let value = CData::new(data_type, tmp_buf);
                values.push(value);
            },
            2 => {
                let tmp_buf = buffer.iter().skip(start+4).take(52).cloned().collect::<Vec<u8>>();
                let value = CData::new(data_type, tmp_buf);
                values.push(value);
            },
            3 => {
                let tmp_buf = buffer.iter().skip(start+4).take(25).cloned().collect::<Vec<u8>>();
                let value = CData::new(data_type, tmp_buf);
                values.push(value);
            },
            _ => {}
        }

        // Prossimo valore di start
        match data_type {
            1 => start = start + 4 + 16,
            2 => start = start + 4 + 52,
            3 => start = start + 4 + 25,
            _ => {}
        }
    }

    for i in 0..100 {
        println!("{}", values[i]);
    }
}
