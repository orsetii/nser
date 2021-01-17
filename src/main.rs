use std::fs::File;
use std::io::Read;

mod packet;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T, Error>;

use crate::packet::*;

fn main() -> Result<()> {
    let mut f = File::open("../data/response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("{:#?}", packet.header);

    for q in packet.questions {
        println!("{:#?}", q);
    }
    for rec in packet.answers {
        println!("{:#?}", rec);
    }
    for rec in packet.authorities {
        println!("{:#?}", rec);
    }
    for rec in packet.resources {
        println!("{:#?}", rec);
    }

    Ok(())
}