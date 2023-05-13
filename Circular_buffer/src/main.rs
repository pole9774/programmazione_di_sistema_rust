use circular_buffer::{CircularBuffer, Error};

fn main() {
    let mut buffer: CircularBuffer<&str> = CircularBuffer::new(5);

    buffer.write("uno");
    buffer.write("due");
    buffer.write("tre");
    buffer.write("quattro");
    buffer.write("cinque");
    buffer.write("sei"); // full
    
    buffer.overwrite("SETTE");

    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read());
    println!("{:?}", buffer.read()); // empty
    println!("{:?}", buffer.read()); // empty
}