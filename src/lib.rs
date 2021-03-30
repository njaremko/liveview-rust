#[macro_use]
extern crate serde;

mod live_view;
mod socket;

pub use live_view::*;
pub use socket::*;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(askama::Template, Clone, Debug, Default)]
#[template(path = "base.html", escape = "none")]
pub struct BaseTemplate {
    pub title: String,
    pub head: String,
    pub body: String,
}

impl Template for BaseTemplate {
    fn render(&self) -> Result<String> {
        Ok(<Self as askama::Template>::render(&self)?)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
