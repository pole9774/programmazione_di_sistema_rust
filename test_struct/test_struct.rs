use std::fmt::Display;
use std::fmt::Formatter;

struct ValueStruct {
    typ: i16,
    val: f64,
    timestamp: u64,
}

impl ValueStruct {
    fn new(typ: i16, val: f64, timestamp: u64) -> Self {
        let v = ValueStruct{typ, val, timestamp};
        println!("Creating ValueStruct at address {:p}", &v);
        v
    }
}

impl Display for ValueStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.typ, self.val, self.timestamp)
    }
}

struct MValueStruct {
    typ: i16,
    val: [f64; 10],
    timestamp: u64,
}

impl MValueStruct {
    fn new(typ: i16, val: [f64; 10], timestamp: u64) -> Self {
        let v = MValueStruct{typ, val, timestamp};
        println!("Creating ValueStruct at address {:p}", &v);
        v
    }
}

impl Display for MValueStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:?}, {})", self.typ, self.val, self.timestamp)
    }
}

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

        let m = MessageStruct {message_type, message: message_bytes};
        m
    }
}

impl Display for MessageStruct {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {:?})", self.message_type, self.message)
    }
}

fn main() {
    let v = ValueStruct::new(2, 3.1415, 3333);
    let mv = MValueStruct::new(5, 
        [0.69498086, 0.52732396, 0.8828666, 0.1401573, 0.7995329, 0.8455509, 0.031405915, 0.67017484, 0.2536498, 0.64809763],
        6543);
    let m = MessageStruct::new(3, "meudjancjdieks,aksj");
    println!("{}", v);
    println!("{}", mv);
    println!("{}", m);
}