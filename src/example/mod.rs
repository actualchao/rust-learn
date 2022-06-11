mod string;
use string::string;
mod vector_hello;
use vector_hello::test_vector;
pub mod aa;
pub use aa::bb::test as aatest;
mod hash_map;
use hash_map::map as map_example;
mod file;
mod panic;


pub fn exec() {
    aatest();
    string();
    file::file();
    map_example();
    test_vector::test();
    panic::test_pinic();
}
