use crate::vendor::Vendor;

pub enum Packet<V: Vendor> {
    Command(V::SupportedCommand),
    Event(V::Event),
}

