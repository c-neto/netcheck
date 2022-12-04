use std::net::Ipv4Addr;


fn report_ip_metadata(ip: Ipv4Addr) {
    let mut kind_checks = vec![
        ("broadcast", ip.is_broadcast()),
        ("documentation", ip.is_documentation()),
        ("link_local", ip.is_link_local()),
        ("loopback", ip.is_loopback()),
        ("multicast", ip.is_multicast()),
        ("private", ip.is_private()),
        ("unspecified", ip.is_unspecified())
    ];
    
    let is_public_ip = kind_checks
        .iter()
        .all(|(_kind, is_kind)| *is_kind == false);

    if is_public_ip {
        kind_checks.push(("public", true));
    }

    for (kind, is_kind) in kind_checks {
        if is_kind {
            println!("{}", kind)
        }
    }
}

fn main() {
    let ip_input = std::env::args()
        .nth(1)
        .expect("no ip given");

    let result = ip_input.parse::<Ipv4Addr>();

    match result {
        Ok(ip) => {
            report_ip_metadata(ip)
        }
        Err(error) => {
            println!("{}", error);
        }
    }
}
