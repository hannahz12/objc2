	.section	__TEXT,__text,regular,pure_instructions
	.syntax unified
	.globl	_get_sel
	.p2align	2
	.code	32
_get_sel:
	movw	r0, :lower16:(__MergedGlobals-(LPC0_0+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC0_0+8))
LPC0_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_same_sel
	.p2align	2
	.code	32
_get_same_sel:
	movw	r0, :lower16:(__MergedGlobals-(LPC1_0+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC1_0+8))
LPC1_0:
	add	r0, pc, r0
	ldr	r0, [r0, #4]
	bx	lr

	.globl	_get_common
	.p2align	2
	.code	32
_get_common:
	movw	r0, :lower16:(__ZN15test_static_sel10get_common5do_it3REF17h46c060c4347a38e4E-(LPC2_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel10get_common5do_it3REF17h46c060c4347a38e4E-(LPC2_0+8))
LPC2_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_get_different_sel
	.p2align	2
	.code	32
_get_different_sel:
	movw	r0, :lower16:(__MergedGlobals-(LPC3_0+8))
	movt	r0, :upper16:(__MergedGlobals-(LPC3_0+8))
LPC3_0:
	add	r0, pc, r0
	ldr	r0, [r0, #8]
	bx	lr

	.globl	_unused_sel
	.p2align	2
	.code	32
_unused_sel:
	movw	r0, :lower16:(__ZN15test_static_sel10unused_sel5do_it3REF17h7b1fdae0240cf991E-(LPC4_0+8))
	movt	r0, :upper16:(__ZN15test_static_sel10unused_sel5do_it3REF17h7b1fdae0240cf991E-(LPC4_0+8))
LPC4_0:
	ldr	r0, [pc, r0]
	bx	lr

	.globl	_use_fns
	.p2align	2
	.code	32
_use_fns:
	movw	r1, :lower16:(__MergedGlobals-(LPC5_0+8))
	movt	r1, :upper16:(__MergedGlobals-(LPC5_0+8))
	movw	r2, :lower16:(__MergedGlobals-(LPC5_1+8))
	movt	r2, :upper16:(__MergedGlobals-(LPC5_1+8))
LPC5_0:
	add	r1, pc, r1
LPC5_1:
	ldr	r2, [pc, r2]
	ldr	r3, [r1, #4]
	ldr	r9, [r1, #8]
	ldr	r1, [r1, #12]
	stm	r0, {r2, r3, r9}
	str	r1, [r0, #12]
	bx	lr

	.globl	_use_same_twice
	.p2align	2
	.code	32
_use_same_twice:
	movw	r2, :lower16:(__MergedGlobals-(LPC6_0+8))
	movt	r2, :upper16:(__MergedGlobals-(LPC6_0+8))
LPC6_0:
	ldr	r2, [pc, r2]
	movw	r3, :lower16:(__MergedGlobals-(LPC6_1+8))
	movt	r3, :upper16:(__MergedGlobals-(LPC6_1+8))
LPC6_1:
	ldr	r3, [pc, r3]
	strd	r2, r3, [r0]
	bx	lr

	.globl	_use_in_loop
	.p2align	2
	.code	32
_use_in_loop:
	cmp	r0, #0
	bxeq	lr
LBB7_1:
	movw	r1, :lower16:(__ZN15test_static_sel11use_in_loop5do_it3REF17h5c831b42249b63fbE-(LPC7_0+8))
	subs	r0, r0, #1
	movt	r1, :upper16:(__ZN15test_static_sel11use_in_loop5do_it3REF17h5c831b42249b63fbE-(LPC7_0+8))
LPC7_0:
	ldr	r1, [pc, r1]
	bne	LBB7_1
	bx	lr

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7get_sel5do_it4NAME17h2dffbace724e41c8E:
	.asciz	"simple"

__ZN15test_static_sel12get_same_sel5do_it4NAME17h505a1d4eeb4946fbE:
	.asciz	"simple"

__ZN15test_static_sel10get_common5do_it4NAME17h3d2d64c9c4874f50E:
	.asciz	"alloc"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel10get_common5do_it3REF17h46c060c4347a38e4E:
	.long	__ZN15test_static_sel10get_common5do_it4NAME17h3d2d64c9c4874f50E

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel17get_different_sel5do_it4NAME17hbbe758ed5ef72d9eE:
	.asciz	"i:am:different:"

__ZN15test_static_sel10unused_sel5do_it4NAME17h4683c999360858fbE:
	.asciz	"unused"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel10unused_sel5do_it3REF17h7b1fdae0240cf991E:
	.long	__ZN15test_static_sel10unused_sel5do_it4NAME17h4683c999360858fbE

	.section	__TEXT,__objc_methname,cstring_literals
__ZN15test_static_sel7use_fns5do_it4NAME17hfc4272ae24fa0543E:
	.asciz	"fourthSel"

__ZN15test_static_sel11use_in_loop5do_it4NAME17hc35160b73d726a00E:
	.asciz	"loopedSelector"

	.section	__DATA,__objc_selrefs,literal_pointers
	.p2align	2
__ZN15test_static_sel11use_in_loop5do_it3REF17h5c831b42249b63fbE:
	.long	__ZN15test_static_sel11use_in_loop5do_it4NAME17hc35160b73d726a00E

	.p2align	2
__MergedGlobals:
	.long	__ZN15test_static_sel7get_sel5do_it4NAME17h2dffbace724e41c8E
	.long	__ZN15test_static_sel12get_same_sel5do_it4NAME17h505a1d4eeb4946fbE
	.long	__ZN15test_static_sel17get_different_sel5do_it4NAME17hbbe758ed5ef72d9eE
	.long	__ZN15test_static_sel7use_fns5do_it4NAME17hfc4272ae24fa0543E

.subsections_via_symbols
