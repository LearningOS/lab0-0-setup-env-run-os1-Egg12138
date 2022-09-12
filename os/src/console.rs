#![allow(dead_code)]
use crate::sbi::console_putchar;
use core::fmt::{self, Write};

struct Stdout;


impl Write for Stdout {
	fn write_str(&mut self, text: &str) -> fmt::Result {

		for c in text.chars() {
			console_putchar(c as usize);
		}
		Ok(())
	}
}


pub fn print(args: fmt::Arguments) {
	Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {

	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::print(format_args!($fmt $(, $($arg)+)?));
	}

}

#[macro_export]
macro_rules! println {
	() => {
		$crate::console::print("\n");
	};

	($fmt: literal $(, $($arg: tt)+)?) => {
		$crate::console::print(format_args_nl!($fmt $(, $($arg)+)?));
	};
}
