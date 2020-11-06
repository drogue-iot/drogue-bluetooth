use nom::{IResult, AsBytes};
use nom::bytes::complete::take;
use heapless::{
    String,
    Vec,
    ArrayLength,
};
use nom::error::ErrorKind;
use crate::command::Command;

pub trait OneByteEnum : From<u8> {}

// ------------------------------------------------------------------------
// Deserialization
// ------------------------------------------------------------------------

pub trait Deserializable<T> {
    fn parse(i: &[u8]) -> IResult<&[u8], T>;
}

impl Deserializable<u8> for u8 {
    fn parse(i: &[u8]) -> IResult<&[u8], u8> {
        let (i, bytes) = take(1u8)(i)?;
        Ok(
            (i,
             bytes[0])
        )
    }
}

impl Deserializable<u16> for u16 {
    fn parse(i: &[u8]) -> IResult<&[u8], u16> {
        let (i, bytes) = take(2u8)(i)?;
        Ok(
            (i,
             u16::from_le_bytes([bytes[0], bytes[1]]))
        )
    }
}

impl<T: OneByteEnum> Deserializable<T> for T {
    fn parse(i: &[u8]) -> IResult<&[u8], T> {
        let (i, byte) = <u8 as Deserializable<_>>::parse(i)?;
        Ok(
            (i,
            T::from(byte) )
        )
    }
}

impl<N: ArrayLength<u8>> Deserializable<Vec<u8,N>> for Vec<u8, N> {
    fn parse(i: &[u8]) -> IResult<&[u8], Vec<u8, N>> {
        let mut len = i.len();
        // take at-most N bytes, but fewer is okay
        if len > N::to_usize() {
            len = N::to_usize()
        }
        let (i, bytes) = take(len)(i)?;
        let mut v = Vec::<u8,N>::new();
        v.extend_from_slice(bytes)
            .map_err(|_|
                nom::Err::Failure(
                    nom::error::Error::new(i, ErrorKind::Eof)
                )
            )?;

        Ok((i, v))
    }
}

// ------------------------------------------------------------------------
// Serialization
// ------------------------------------------------------------------------


pub trait Serializable {
    fn serialize<N:ArrayLength<u8>>(&self, o: &mut Vec<u8,N>) -> Result<(),()>;
}

impl Serializable for bool {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        if *self {
            o.push(0x01).map_err(|_|())
        } else {
            o.push(0x00).map_err(|_|())
        }
    }
}

impl Serializable for u16 {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        let bytes = self.to_le_bytes();
        o.extend_from_slice( &bytes ).map_err(|_|())
    }
}

impl<SN:ArrayLength<u8>> Serializable for String<SN> {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        o.extend_from_slice( self.as_bytes() ).map_err(|_|())
    }
}

impl<T: Serializable, VN:ArrayLength<T>> Serializable for Vec<T, VN> {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        for item in self {
            item.serialize(o)?;
        }
        Ok(())
    }
}

impl<VN:ArrayLength<u8>> Serializable for Vec<u8, VN> {
    fn serialize<N: ArrayLength<u8>>(&self, o: &mut Vec<u8, N>) -> Result<(), ()> {
        o.extend_from_slice( self.as_bytes() ).map_err(|_|())
    }
}

#[cfg(test)]
mod tests {

    use crate::ser::Deserializable;

    #[test]
    fn parsing_primitives() {
        assert_eq!( 42u8, <u8 as Deserializable<_>>::parse( &[ 42u8 ] ).unwrap().1);
        assert_eq!( 0xFC00, <u16 as Deserializable<_>>::parse( &[ 0x00, 0xFC ] ).unwrap().1);
    }

}
