pub mod prelude {
    pub use serde::{Serialize, Deserialize};
    pub use ic_cdk_macros::*;
}

use prelude::*;

fn main() {}

#[query]
fn test() -> String {
    let s = String::from("Hello, world!");
    ic_cdk::println!("{}", s);
    s
}