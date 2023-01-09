use super::*;
use bindings::*;
use core::marker::PhantomData;

/// A type representing an agile reference to a COM/WinRT object.
#[repr(transparent)]
#[derive(Clone, PartialEq, Eq)]
pub struct AgileReference<T>(IAgileReference, PhantomData<T>);

impl<T: Interface> AgileReference<T> {
    /// Creates an agile reference to the object.
    pub fn new<'a>(object: &'a T) -> Result<Self>
    where
        &'a T: IntoParam<'a, IUnknown>,
    {
        unsafe { RoGetAgileReference(AGILEREFERENCE_DEFAULT, &T::IID, object).map(|reference| Self(reference, Default::default())) }
    }

    /// Retrieves a proxy to the target of the `AgileReference` object that may safely be used within any thread context in which get is called.
    pub fn resolve(&self) -> Result<T> {
        unsafe { self.0.Resolve() }
    }
}

unsafe impl<T: Interface> Send for AgileReference<T> {}
unsafe impl<T: Interface> Sync for AgileReference<T> {}
