use std::fmt;

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
