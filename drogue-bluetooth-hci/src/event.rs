
/// The body of an HCI event.
pub trait Event {
    const CODE: u8;
}


/// A group of events.

/*
pub trait EventGroup {

}
 */

#[macro_export]
macro_rules! hci_events {
    ( $($code:literal => $name:ty),* $(,)?) => {
        $(
            impl $crate::event::Event for $name {
                const CODE: u8 = $code;
            }
        )*
    }
}

/*
#[macro_export]
macro_rules! hci_events {
    ( $name:ident : $( $code:literal $event:ident),* $(,)? ) => {
        pub enum $name {
            $(
                $event($event),
            )*
        }
    }

}
 */

/*
#[macro_export]
macro_rules! hci_events {
    ( $($code:expr => $name:ident { $($body:tt)* },)* ) => {
        #[derive(Debug)]
        pub enum Event {
            $(
                $name($name),
            )*
        }

        impl $crate::event::EventGroup for Event {

        }

        $(
            hci_events!( @EVENT $name, $($body)* );
        )*
    };

    ( @EVENT $name:ident, $( $field:ident: $ty:ty,)* ) => {
        #[derive(Debug)]
        pub struct $name {
            $(
                $field: $ty,
            )*
        }

        impl $crate::event::Event for $name {

        }

        impl $crate::ser::Deserializable<$name> for $name {
            fn parse(i: &[u8]) -> nom::IResult<&[u8], Self> {
                $(
                    let (i, $field) = <$ty as $crate::ser::Deserializable<_>>::parse(i)?;
                )*
                Ok(
                    (i,
                      Self {
                          $(
                              $field,
                          )*
                      })
                )

            }
        }
    };
}


 */