use std::io::BufRead;
use std::str::Chars;

struct Packet {
    value: u64,
    subpackets: Vec<Packet>,
    version: u8,
    id: u8,
}

impl Packet {
    pub fn new() -> Self {
        Self {
            value: 0,
            subpackets: vec![],
            version: 0,
            id: 0
        }
    }

    pub fn from(bit_str: &String) -> Self {
        Self::from_iter(&mut bit_str.chars())
    }

    pub fn from_iter(chars: &mut Chars) -> Self {
        let mut packet = Self::new();
        for _ in 0..3 {
            packet.version <<= 1;
            packet.version += ((chars.next().unwrap() as u8) - 0x30);
        }
        for _ in 0..3 {
            packet.id <<= 1;
            packet.id += ((chars.next().unwrap() as u8) - 0x30);
        }
        if packet.id == 4 {
            while chars.next().unwrap() == '1' {
                for _ in 0..4 {
                    packet.value <<= 1;
                    packet.value += ((chars.next().unwrap() as u64) - 0x30);
                }
            }
            for _ in 0..4 {
                packet.value <<= 1;
                packet.value += ((chars.next().unwrap() as u64) - 0x30);
            }
        } else {
            if chars.next().unwrap() == '0' {
                let mut bytes = 0;
                for _ in 0..15 {
                    bytes <<= 1;
                    bytes += ((chars.next().unwrap() as u16) - 0x30);
                }
                let mut buf: String = String::new();
                for _ in 0..bytes {
                    buf.push(chars.next().unwrap());
                }
                let mut buf_chars = buf.chars();
                while buf_chars.clone().next().is_some() {
                    packet.subpackets.push(Packet::from_iter(&mut buf_chars));
                }
            } else {
                let mut packets = 0;
                for _ in 0..11 {
                    packets <<= 1;
                    packets += ((chars.next().unwrap() as u16) - 0x30);
                }
                for _ in 0..packets {
                    packet.subpackets.push(Packet::from_iter(chars));
                }
            }
        }


        packet
    }

    pub fn evaluate(&self) -> u64 {
        match self.id {
            0 => {
                self.subpackets.iter().map(|packet|packet.evaluate()).sum()
            }
            1 => {
                self.subpackets.iter().map(|packet|packet.evaluate()).product()
            }
            2 => {
                self.subpackets.iter().map(|packet|packet.evaluate()).min().unwrap()
            }
            3 => {
                self.subpackets.iter().map(|packet|packet.evaluate()).max().unwrap()
            }
            4 => self.value,
            5 => match self.subpackets[0].evaluate() > self.subpackets[1].evaluate() {
                true => 1,
                false => 0
            }
            6 => match self.subpackets[0].evaluate() < self.subpackets[1].evaluate() {
                true => 1,
                false => 0
            }
            7 => match self.subpackets[0].evaluate() == self.subpackets[1].evaluate() {
                true => 1,
                false => 0
            }
            _ => 0
        }
    }
}

fn main() {
    part_two();
}

fn part_one() {
    let stdin = std::io::stdin();

    let mut hex_string = String::new();
    stdin.read_line(&mut hex_string);

    let mut bit_str: String = String::new();
    for str in hex_string.chars().map(|c| match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => ""
    }) {
        bit_str.push_str(str);
    }

    let packet: Packet = Packet::from(&bit_str);

    println!("Version Sum: {}", packet_version_sum(&packet));
}

fn part_two() {
    let stdin = std::io::stdin();

    let mut hex_string = String::new();
    stdin.read_line(&mut hex_string);

    let mut bit_str: String = String::new();
    for str in hex_string.chars().map(|c| match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => ""
    }) {
        bit_str.push_str(str);
    }

    let packet: Packet = Packet::from(&bit_str);

    println!("Version Sum: {}", (&packet).evaluate());
}

fn packet_version_sum(packet: &Packet) -> u64 {
    (packet.version as u64) + packet.subpackets.iter().map(|p|packet_version_sum(p)).sum::<u64>()
}

