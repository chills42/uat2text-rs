#[macro_use]
extern crate bitfield;
extern crate byteorder;
extern crate hex;
extern crate num;
extern crate num_derive;

use std::fmt;
use std::io::{self, BufRead};
use std::io::Cursor;

use byteorder::{BigEndian, ReadBytesExt};

mod aux_state_vector;
use aux_state_vector::*;

mod header;
use header::*;

mod mode_status;
use mode_status::*;

mod state_vector;
use state_vector::*;

bitfield!{
  pub struct TargetState(u64);
  impl Debug;
  pub heading_or_track, _ : 63, 49;
  pub target_altitude, _ : 48, 32;
}

impl TargetState {
}

impl fmt::Display for TargetState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " Heading Or Track raw:          {:?}", self)
    }
}


struct UatMessage {
  header: UatHeader,
  state_vector: Option<StateVector>,
  mode_status: Option<ModeStatus>,
}

impl fmt::Display for UatMessage {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.header.code() {
      0 ..= 1 => {
        if self.state_vector.is_some() {
          if self.mode_status.is_some() {
          write!(f, "{}{}\n{}\n", self.header, self.state_vector.as_ref().unwrap(), self.mode_status.as_ref().unwrap())
          } else {
            write!(f, "{}{}\n\n", self.header, self.state_vector.as_ref().unwrap())
          }
        } else {
          write!(f, "{}", self.header)
        }
      },
      _ => write!(f, "ASET")
    }
  }
}

fn print_downlink(line: String) {
    let rest = &line[1..];
    println!("DOWNLINK\n{}", rest);
    let data = hex::decode(&line[1..].split(';').next().unwrap()).expect("hex decode");
    let mut rdr = Cursor::new(&data[0..4]);
    let val = rdr.read_u32::<BigEndian>().unwrap();
    let header = UatHeader(val);

    println!("  Code: {}", header.code());

    let mut state_data = Vec::new();
    state_data.extend_from_slice(&data[4..17]);
    state_data.push(u8::min_value());
    state_data.push(u8::min_value());
    state_data.push(u8::min_value());
    state_data.push(u8::min_value());
    let mut rdr = Cursor::new(state_data);
    let sv = rdr.read_u128::<BigEndian>().map(StateVector).ok();

    let ms = match  header.code() {
      1 | 3 => {
        let mut mode_data = Vec::new();
        mode_data.extend_from_slice(&data[17..28]);
        mode_data.push(u8::min_value());
        mode_data.push(u8::min_value());
        mode_data.push(u8::min_value());
        mode_data.push(u8::min_value());
        mode_data.push(u8::min_value());
        let mut rdr = Cursor::new(mode_data);
        Some(rdr.read_u128::<BigEndian>().map(ModeStatus).unwrap())
      },
      _ => None
    };
    println!("{:?}", ms);

    let aux_sv = match header.code() {
      1 | 2 | 5 | 6 => {
        let mut aux_data = Vec::new();
        aux_data.extend_from_slice(&data[29..33]);
        aux_data.push(u8::min_value());
        aux_data.push(u8::min_value());
        aux_data.push(u8::min_value());
        aux_data.push(u8::min_value());
        let mut rdr = Cursor::new(aux_data);
        Some(rdr.read_u64::<BigEndian>().map(AuxStateVector).unwrap())
      },
      _ => None
    };
    let target_state = match header.code() {
      3 | 4 => {
        let mut target_data = Vec::new();
        target_data.extend_from_slice(&data[29..33]);
        target_data.push(u8::min_value());
        target_data.push(u8::min_value());
        target_data.push(u8::min_value());
        target_data.push(u8::min_value());
        let mut rdr = Cursor::new(target_data);
        Some(rdr.read_u64::<BigEndian>().map(TargetState).unwrap())
      },
      6 => {
        let mut target_data = Vec::new();
        target_data.extend_from_slice(&data[24..28]);
        target_data.push(u8::min_value());
        target_data.push(u8::min_value());
        target_data.push(u8::min_value());
        target_data.push(u8::min_value());
        let mut rdr = Cursor::new(target_data);
        Some(rdr.read_u64::<BigEndian>().map(TargetState).unwrap())
      },
      _ => None
    };
    if let Some(aux_state_vector_value) = aux_sv {
      println!("{}", aux_state_vector_value);
    }

    if let Some(target_state_value) = target_state {
      println!("{:?}", target_state_value);
    }
    let message = UatMessage { header, state_vector: sv, mode_status: ms };
    println!("{}", message);
}

fn main() {
    let stdin = io::stdin();
    for line in stdin.lock().lines().flatten() {
      let ch = line.chars().next().unwrap();
      match ch {
        '+' => print_uplink(line),
        '-' => print_downlink(line),
        _ => println!("UNKNOWN MESSAGE"),
      }
    }
}

fn print_uplink(line: String) {
    let rest = &line[1..];
    println!("UPLINK\n{}", rest);
}
