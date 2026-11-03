// enum: 가능한 variant(enum 타입이 가질 수 있는 각각의 값)를 열거해서 type을 정의하는 방법이다.
// -> nominal한 enumerated type과, 그 type의 값을 만들거나 pattern-match하는 데 쓸 수 있는 constructor 집합을 동시에 정의
// -> struct는 관련 field를 묶고, enum은 '값이 가능한 값 집합 중 하나'임을 말한다.

#[derive(Debug)]
enum IpAddrKind {
    v4,
    v6,
}

fn route(ip_kind: &IpAddrKind) {
    println!("route: {:?}", ip_kind);
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

impl IpAddr {
    fn new(kind: IpAddrKind, address: String) -> Self {
        Self { kind, address }
    }

    fn print(&self) {
        println!("[ip info] -> kind: {:?}, addr: {}", self.kind, self.address);
    }
}

struct Ipv4Addr {
    address: [u8; 4],
}

impl Ipv4Addr {
    fn new(address: [u8; 4]) -> Self {
        Self { address }
    }
}

struct Ipv6Addr {
    address: [u8; 16],
}

impl Ipv6Addr {
    fn new(address: [u8; 16]) -> Self {
        Self { address }
    }
}

enum IpAddress {
    v4(Ipv4Addr),
    v6(Ipv6Addr),
}

impl IpAddress {
    fn print(&self) {
        match self {
            Self::v4(addr) => {
                println!("[ipv4]: {:?}", addr.address);
            }
            Self::v6(addr) => {
                println!("[ipv6]: {:x?}", addr.address);
            }
        }
    }

    fn as_ipv4(&self) -> Option<&Ipv4Addr> {
        match self {
            Self::v4(ip) => Some(ip),
            Self::v6(_) => None,
        }
    }
}

// 정의한 각 variant의 이름은 그 enum의 instance를 만드는 function(constructor)이 된다.
// variant마다 연결된 data의 type과 개수가 달라도 된다.
// enum의 Variant에는 어떤 타입이든 넣을 수 있다.
/*
enum IpAddress {
    v4(u8, u8, u8, u8),
    v6(String),
}
*/

// 같은 data를 struct 여러 개로 담으면 각자 다른 type이 되어 한 function으로 받기 어렵다.
// 예시
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);

enum Message {
    Quit,
    Move { x: i32, y: i32 }, // struct-like variant
    Write(String),
    ChangeColor(i32, i32, i32), // tuple-like variant
}

impl Message {
    fn call(&self) {
        //...
    }
}

fn main() {
    let ipv4 = IpAddrKind::v4;
    let ipv6 = IpAddrKind::v6;
    println!("ipv4: {:?}, ipv6: {:?}", ipv4, ipv6);

    route(&ipv4);
    route(&ipv6);

    let home = IpAddr::new(ipv4, String::from("127.0.0.1"));
    home.print();

    let loopback = IpAddr::new(ipv6, String::from("::1"));
    loopback.print();

    let enum_home = IpAddress::v4(Ipv4Addr::new([127, 0, 0, 1]));
    let enum_loopback = IpAddress::v6(Ipv6Addr::new([
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1,
    ]));
    enum_home.print();
    enum_loopback.print();

    /*
    let ipv4_addr: Option<&Ipv4Addr> = enum_home.as_ipv4();
    let default_addr = Ipv4Addr::new([0, 0, 0, 0]);
    let addr = ipv4_addr.unwrap_or(&default_addr);

    println!("[ipv4] {:?}", addr.address);
    */

    /*
    let m = Message::Write((String::from("hello")));
    m.call();
    */
}
