	.file	"bar.c"
	.section .rdata,"dr"
LC0:
	.ascii "bar\0"
	.text
	.globl	_bar
	.def	_bar;	.scl	2;	.type	32;	.endef
_bar:
	pushl	%ebp
	movl	%esp, %ebp
	subl	$24, %esp
	movl	$LC0, (%esp)
	call	_puts
	nop
	leave
	ret
	.ident	"GCC: (tdm-1) 5.1.0"
	.def	_puts;	.scl	2;	.type	32;	.endef
