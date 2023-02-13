use crate::Error;

pub trait Load {
    fn load(&self) -> Result<String, Error>;
}

pub trait Store {
    fn store(&self, conent: &str) -> Result<(), Error>;
}
