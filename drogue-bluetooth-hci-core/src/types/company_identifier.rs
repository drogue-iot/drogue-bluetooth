use heapless::{
    Vec,
    ArrayLength
};
use drogue_bluetooth_hci::ser::Serializable;

#[derive(Debug, Copy, Clone)]
pub struct CompanyIdentifier(u16);

impl From<&CompanyIdentifier> for u16 {
    fn from(id: &CompanyIdentifier) -> Self {
        id.0
    }
}

impl Serializable for CompanyIdentifier {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        u16::from(self).serialize(o)
    }
}

