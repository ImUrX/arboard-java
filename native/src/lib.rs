#![allow(non_snake_case)]

mod clipboard;
mod imagedata;

pub use clipboard::*;
pub use imagedata::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
