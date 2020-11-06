use crate::ser::{Deserializable, Serializable};
use crate::command::{Command, ReturnParameters};
use core::convert::TryFrom;
use core::any::Any;

pub trait Vendor : Sized {
    type SupportedCommand: Serializable;
    type Event: Deserializable<Self::Event>;
}

#[macro_export]
macro_rules! vendor {
    {
        [$name:ident] $( $block_name:ident => { $($body:tt)* }),* $(,)?
    } => {
        $(
            $crate::vendor!( @ $block_name => { $($body)* });
        )*

        pub struct $name {

        }

        impl $crate::vendor::Vendor for $name{
            type SupportedCommand = SupportedCommands;
            type Event = SupportedEvents;
        }
    };

    ( @ commands => { $($command:ident),* $(,)? }) => {
        pub enum SupportedCommands {
            $(
                $command($command),
            )*
        }

        impl $crate::ser::Serializable for SupportedCommands {
            fn serialize<N: $crate::heapless::ArrayLength<u8>>(&self, o: &mut $crate::heapless::Vec<u8, N>) -> Result<(),()> {
                match self {
                    $(
                        Self::$command(val) => { val.serialize(o)?; }
                    )*
                }
                Ok(())
            }
        }

        $(
            impl From<$command> for SupportedCommands {
                fn from(command: $command) -> Self {
                    Self::$command(command)
                }
            }
        )*
    };

    ( @ events => { $($event:ident),* $(,)? }) => {
        pub enum SupportedEvents {
            $(
                $event($event),
            )*
            UnhandledEvent(u8),
        }

        impl $crate::ser::Deserializable<SupportedEvents> for SupportedEvents {
            fn parse(i: &[u8]) -> $crate::nom::IResult<&[u8], Self> {
                let (i, code)    = $crate::nom::bytes::complete::take(1u8)(i)?;
                let (i, len)     = $crate::nom::bytes::complete::take(1u8)(i)?;
                let (i, payload) = $crate::nom::bytes::complete::take(len[0])(i)?;
                match code[0] {
                    $(
                        <$event as $crate::event::Event>::CODE => {
                            <$event as $crate::ser::Deserializable<_>>::parse(payload).map(|(i,v)| (i,Self::$event(v)) )
                        },
                    )*
                    _ => {
                        Ok((i,Self::UnhandledEvent(code[0])))
                    }
                }
            }
        }
    };


}