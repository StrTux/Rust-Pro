#[derive(Debug)]
enum IPAddKind {
    IPv4,
    IPv6,
}

fn main() {
    let ip = "192.168.1.1";
    let kind = IPAddKind::IPv4;
    route(ip, kind);
}

fn route(ip: &str, kind: IPAddKind) {
    println!("Routing request ip {} of kind {:?}", ip, kind);
}



