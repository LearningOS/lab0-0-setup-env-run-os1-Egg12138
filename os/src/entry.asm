
	.section .text.entry  	# named `.text.entry` is just for convinient.
	.globl _start			# pass the control privilige to rust prog
_start:
	la sp, boot_stack_top
	call rust_main			# we have to ensure the rust_main is not mangled

	.section .bss.stack
	.globl	boot_stack
boot_stack:					# bottom of the stack
	.space 4096 * 16	    # length of stack: 4K * 16 = 64KiB
	.globl boot_stack_top	# globl marked this section is a global symbol

boot_stack_top: 			# top of the stack

