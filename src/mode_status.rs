use std::fmt;

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
