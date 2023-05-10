use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt::Display;
use std::fmt::{Debug, Formatter};
use std::env;

#[repr(C)]
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

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct MValueStruct {
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
struct MessageStruct {
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
    fn new(fields: Vec<&str>) -> Self {
        let type_ = fields[0].parse().unwrap();
        match type_ {
            1 => {
                let tmp_type = fields[1].parse().unwrap();
                let tmp_val = fields[2].parse().unwrap();
                let tmp_timestamp = fields[3].parse().unwrap();
                CData{ type_: type_, data: ExportData{ val: ValueStruct { type_: tmp_type, val: tmp_val, timestamp: tmp_timestamp }}}
            },
            2 => {
                let tmp_type = fields[1].parse().unwrap();
                let mut tmp_val = [0.0f32; 10];
                for i in 0..10 {
                    tmp_val[i] = fields[2+i].parse().unwrap();
                }
                let tmp_timestamp = fields[12].parse().unwrap();
                CData{ type_: type_, data: ExportData{ mvals: MValueStruct { type_: tmp_type, val: tmp_val, timestamp: tmp_timestamp }}}
            },
            3 => {
                let tmp_type = fields[1].parse().unwrap();
                let mut tmp_message = [0u8; 21];
                let tmp_str = fields[2..].join(" ");
                let bytes = tmp_str.as_bytes();
                for i in 0..std::cmp::min(21, bytes.len()) {
                    tmp_message[i] = bytes[i];
                }
                CData{ type_: type_, data: ExportData{ messages: MessageStruct { message_type: tmp_type, message: tmp_message}}}
            },
            _ => CData{ type_: type_, data: ExportData{ val: ValueStruct { type_: type_, val: 0.0, timestamp: 0 }}}
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

    let file = File::open(file_name).expect("Impossibile aprire il file");
    let reader = BufReader::new(file);
    let mut values = Vec::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let fields: Vec<&str> = line.split(' ').collect();
        let value = CData::new(fields);
        values.push(value);
    }

    for i in 0..100 {
        println!("{}", values[i]);
    }
}
