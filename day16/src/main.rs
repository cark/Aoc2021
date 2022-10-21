//#![allow(dead_code)]

// No allocation... weee !

use std::marker::PhantomData;

fn main() {
    let input = include_str!("input.txt");
    println!("Part1 : {}", Reader::new(input).version_sum());
    println!("Part2 : {}", Reader::new(input).eval());
}

#[derive(Clone, Copy)]
struct Reader<'a> {
    text: &'a str,
    bit_index: usize,
}

impl<'a> Reader<'a> {
    fn new(text: &'a str) -> Self {
        Self { text, bit_index: 0 }
    }

    #[inline(always)]
    fn hex_at(&self, string_index: usize) -> u8 {
        char::from_u32(self.text.as_bytes()[string_index] as u32)
            .unwrap()
            .to_digit(16)
            .unwrap() as u8
    }

    fn next_bits(&mut self, bit_count: u8) -> u64 {
        let mut hex_index = usize::MAX;
        let mut hex = None;
        let mut result: u64 = 0;
        let mut bits_left = bit_count;
        while bits_left > 0 {
            result <<= 1;
            let (curr_index, bit_index) = (self.bit_index >> 2, self.bit_index & 0b11);
            if curr_index != hex_index {
                hex_index = curr_index;
                hex = Some(self.hex_at(hex_index));
            }
            let bit = ((hex.unwrap() >> (3 - bit_index)) & 1) as u64;
            result |= bit;
            self.bit_index += 1;
            bits_left -= 1
        }
        result
    }

    fn version_sum(&mut self) -> u64 {
        Walker::<VersionSum>::new(self).walk()
    }

    fn eval(&mut self) -> u64 {
        Walker::<Evaluator>::new(self).walk()
    }
}

struct Walker<'a, 'b, W: WalkReducer + ?Sized> {
    reader: &'b mut Reader<'a>,
    reducer: PhantomData<W>,
}

impl<'a, 'b, W: WalkReducer> Walker<'a, 'b, W> {
    fn new(reader: &'b mut Reader<'a>) -> Self {
        Walker {
            reader,
            reducer: PhantomData::<W>,
        }        
    }
    
    fn walk(&mut self) -> u64 {
        self.walk_packet()
    }

    fn walk_packet(&mut self) -> u64 {
        let version = self.reader.next_bits(3);
        let type_id = self.reader.next_bits(3);

        match type_id {
            4 => W::process_packet(self, version, 4, None),
            _ => {
                let state = OpPacketsState::new(self.reader);
                W::process_packet(self, version, type_id, Some(state))
            }
        }
    }

    fn literal_groups_val(&mut self) -> u64 {
        let mut result = 0;
        loop {
            result <<= 4;
            let group = self.reader.next_bits(5);
            result += group & 0b1111;
            if group & 0b10000 == 0 {
                break result;
            }
        }
    }

    fn walk_op_packet(&mut self, op_packet_state: &mut OpPacketsState) -> u64 {
        match op_packet_state {
            OpPacketsState::Type0 { bits, .. } => {
                let current_bit = self.reader.bit_index;
                let result = self.walk_packet();
                *bits += self.reader.bit_index - current_bit;
                result
            }
            OpPacketsState::Type1 { packets, .. } => {
                *packets += 1;
                self.walk_packet()
            }
        }
    }

    fn reduce_op_packets<F: Fn(u64, u64) -> u64>(
        &mut self,
        op_packet_state: &mut OpPacketsState,
        mut state: u64,
        f: F,
    ) -> u64 {
        while op_packet_state.has_more() {
            state = f(state, self.walk_op_packet(op_packet_state));
        }
        state
    }
}

trait WalkReducer {
    fn process_packet(
        walker: &mut Walker<'_, '_, Self>,
        version: u64,
        type_id: u64,
        op_packet_state: Option<OpPacketsState>,
    ) -> u64;
}

struct VersionSum;

impl WalkReducer for VersionSum {
    fn process_packet(
        walker: &mut Walker<'_, '_, Self>,
        version: u64,
        type_id: u64,
        op_packet_state: Option<OpPacketsState>,
    ) -> u64 {
        match type_id {
            4 =>  {
                walker.literal_groups_val();
                version
            },
            _ => version + walker.reduce_op_packets(&mut op_packet_state.unwrap(), 0, u64::wrapping_add)
        }
    }
}

struct Evaluator;

impl WalkReducer for Evaluator {
    fn process_packet(
        walker: &mut Walker<'_, '_, Self>,
        _version: u64,
        type_id: u64,
        op_packet_state: Option<OpPacketsState>,
    ) -> u64 {
        match type_id {
            0 => walker.reduce_op_packets(&mut op_packet_state.unwrap(), 0, u64::wrapping_add),
            1 => walker.reduce_op_packets(&mut op_packet_state.unwrap(), 1, u64::wrapping_mul),
            2 => walker.reduce_op_packets(&mut op_packet_state.unwrap(), u64::MAX, u64::min),
            3 => walker.reduce_op_packets(&mut op_packet_state.unwrap(), 0, u64::max),
            4 => walker.literal_groups_val(),
            5 => u64::from(walker.walk_packet() > walker.walk_packet()),
            6 => u64::from(walker.walk_packet() < walker.walk_packet()),
            7 => u64::from(walker.walk_packet() == walker.walk_packet()),
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
enum OpPacketsState {
    Type0 { bit_count: usize, bits: usize },
    Type1 { packet_count: usize, packets: usize },
}

impl OpPacketsState {
    fn new(reader: &'_ mut Reader) -> Self {
        if reader.next_bits(1) == 0 {
            OpPacketsState::Type0 {
                bit_count: reader.next_bits(15) as usize,
                bits: 0,
            }
        } else {
            OpPacketsState::Type1 {
                packet_count: reader.next_bits(11) as usize,
                packets: 0,
            }
        }
    }
    fn has_more(&self) -> bool {
        match self {
            OpPacketsState::Type0 { bit_count, bits } => bits < bit_count,
            OpPacketsState::Type1 {
                packet_count,
                packets,
            } => packets < packet_count,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_at() {
        let r = Reader::new("D2FE28");
        assert_eq!(r.hex_at(0), 13);
        assert_eq!(r.hex_at(1), 2);
        assert_eq!(r.hex_at(2), 15);
        assert_eq!(r.hex_at(5), 8);
    }

    #[test]
    fn test_next_bits() {
        let mut r = Reader::new("D2FE28");
        assert_eq!(r.next_bits(3), 6);
        assert_eq!(r.next_bits(3), 4);
        assert_eq!(r.next_bits(5), 0b10111);
        assert_eq!(r.next_bits(5), 0b11110);
        assert_eq!(r.next_bits(5), 0b00101);
        assert_eq!(r.next_bits(3), 0);
    }

    #[test]
    fn test_version_sum() {
        assert_eq!(Reader::new("D2FE28").version_sum(), 6);
        assert_eq!(Reader::new("8A004A801A8002F478").version_sum(), 16);
        assert_eq!(Reader::new("620080001611562C8802118E34").version_sum(), 12);
        assert_eq!(
            Reader::new("C0015000016115A2E0802F182340").version_sum(),
            23
        );
        assert_eq!(
            Reader::new("A0016C880162017C3686B18A3D4780").version_sum(),
            31
        );
    }

    #[test]
    fn test_eval() {
        assert_eq!(Reader::new("C200B40A82").eval(), 3);
        assert_eq!(Reader::new("04005AC33890").eval(), 54);
        assert_eq!(Reader::new("9C0141080250320F1802104A08").eval(), 1);
    }
}
