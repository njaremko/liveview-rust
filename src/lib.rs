#[macro_use]
extern crate serde;

mod live_view;
mod socket;

pub use live_view::*;
pub use socket::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
