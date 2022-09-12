#![no_main]
#![allow(unused_imports)]
#![feature(core_panic, format_args_nl, panic_info_message)]
#![no_std]
mod lang_items;
mod sbi;
mod console;

use sbi::console_putchar;
use core::arch::global_asm;
use core::fmt::{self, Write};

global_asm!(include_str!("entry.asm"));

fn _start() {
	loop{}
}



#[no_mangle]
fn rust_main() -> ! {
	clear_bss();
	println!("Hello World!");
	panic!("Shutdown");
	}


/// clear_bss() does init jobs.
/// clear_bss call extern C functions:
/// * sbss()
/// * ebss()
/// gens a literator: from sbss(as usize) -> ebss(as usize)
/// and travels the iterable
/// for each item, we use closure:  
///  in the closure, we use unsafe rust, where the syntax is like C's pointer and each a is an address(raw pointer) 
///  indicating a mutable u8 and then we write_volatile the content of a (to 0).
/// finally, we init the bss section with zeros.
fn clear_bss() {
	extern  "C" {
		fn sbss();
		fn ebss();
	}
	(sbss as usize..ebss as usize ).for_each(|a| {
		unsafe {
			(a as *mut u8).write_volatile(0);
		}
	})


}