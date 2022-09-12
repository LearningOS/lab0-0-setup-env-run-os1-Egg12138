#![feature(core_panic)]
#![allow(unused)] 
const SBI_SET_TIMER: usize = 0;
const SBI_CONSOLE_PUTCHAR: usize = 1;
const SBI_CONSOLE_GETCHAR: usize = 2;
const SBI_CLEAR_IPI: usize = 3;
const SBI_SEND_IPI: usize = 4;
const SBI_REMOTE_FENCE_I: usize = 5;
const SBI_REMOTE_SFENCE_VMA: usize = 6;
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;
const SBI_SHUTDOWN: usize = 8;

use core::{arch::asm, panicking::panic};


/// `sbi_call()`助于实现内核和RustSBI的通信。
///  `which` 表示请求 RustSBI 的服务的类型（RustSBI 可以提供多种不同类型的服务）， 
/// `arg0` ~ `arg2` 表示传递给 RustSBI 的 3 个参数，而 RustSBI 在将请求处理完毕后，会给内核一个返回值，这个返回值也会被 sbi_call 函数返回。
#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {

	let mut ret;
	unsafe {

		asm!(
			"ecall",
			inlateout("a0") arg0 => ret, //x10
			in("a7") which,    // x17
			in("a1") arg1,      // x11
			in("a2") arg2, 		// x12
		);
	}
	ret
}


pub fn console_putchar(c: usize) {
	sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}


pub fn shut_down() -> !{

	sbi_call(SBI_SHUTDOWN, 0, 0, 0);
	panic!("Shutdown...");
}