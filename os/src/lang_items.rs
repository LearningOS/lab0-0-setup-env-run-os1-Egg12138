use core::panic::PanicInfo;
use crate::sbi::shut_down;
use crate::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> !{

	if let Some(location) = info.location() {
		println!("Panicked ({}):{}=> \n{}",
	location.file(),
	location.line(),
	info.message().unwrap()
		);
		
	} else {
		println!("Panicked {}", info.message().unwrap());
	}
	shut_down()
}