use crate::ser::Deserializable;
use nom::IResult;
use nom::bytes::complete::take;
use crate::vendor::Vendor;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq)]
pub struct Opcode(u16);

impl Opcode {
    pub const fn build(ogf: u8, ocf: u16) -> Self {
        Self(
            ((ogf as u16 & 0b111111) << 10) as u16 | (ocf & 0b1111111111)
        )
    }

    pub fn opcode(&self) -> u16 {
        self.0
    }

    pub fn ogf(&self) -> u8 {
        // upper 6 bits
        ((self.0 >> 10) as u8) & 0b111111
    }

    pub fn ocf(&self) -> u16 {
        // lower 10 bits
        self.0 & 0b1111111111
    }
}

impl From<Opcode> for u16 {
    fn from(opcode: Opcode) -> Self {
        opcode.0
    }
}

impl From<u16> for Opcode {
    fn from(opcode: u16) -> Self {
        Self(opcode)
    }
}

impl From<(u8, u16)> for Opcode {
    fn from(components: (u8, u16)) -> Self {
        Self::build(components.0, components.1)
    }
}

impl Deserializable<Opcode> for Opcode {
    fn parse(i: &[u8]) -> IResult<&[u8], Opcode> {
        let (i, bytes) = take(2u8)(i)?;
        let opcode = u16::from_le_bytes([bytes[0], bytes[1]]);
        Ok((i,
            opcode.into()
        ))
    }
}

pub trait ReturnParameters: Deserializable<Self> + Sized {}

pub trait Command: Sized {
    const OPCODE: Opcode;
    type ReturnParameters: ReturnParameters;
    fn opcode(&self) -> Opcode {
        Self::OPCODE
    }
}

#[macro_export]
macro_rules! hci_commands {
    ( $($ogf:literal $group:ident => { $( $ocf:literal $command:ident => $return:ty),* $(,)? } ),*  $(,)? )  => {
        $(
            $(
                impl $crate::command::Command for $command {
                    const OPCODE: $crate::command::Opcode = $crate::command::Opcode::build($ogf, $ocf);
                    type ReturnParameters = $return;
                }
            )*
        )*
    };
}


#[cfg(test)]
mod tests {
    use crate::command::Opcode;

    #[test]
    pub fn bidirectional() {
        let o = Opcode(0x200A);
        assert_eq!(0x08, o.ogf());
        assert_eq!(0x0A, o.ocf());

        let o: Opcode = (0x08, 0x0A).into();
        assert_eq!(0x08, o.ogf());
        assert_eq!(0x0A, o.ocf());
        assert_eq!(0x200Au16, o.into());
    }
}