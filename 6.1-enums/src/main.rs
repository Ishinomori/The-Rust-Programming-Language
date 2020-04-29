#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    fn route(ip_kind: IpAddrKind) {}

    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // only with enum
    enum IpAddrOnlyEnum {
        V4(String),
        V6(String),
    }

    let home = IpAddrOnlyEnum::V4(String::from("127.0.0.1"));
    let loopback = IpAddrOnlyEnum::V6(String::from("::1"));

    enum IpAddrMultiTypes {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddrMultiTypes::V4(127, 0, 0, 1);
    let loopback = IpAddrMultiTypes::V6(String::from("::1"));

    /* ------------------------------------------------------------- */

    enum Message {
        Quit,                       // no data associated
        Move { x: i32, y: i32 },    // anonymous struct
        Write(String),              // String
        ChangeColor(i32, i32, i32), // three i32
    }

    // same stuff with structs
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    // method for Message
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
}
