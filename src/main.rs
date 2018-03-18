#![feature(proc_macro)]
extern crate hello_proc_macro;

use hello_proc_macro::hello;

#[hello]
fn other(_s: &str, _v: i32) {
}

fn main() {
    other("pippo", 42);
}

// You should implement rstest procedural macro to run `some_test` as cargo test by
// using `fixture()` and `fix_string()` fixtures
//

//extern crate rstest;
//
//use rstest::rstest;
//
//fn fixture() -> &'static str {
//    "42"
//}
//
//fn fix_string() -> String {
//    "String".to_string()
//}
//
//#[rstest]
//fn some_test(fixture: &str, fix_string: String) {
//    assert_eq!(fixture, "42");
//    assert_eq!(fix_string, "String".to_string());
//}
