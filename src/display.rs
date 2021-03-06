use std::fmt;

use { Segment, MultiAddr };

impl fmt::Display for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "/{}", self.name()));
        match *self {
            Segment::IP4(ref addr) => {
                try!(write!(f, "/{}", addr));
            }
            Segment::IP6(ref addr) => {
                try!(write!(f, "/{}", addr));
            }
            Segment::Udp(ref port)
            | Segment::Dccp(ref port)
            | Segment::Sctp(ref port)
            | Segment::Tcp(ref port) => {
                try!(write!(f, "/{}", port));
            }
            Segment::Ipfs(ref multihash) => {
                try!(write!(f, "/{}", multihash));
            }
            Segment::Udt
            | Segment::Utp
            | Segment::Http
            | Segment::Https => {
            }
        }
        Ok(())
    }
}

impl fmt::Display for MultiAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for segment in self.segments() {
            try!(write!(f, "{}", segment));
        }
        Ok(())
    }
}

impl fmt::Debug for MultiAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(f.write_str("MultiAddr(\""));
        try!(fmt::Display::fmt(self, f));
        try!(f.write_str("\")"));
        Ok(())
    }
}

impl fmt::Debug for Segment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(f.write_str("Segment(\""));
        try!(fmt::Display::fmt(self, f));
        try!(f.write_str("\")"));
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::net::{ Ipv4Addr, Ipv6Addr };

    use mhash::{ MultiHash, MultiHashVariant };

    use { MultiAddr, Segment };

    #[test]
    fn ip4() {
        assert_eq!(
            MultiAddr::new(vec![Segment::IP4(Ipv4Addr::new(1, 2, 3, 4))]).to_string(),
            "/ip4/1.2.3.4");
    }

    #[test]
    fn ip6() {
        let addr = Ipv6Addr::new(0x2a02, 0x6b8, 0, 0, 0, 0, 0x11, 0x11);
        assert_eq!(
            MultiAddr::new(vec![Segment::IP6(addr)]).to_string(),
            "/ip6/2a02:6b8::11:11");
    }

    #[test]
    fn ipfs() {
        let multihash = MultiHash::new(MultiHashVariant::Sha2_256, &[
            213, 46, 187, 137, 216, 91, 2, 162,
            132, 148, 130, 3, 166, 47, 242, 131,
            137, 197, 124, 159, 66, 190, 236, 78,
            194, 13, 183, 106, 104, 145, 28, 11,
        ]).unwrap();
        assert_eq!(
            MultiAddr::new(vec![Segment::Ipfs(multihash)]).to_string(),
            "/ipfs/QmcgpsyWgH8Y8ajJz1Cu72KnS5uo2Aa2LpzU7kinSupNKC");
    }
}
