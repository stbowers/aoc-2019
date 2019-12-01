#[cfg(feature = "file_utils")]
pub mod file_utils;

#[cfg(feature = "text_utils")]
pub mod text_utils;

pub mod prelude;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
