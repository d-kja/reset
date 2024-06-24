mod utils;
use std::process;

use utils::{core::core, error::process_exit};

fn main() {
    process::exit(match core() {
        Ok(_) => 0,
        Err(error) => {
            process_exit(error);
            1
        }
    })
}
