#![no_std]
#![feature(c_str_literals)]
#[panic_handler]
fn panic(_: &::core::panic::PanicInfo) -> ! {
    loop {}
}

use core::ffi::c_char;
use core::ffi::c_void;

extern "C" {
    fn print_text_fmt_int(x: i32, y: i32, str: *const c_char, n: i32) -> c_void;
}

#[no_mangle]
pub extern "C" fn silly_print(x: i32) {
    let silly_ints: [i32; 5] = [1, 2, 3, 4, 5];

    let silly = c"silly %d".as_ptr();
    let goofy = c"goofy %d".as_ptr();

    for (i, num) in silly_ints.iter().enumerate() {
        unsafe {
            print_text_fmt_int(x, 33 + (i as i32 * 20), silly, *num);
        }
    }

    for (i, num) in silly_ints.iter().enumerate() {
        unsafe {
            print_text_fmt_int(x, 133 + (i as i32 * 20), goofy, (x * *num));
        }
    }

    let number: i32 = 5;
    let number: i32 = number * 5;
    let number: i32 = number - 3;
    let number: i32 = number + number;
    let number: i32 = number / 2;

    unsafe {
            print_text_fmt_int(x, 0, c"%d".as_ptr(), number);
    }
}
