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
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

bitfield!{
  pub struct UatHeader(u32);
  impl Debug;
  pub code, _ : 31, 27;
  pub qualifier, _ : 26, 24;
  pub address, _ : 23, 0;
}

#[derive(FromPrimitive, Debug)]
enum AddressQualifier {
  AdsbStandard = 0,
  AdsbSelfAssigned = 1,
  TisbStandard = 2,
  TisbTrackFile = 3,
  SurfaceVehicle = 4,
  AdsbFixed = 5,
  Reserved1 = 6,
  Reserved2 = 7
}

impl fmt::Display for AddressQualifier {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl UatHeader {
  fn address_qualifier(&self) -> AddressQualifier {
    FromPrimitive::from_u32(self.qualifier()).unwrap()
  }
}

impl fmt::Display for UatHeader {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HDR:\n MDB Type:           {}\n Address:            {:X} ({})\n", self.code(), self.address(), self.address_qualifier())
    }
}

bitfield!{
  pub struct StateVector(u128);
  impl Debug;
  pub raw_lat, _ : 127, 105;
  pub raw_lng, _ : 104, 81;
  pub raw_alt_type, _ : 80;
  pub raw_alt, _ : 79, 68;
  pub raw_nic, _ : 67, 64;
  pub raw_ag, _ : 63, 62;
  pub raw_ns, _ : 60;
  pub raw_ns_vel, _ : 59, 50;
  pub raw_ew, _ : 49;
  pub raw_ew_vel, _ : 48, 39;
  pub raw_vv_source, _ : 38;
  pub raw_vv_sign, _ : 37;
  pub raw_vv, _ : 36, 28;
  pub raw_utc, _ : 27;
}

impl fmt::Display for StateVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "SV:\n {}\n{}\n Altitude:           {} ft ({})\n N/S velocity:       {}\n E/W velocity:       {}\n Track: \n Speed: \n Vertical rate:      {}\n UTC coupling:       {}\n TIS-B site ID:      {}\n", self.nic_string(), self.geo(), self.alt(), self.alt_type(), self.ns_vel(), self.ew_vel(), self.vv_string(), if self.raw_utc() { "yes" } else { "no" }, self.tisb_site_id())
    }
}

impl StateVector {
  fn alt_type(&self) -> String {
      if self.raw_alt_type() { "Geometric" } else { "Barometric" }.to_owned()
  }

  fn alt(&self) -> i32 {
      self.raw_alt() as i32 * 25 - 1025
  }

  fn lat(&self) -> f64 {
    (self.raw_lat() as f64 * 360.0)/(2_f64.powf(24.0))
  }

  fn lng(&self) -> f64 {
    let base = (self.raw_lng() as f64 * 360.0)/(2_f64.powf(24.0));
    if base <= 180.0 {
      base
    } else {
      360.0 - base
    }
  }

  fn nic_string(&self) -> String {
    format!("NIC:                {}", self.raw_nic())
  }

  fn ns_vel(&self) -> String {
    match self.raw_ag() {
      0 => {
        let vel = self.raw_ns_vel();
        if vel == 0 {
          "Not available".to_owned()
        } else if vel == 1023 {
          "> 1021.5 kt".to_owned()
        } else {
          format!("{} kt", self.raw_ns_vel() - 1)
        }
      },
      1 => {
        let vel = self.raw_ns_vel();
        if vel == 0 {
          "Not available".to_owned()
        } else if vel == 1023 {
          "> 4086 kt".to_owned()
        } else {
          format!("{} kt", (self.raw_ns_vel() - 1) * 4)
        }
      },
      _ => "Unknown".to_owned()
    }
  }

  fn ew_vel(&self) -> String {
    match self.raw_ag() {
      0 => {
        let vel = self.raw_ew_vel();
        if vel == 0 {
          "Not available".to_owned()
        } else if vel == 1023 {
          "> 1021.5 kt".to_owned()
        } else {
          format!("{} kt", self.raw_ew_vel() - 1)
        }
      },
      1 => {
        let vel = self.raw_ew_vel();
        if vel == 0 {
          "Not available".to_owned()
        } else if vel == 1023 {
          "> 4086 kt".to_owned()
        } else {
          format!("{} kt", (self.raw_ew_vel() - 1) * 4)
        }
      },
      _ => "Unknown".to_owned()
    }
  }

  fn lat_string(&self) -> String {
    let lat = self.raw_lat();
    let ns = if lat <= 4194304 {
      "N"
    } else if lat >= 12582912 {
      "S"
    } else {
      "??"
    };
    format!("Latitude:           {:.4} {}", self.lat(), ns)
  }

  fn vv_string(&self) -> String {
    format!("{}{} ft/min (from geometric altitude)", if self.raw_vv_sign() { "-" } else { "" }, (self.raw_vv() - 1) * 64)
  }

  fn geo(&self) -> String {
    let lng = self.raw_lng();
    let ew = match lng {
      0 => "PM",
      1 ..= 8388607 => "E",
      8388608 => "EW",
      8388609 ..= 16777215 => "W",
      _ => "?"
    };
    format!(" {}\n Longitude:          {:.4} {}", self.lat_string(), self.lng(), ew)
  }

  fn tisb_site_id(&self) -> String {
    "?".into()
  }
}

bitfield!{
  pub struct ModeStatus(u128);
  impl Debug;
  pub raw_emitter_category, _ : 127, 112;
  pub raw_call_sign_345, _ : 111, 96;
  pub raw_call_sign_678, _ : 95, 80;
  pub raw_eps, _ : 79, 77;
  pub raw_mops, _ : 76, 74;
  pub raw_sil, _ : 73, 72;
  pub raw_transmit_mso, _ : 71, 66;
  pub raw_nacp, _ : 63, 60;
  pub raw_nacv, _ : 59, 57;
  pub raw_nicbaro, _ : 56;
  pub raw_cdti, _ : 55;
  pub raw_acas, _ : 54;
  pub raw_op_modes, _ : 53, 51;
  pub raw_true_msg, _ : 50;
  pub raw_csid, _ : 49;
}

impl ModeStatus {
  fn emitter_category(&self) -> String {
      let ec = (self.raw_emitter_category()/1600) % 40;
      format!("{}", ec)
  }

  fn callsign(&self) -> String {
    let base40: Vec<char>= "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ    ".chars().collect();
    let mut call = Vec::new();
    let ec = self.raw_emitter_category();
    call.push((ec/40)%40);
    call.push(ec%40);
    let block2 = self.raw_call_sign_345();
    call.push((block2 / 1600) % 40);
    call.push((block2 / 40) % 40);
    call.push(block2  % 40);
    let block3 = self.raw_call_sign_678();
    call.push((block3 / 1600) % 40);
    call.push((block3 / 40) % 40);
    call.push(block3  % 40);
    call.iter().map(|x| base40[*x as usize]).collect()
  }

  fn capabilities_string(&self) -> String {
    format!("Capabilities:       {} {}", if self.raw_cdti() { "CDTI" } else {""}, if self.raw_acas() { "ACAS" } else {""})
  }
}

impl fmt::Display for ModeStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " Emitter category:   {}\n Callsign:           {}\n Emergency status:   {}\n UAT version:       {}\n SIL:                {}\n Transmit MSO:       {}\n NACp:        {}\n NACv:            {}\n NICbaro:          {}\n {}\n {:?}", self.emitter_category(), self.callsign(), self.raw_eps(), self.raw_mops(), self.raw_sil(), self.raw_transmit_mso(), self.raw_nacp(), self.raw_nacv(), self.raw_nicbaro(), self.capabilities_string(), self)
    }
}

bitfield!{
  pub struct AuxStateVector(u64);
  impl Debug;
  pub raw_secondary_altitude, _ : 63, 52;
}

impl AuxStateVector {
  fn secondary_alt(&self) -> i32 {
    self.raw_secondary_altitude() as i32 * 25 - 1025
  }
}

impl fmt::Display for AuxStateVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, " Secondary Altitude:          {:?}", self.secondary_alt())
    }
}

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
