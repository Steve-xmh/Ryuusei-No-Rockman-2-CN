//! 一些用于调试的函数
//!

use core::fmt::Write;

#[inline(always)]
pub fn nogba_breakpoint() {
    unsafe {
        core::arch::asm!("mov r11, r11", options(nomem, nostack));
    }
}

pub fn is_in_nogba() -> bool {
    unsafe {
        static mut IS_NOGBA: u32 = 0;
        if IS_NOGBA == 0 {
            let addr = 0x04FFFA00 as *mut u8;
            let mut data = [0u8; 16];
            // read 16 bytes
            crate::mem::copy(core::slice::from_raw_parts_mut(addr, 16), &mut data);
            IS_NOGBA = if data.starts_with(&[b'n', b'o', b'$', b'g', b'b', b'a']) {
                1
            } else {
                2
            };
        }
        IS_NOGBA == 1
    }
}

pub struct DebugWriter;

pub fn print_directly(msg: &str) {
    if is_in_nogba() {
        let nogba_string_out = 0x04FFFA1C as *mut u32;
        for c in msg.chars() {
            unsafe {
                *nogba_string_out = c as u32;
            }
        }
    }
}

impl Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        print_directly(s);
        Ok(())
    }
}

#[cfg(not(debug_assertions))]
pub fn _print(args: ::core::fmt::Arguments) {}
#[cfg(debug_assertions)]
pub fn _print(args: ::core::fmt::Arguments) {
    if !is_in_nogba() {
        return;
    }
    let mut writer = DebugWriter;
    writer.write_fmt(args).expect("can't write debug message");
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::nogba::_print(format_args!($($arg)*));
    };
}

#[cfg(with_println)]
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ($crate::print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::print!(
        concat!($fmt, "\n"), $($arg)*));
}

#[cfg(not(with_println))]
#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($fmt:expr) => ();
    ($fmt:expr, $($arg:tt)*) => ();
}

#[macro_export]
macro_rules! dbg {
    () => {
        $crate::println!("[{}:{}]", file!(), line!())
    };
    ($val:expr $(,)?) => {
        // Use of `match` here is intentional because it affects the lifetimes
        // of temporaries - https://stackoverflow.com/a/48732525/1063961
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($($crate::dbg!($val)),+,)
    };
}
