#![no_std]
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
pub extern "C" fn silly_print() {
    let silly_ints: [i32; 6] = [1, 2, 3, 4, 5, 6];

    let c_str: [i8; 9] = [115, 105, 108, 108, 121, 32, 37, 100, 0];
    let c_str: *const [i8; 9] = core::ptr::addr_of!(c_str);
    let c_str: *const c_char = c_str.cast::<i8>();

    for (i, num) in silly_ints.iter().enumerate() {
        unsafe {
            print_text_fmt_int(33, 33 + (i as i32 * 20), c_str, *num);
        }
    }
}
