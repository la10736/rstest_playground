#![feature(proc_macro)]
extern crate hello_proc_macro;

use hello_proc_macro::hello;

#[hello]
fn other(_s: &str, _v: i32) {
}

fn main() {
    other("pippo", 42);
}
