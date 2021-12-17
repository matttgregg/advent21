use crate::{DayResult, DaySolver};
use std::time::SystemTime;

pub struct Day {}

impl DaySolver for Day {
    fn solve(&self) -> DayResult {
        let data = include_str!("data/day16.dat");
        let start = SystemTime::now();

        let mut program = Program::new(data);
        let packets = program.read_all_packets();
        let version_total = program.version_total;
        let evaluated = packets[0].evaluate();

        let timed = SystemTime::now().duration_since(start).unwrap();
        let description = format!(
            "Sum of all versions numbers = {}, evaluated to {}",
            version_total, evaluated
        );

        DayResult {
            description,
            part1: format!("{}", version_total),
            part2: format!("{}", evaluated),
            timing_us: timed.as_micros(),
        }
    }
}

#[derive(Debug)]
enum PacketContents {
    Literal(u128),
    SubPackets(Vec<Packet>),
}

#[derive(Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    contents: PacketContents,
}

impl Packet {
    fn evaluate(&self) -> u64 {
        match self.type_id {
            0 => {
                // Sum packet.
                let mut acc = 0;
                if let PacketContents::SubPackets(packets) = &self.contents {
                    for p in packets {
                        acc += p.evaluate()
                    }
                    acc
                } else {
                    panic!("Expected sub packets.")
                }
            }
            1 => {
                // Product packet.
                let mut acc = 1;
                if let PacketContents::SubPackets(packets) = &self.contents {
                    for p in packets {
                        acc *= p.evaluate()
                    }
                    acc
                } else {
                    panic!("Expected sub packets.")
                }
            }
            2 => {
                // Minimum packet.
                let mut acc = u64::max_value();
                if let PacketContents::SubPackets(packets) = &self.contents {
                    for p in packets {
                        acc = std::cmp::min(acc, p.evaluate())
                    }
                    acc
                } else {
                    panic!("Expected sub packets.")
                }
            }
            3 => {
                // Maximum packet.
                let mut acc = 0u64;
                if let PacketContents::SubPackets(packets) = &self.contents {
                    for p in packets {
                        acc = std::cmp::max(acc, p.evaluate())
                    }
                    acc
                } else {
                    panic!("Expected sub packets.")
                }
            }
            4 => {
                if let PacketContents::Literal(literal) = &self.contents {
                    *literal as u64
                } else {
                    panic!("Expected literal")
                }
            }
            5 => {
                // Greater-than packet.
                if let PacketContents::SubPackets(packets) = &self.contents {
                    if packets[0].evaluate() > packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                } else {
                    panic!("Expected sub packets")
                }
            }
            6 => {
                // Less-than packet.
                if let PacketContents::SubPackets(packets) = &self.contents {
                    if packets[0].evaluate() < packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                } else {
                    panic!("Expected sub packets")
                }
            }
            7 => {
                // Equality packet.
                if let PacketContents::SubPackets(packets) = &self.contents {
                    if packets[0].evaluate() == packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                } else {
                    panic!("Expected sub packets")
                }
            }
            _ => panic!("unrecognized operation"),
        }
    }
}

struct Program {
    data: Vec<char>,
    buffer: Vec<char>,
    ptr: u128,
    version_total: u128,
}

impl Program {
    fn new(string_data: &str) -> Self {
        let mut data = string_data.chars().collect::<Vec<char>>();
        data.reverse();

        Self {
            data,
            buffer: vec![],
            ptr: 0u128,
            version_total: 0u128,
        }
    }

    fn ended(&self) -> bool {
        self.buffer.iter().all(|c| *c == '0') && self.data.iter().all(|c| *c == '0')
    }

    fn read_all_packets(&mut self) -> Vec<Packet> {
        let mut all_packets = vec![];
        while !self.ended() {
            all_packets.push(self.next_packet());
        }
        all_packets
    }

    fn next_packet(&mut self) -> Packet {
        // Take 3 for the version.
        let version = self.read_3_as_u8();
        self.version_total += version as u128;
        // Take 3 for the ID.
        let type_id = self.read_3_as_u8();
        // Decide on the type.
        let contents = if type_id == 4 {
            // Reading a literal.
            PacketContents::Literal(self.read_literal())
        } else {
            // Nested
            self.read_sub_packets()
        };

        Packet {
            version,
            type_id,
            contents,
        }
    }

    fn read_sub_packets(&mut self) -> PacketContents {
        // Check the length type
        let length_type = self.read_one_u8();

        let mut sub_packets = vec![];
        if length_type == 0 {
            // Read 15 bits, as a bit length.
            let bit_length = self.read_n_as_u32(15);
            let end_at = self.ptr + bit_length as u128;
            while self.ptr < end_at {
                sub_packets.push(self.next_packet());
            }
        } else {
            // read 11 bits, as a packet count.
            let packet_count = self.read_n_as_u32(11) as usize;
            for _ in 0..packet_count {
                sub_packets.push(self.next_packet());
            }
        }

        PacketContents::SubPackets(sub_packets)
    }

    fn read_literal(&mut self) -> u128 {
        let mut cont = 1u8;
        let mut val = 0u128;
        while cont == 1u8 {
            val *= 16u128;
            cont = self.read_one_u8();
            val += self.read_4_as_u8() as u128;
        }

        val
    }

    fn read_n_as_u32(&mut self, n: usize) -> u32 {
        let mut value = 0u32;
        for _ in 0..n {
            value *= 2;
            value += self.read_one_u8() as u32
        }
        value
    }

    fn read_3_as_u8(&mut self) -> u8 {
        let mut value = 0u8;
        for power in [4, 2, 1].iter() {
            value += power * self.read_one_u8()
        }
        value
    }

    fn read_4_as_u8(&mut self) -> u8 {
        let mut value = 0u8;
        for power in [8, 4, 2, 1].iter() {
            value += power * self.read_one_u8()
        }
        value
    }

    fn read_one_u8(&mut self) -> u8 {
        match self.read_one() {
            Some('1') => 1u8,
            Some('0') => 0u8,
            Some(_) => panic!("unexpected char in input"),
            _ => panic!("reached end of input"),
        }
    }

    fn read_one(&mut self) -> Option<char> {
        self.ptr += 1;
        if let Some(bin_digit) = self.buffer.pop() {
            Some(bin_digit)
        } else if let Some(hex_digit) = self.data.pop() {
            // We push the digits into the buffer (in reverse order so we 'take' from the front.
            let hex_value = hex_digit.to_digit(16).unwrap();
            self.buffer = format!("{:04b}", hex_value).chars().collect::<Vec<char>>();
            self.buffer.reverse();
            self.buffer.pop()
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data() {
        for example in [
            ("D2FE28", 6),
            ("38006F45291200", 9),
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ]
        .iter()
        {
            let mut program = Program::new(example.0);
            let _packets = program.read_all_packets();
            assert_eq!(program.version_total, example.1);
        }

        for example in [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
        ]
        .iter()
        {
            let mut program = Program::new(example.0);
            let packets = program.read_all_packets();
            assert_eq!(packets[0].evaluate(), example.1);
        }
    }
}
