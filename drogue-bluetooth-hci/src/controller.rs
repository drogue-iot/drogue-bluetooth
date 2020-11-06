use crate::vendor::Vendor;
use core::marker::PhantomData;

pub struct Controller<V:Vendor> {
    _vendor: PhantomData<V>,

}

impl<V:Vendor> Controller<V> {

}