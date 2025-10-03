#[derive(Debug)]
struct Point {
    x: u64,
    y: u64,
}

struct UnitStruct;

#[derive(Debug)]
enum Message {
    // TODO: Define the different variants used below.
    Resize,
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
    Move(Point),
}

impl Message {
    fn call(&self) {
        println!("{self:?}");
    }
}

fn main() {
    let messages = [
        Message::Resize {},
        Message::Move(Point { x: 10, y: 15 }),
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
