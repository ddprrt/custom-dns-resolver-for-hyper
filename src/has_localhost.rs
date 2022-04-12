use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    vec::IntoIter,
};

pub(crate) trait IsLocalhost {
    fn is_localhost(&self) -> bool;
}

impl IsLocalhost for Ipv4Addr {
    fn is_localhost(&self) -> bool {
        Ipv4Addr::new(127, 0, 0, 1).eq(self) || Ipv4Addr::new(0, 0, 0, 0).eq(self)
    }
}

impl IsLocalhost for Ipv6Addr {
    fn is_localhost(&self) -> bool {
        Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).eq(self)
    }
}

impl IsLocalhost for IpAddr {
    fn is_localhost(&self) -> bool {
        match self {
            IpAddr::V4(ref a) => a.is_localhost(),
            IpAddr::V6(ref a) => a.is_localhost(),
        }
    }
}

impl IsLocalhost for SocketAddr {
    fn is_localhost(&self) -> bool {
        self.ip().is_localhost()
    }
}

pub(crate) trait HasLocalhost {
    fn has_localhost(&mut self) -> bool;
}

impl HasLocalhost for IntoIter<SocketAddr> {
    fn has_localhost(&mut self) -> bool {
        self.any(|el| el.is_localhost())
    }
}
