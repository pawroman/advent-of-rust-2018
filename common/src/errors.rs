pub use failure::{Error, Fail};


#[derive(Debug, Fail)]
#[fail(display = "Invalid number of arguments: {}", num_args)]
pub struct InvalidArguments {
    pub num_args: usize,
}
