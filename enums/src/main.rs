mod option_enum;
mod match_pattern;
mod if_let_pattern;

enum IpAddrKind {
    V4,
    V6,
}

enum IpAddrInfo {
    V4(u8, u8, u8, u8), // We can also store associated data directly in the enum variants
    V6(String), // We are de facto making a constructor that takes a String argument
}


struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum Message { // Enums can contain data as well as structs.
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message  {
    fn call (&self) {
        // method body would be defined here
    }
}


fn main() {
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let ip = IpAddrInfo::V4(127, 0, 0, 1);
    let ip_6 = IpAddrInfo::V6(String::from("::1"));


    let m  = Message::Write(String::from("hello"));
    m.call();


    match_pattern::run();

}

fn route (ip_kind: IpAddrKind) {

}