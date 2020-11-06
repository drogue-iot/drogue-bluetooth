use drogue_bluetooth_hci::ser::OneByteEnum;
//use crate::ser::OneByteEnum;

macro_rules! status_impl {
    ( $($value:expr => $name:ident,)* ) => {
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum Status {
          $(
            $name,
          )*
          UnknownStatus(u8),
        }

        impl From<Status> for u8 {
            fn from(val: Status) -> Self {
                match val {
                    $( Status::$name => $value, )*
                    Status::UnknownStatus(v) => v,
                }
            }
        }

        impl From<u8> for Status {
            fn from(val: u8) -> Self {
                match val {
                    $( $value => Status::$name, )*
                    _ => Status::UnknownStatus(val),
                }
            }
        }
    };
}

status_impl!{
    0x00 => OK,
    0x01 => UnknownHciCommand,
    0x02 => UnknownConnectionIdentifier,
    0x03 => HardwareFailure,
    0x04 => PageTimeout,
    0x05 => AuthenticationFailure,
    0x06 => PinOrKeyMissing,
    0x07 => MemoryCapacityExceeded,
    0x08 => ConnectionTimeout,
}

impl OneByteEnum for Status {

}

#[cfg(test)]
mod tests {
    use crate::types::status::Status::{UnknownConnectionIdentifier, UnknownStatus};

    #[test]
    fn bidirectional_conversion() {
        assert_eq!( UnknownConnectionIdentifier, 0x02.into());
        assert_eq!( 0x02u8, UnknownConnectionIdentifier.into());

        assert_eq!( UnknownStatus(0xFE), 0xFE.into());
        assert_eq!( 0xFEu8, UnknownStatus(0xFE).into());
    }
}
