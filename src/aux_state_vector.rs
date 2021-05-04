use std::fmt;

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
