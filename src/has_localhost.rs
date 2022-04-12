use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};

pub(crate) trait HasLocalhost {
    fn has_localhost(&self) -> bool;
}

impl HasLocalhost for Ipv4Addr {
    fn has_localhost(&self) -> bool {
        Ipv4Addr::new(127, 0, 0, 1).eq(self)
    }
}

impl HasLocalhost for Ipv6Addr {
    fn has_localhost(&self) -> bool {
        Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).eq(self)
    }
}

impl HasLocalhost for IpAddr {
    fn has_localhost(&self) -> bool {
        match self {
            IpAddr::V4(ref a) => a.has_localhost(),
            IpAddr::V6(ref a) => a.has_localhost(),
        }
    }
}

impl HasLocalhost for SocketAddr {
    fn has_localhost(&self) -> bool {
        self.ip().has_localhost()
    }
}
