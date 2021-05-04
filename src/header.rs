use std::fmt;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

bitfield!{
  pub struct UatHeader(u32);
  impl Debug;
  pub code, _ : 31, 27;
  pub qualifier, _ : 26, 24;
  pub address, _ : 23, 0;
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
