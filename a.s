	.att_syntax
	.align 8
factorial:
	movq %rdi, %r10
	movq $1, %rcx
.EE_13:
	cmpq $0, %rsi
	jg .EE_14
	movq %rcx, %rax
	retq
.EE_14:
	imulq %rsi, %rcx
	subq $1, %rsi
	jmp .EE_13

	.align 8
	.globl main
main:
	pushq %rbx
	pushq %r15
	pushq %r12
	movq $32768, %rdi
	movq $32768, %rsi
	callq __gc_initialize
	movq __gc_rootstack_begin(%rip), %r15
	movq %rdi, %rbx
	callq read_int
	movq %rax, %r12
	movq __gc_free_ptr(%rip), %rcx
	addq $16, %rcx
	cmpq __gc_fromspace_end(%rip), %rcx
	jl .EE_17
	movq $16, %rdi
	callq __gc_collect
.EE_17:
	movq __gc_free_ptr(%rip), %r11
	addq $16, __gc_free_ptr(%rip)
	movq $2, 0(%r11)
	movq %r11, %rbx
	movq %rbx, %r11
	leaq factorial(%rip), %rax
	movq %rax, 8(%r11)
	movq %rbx, %rax
	movq 8(%rax), %rcx
	movq %rbx, %rdi
	movq %r12, %rsi
	movq %rcx, %rax
	callq *%rax
	movq %rax, %rdi
	callq print_int
	popq %r12
	popq %r15
	popq %rbx
	retq

	.section .note.GNU-stack,"",@progbits
