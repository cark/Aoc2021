//#![allow(dead_code)]

fn main() {
    let text = include_str!("input.txt");
    let bit_vec = BitVec::from_hex(text);
    println!(
        "Part1 : {}",
        Packet::parse(&mut bit_vec.reader()).version_sum()
    );
    println!("Part2 : {}", Packet::parse(&mut bit_vec.reader()).eval());
}

struct BitVec {
    bytes: Vec<u8>,
}

#[inline]
fn reverse_hex(mut val: u8) -> u8 {
    let mut result = 0;
    for _ in 0..4 {
        result <<= 1;
        result |= val & 1;
        val >>= 1;
    }
    result
}

impl BitVec {
    fn from_hex(hex_str: &str) -> BitVec {
        let trimed = hex_str.trim();
        let mut bytes: Vec<u8> = vec![0; trimed.len() / 2];
        for (i, char) in trimed.chars().enumerate() {
            let hex = reverse_hex(char.to_digit(16).unwrap() as u8);
            bytes[i >> 1] |= hex << ((i & 1) << 2);
        }
        BitVec { bytes }
    }

    fn reader(&self) -> BitReader<'_> {
        BitReader {
            bit_vec: self,
            at_bit: 0,
        }
    }
}

struct BitReader<'a> {
    bit_vec: &'a BitVec,
    at_bit: usize,
}

impl BitReader<'_> {
    #[inline]
    fn len(&self) -> usize {
        self.bit_vec.bytes.len() * 8
    }

    fn next_bit(&mut self) -> Option<u8> {
        if self.at_bit < self.len() {
            let byte_index = self.at_bit / 8;
            let bit_index = self.at_bit % 8;
            self.at_bit += 1;
            Some((self.bit_vec.bytes[byte_index] >> bit_index) & 1)
        } else {
            None
        }
    }

    fn next_bits(&mut self, length: u8) -> usize {
        let mut result = 0;
        for _ in 0..length {
            result <<= 1;
            if let Some(bit) = self.next_bit() {
                result += usize::from(bit);
            } else {
                panic!("input exhausted");
            }
        }
        result
    }
}

struct PacketHeader {
    version: u8,
    type_id: u8,
}

impl PacketHeader {
    fn parse(reader: &mut BitReader) -> Self {
        Self {
            version: reader.next_bits(3) as u8,
            type_id: reader.next_bits(3) as u8,
        }
    }
}

struct Packet {
    header: PacketHeader,
    payload: Payload,
}

impl Packet {
    fn parse(reader: &mut BitReader) -> Self {
        let header = PacketHeader::parse(reader);
        let payload = Payload::parse(reader, &header);
        Self { header, payload }
    }

    fn version_sum(&self) -> usize {
        self.header.version as usize
            + if let Payload::Operator(sub_packets) = &self.payload {
                sub_packets.iter().map(Packet::version_sum).sum()
            } else {
                0
            }
    }

    fn eval(&self) -> usize {
        self.payload.eval(self.header.type_id)
    }
}

enum Payload {
    LiteralValue(usize),
    Operator(Vec<Packet>),
}

impl Payload {
    fn parse(reader: &mut BitReader, header: &PacketHeader) -> Payload {
        match header.type_id {
            4 => Self::parse_literal(reader),
            _ => Self::parse_operator(reader),
        }
    }
    fn parse_literal(reader: &mut BitReader) -> Payload {
        let mut value: usize = 0;
        loop {
            let group = Group::parse(reader);
            value <<= 4;
            value |= group.value as usize;
            if group.last {
                break;
            }
        }
        Self::LiteralValue(value)
    }

    fn parse_operator(reader: &mut BitReader) -> Payload {
        let length_type_id = reader.next_bits(1);
        if length_type_id == 0 {
            let bit_count = reader.next_bits(15);
            let max_index = reader.at_bit + bit_count;
            let mut packets = vec![];
            while reader.at_bit < max_index {
                packets.push(Packet::parse(reader))
            }
            Self::Operator(packets)
        } else {
            let packet_count = reader.next_bits(11);
            let mut packets = Vec::with_capacity(packet_count);
            for _ in 0..packet_count {
                packets.push(Packet::parse(reader));
            }
            Self::Operator(packets)
        }
    }

    fn eval(&self, type_id: u8) -> usize {
        match self {
            Self::LiteralValue(value) => *value,
            Self::Operator(packets) => match type_id {
                0 => packets.iter().map(Packet::eval).sum(),
                1 => packets.iter().map(Packet::eval).product(),
                2 => packets.iter().map(Packet::eval).min().unwrap(),
                3 => packets.iter().map(Packet::eval).max().unwrap(),
                5 => usize::from(packets[0].eval() > packets[1].eval()),
                6 => usize::from(packets[0].eval() < packets[1].eval()),
                7 => usize::from(packets[0].eval() == packets[1].eval()),
                _ => unreachable!(),
            },
        }
    }
}

struct Group {
    last: bool,
    value: u8,
}

impl Group {
    fn parse(reader: &mut BitReader) -> Group {
        Group {
            last: reader.next_bits(1) != 1,
            value: reader.next_bits(4) as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reverse_hex() {
        assert_eq!(reverse_hex(0b1000), 0b0001);
        assert_eq!(reverse_hex(0b1100), 0b0011);
        assert_eq!(reverse_hex(0b1010), 0b0101);
    }

    #[test]
    fn test_from_hex() {
        let bv = BitVec::from_hex("D2FE28");
        let mut br = bv.reader();
        let mut s = "".to_string();
        while let Some(bit) = br.next_bit() {
            s.push(if bit == 1 { '1' } else { '0' });
        }
        assert_eq!(s, "110100101111111000101000");
    }

    #[test]
    fn test_next_bits() {
        let bv = BitVec::from_hex("D2FE28");
        let mut br = bv.reader();
        assert_eq!(br.next_bits(3), 6);
        assert_eq!(br.next_bits(3), 4);
        assert_eq!(br.next_bits(5), 23);
        assert_eq!(br.next_bits(5), 30);
        assert_eq!(br.next_bits(5), 5);
        assert_eq!(br.next_bits(3), 0);
        let result = std::panic::catch_unwind(move || br.next_bits(1));
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_header() {
        let bv = BitVec::from_hex("D2FE28");
        let mut br = bv.reader();
        let header = PacketHeader::parse(&mut br);
        assert_eq!(header.version, 6);
        assert_eq!(header.type_id, 4);
    }

    #[test]
    fn test_parse_literal() {
        let packet = Packet::parse(&mut BitVec::from_hex("D2FE28").reader());
        assert!(matches!(packet.payload, Payload::LiteralValue(2021)));
    }

    #[test]
    fn test_operator_parsing() {
        let packet = Packet::parse(&mut BitVec::from_hex("38006F45291200").reader());
        assert_eq!(packet.header.version, 1);
        assert_eq!(packet.header.type_id, 6);
        let Payload::Operator(packets) = packet.payload else { panic!("not an operator") };
        let Payload::LiteralValue(value) = packets[0].payload else { panic!("not a literal") };
        assert_eq!(value, 10);
        let Payload::LiteralValue(value) = packets[1].payload else { panic!("not a literal") };
        assert_eq!(value, 20);

        let packet = Packet::parse(&mut BitVec::from_hex("EE00D40C823060").reader());
        assert_eq!(packet.header.version, 7);
        assert_eq!(packet.header.type_id, 3);
        let Payload::Operator(packets) = packet.payload else { panic!("not an operator") };
        assert_eq!(packets.len(), 3);
    }

    #[test]
    fn test_version_sum() {
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("8A004A801A8002F478").reader()).version_sum(),
            16
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("620080001611562C8802118E34").reader())
                .version_sum(),
            12
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("C0015000016115A2E0802F182340").reader())
                .version_sum(),
            23
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("A0016C880162017C3686B18A3D4780").reader())
                .version_sum(),
            31
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("C200B40A82").reader()).eval(),
            3
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("04005AC33890").reader()).eval(),
            54
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("880086C3E88112").reader()).eval(),
            7
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("CE00C43D881120").reader()).eval(),
            9
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("D8005AC2A8F0").reader()).eval(),
            1
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("F600BC2D8F").reader()).eval(),
            0
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("9C005AC2F8F0").reader()).eval(),
            0
        );
        assert_eq!(
            Packet::parse(&mut BitVec::from_hex("9C0141080250320F1802104A08").reader()).eval(),
            1
        );
    }
}
