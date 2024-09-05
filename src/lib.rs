#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("lib.hpp");

        type Binary;

        fn load_binary(data: &[u8]) -> Result<UniquePtr<Binary>>;
        fn add_library(&self, name: &str);
        fn has_library(&self, name: &str) -> bool;
        fn build(&self) -> Vec<u8>;

        fn test();
    }
}

pub use ffi::*;
