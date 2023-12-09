use std::env;

pub fn setup(){
    let level = String::from("info");
    env::set_var("RUST_LOG", level);
}
