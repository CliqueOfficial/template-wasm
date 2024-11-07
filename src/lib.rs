#![no_std]
extern crate alloc;

use clique_wasm_sdk::{console_log, get_param, set_output, Val};

#[inline]
fn fibonacci(n: u64) -> u64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n {
        let temp = a + b;
        a = b;
        b = temp;
    }
    b
}

#[no_mangle]
pub extern "C" fn evaluate() {
    match get_param("n") {
        Ok(v) => match v {
            Some(n) => {
                if let Some(n) = n.as_u64()  {
                    set_output("result", &Val::Uint64(fibonacci(n)));
                }
            },
            None => {},
        },
        Err(err) => {
            console_log(&err);
        }
    };
}