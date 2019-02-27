[BITS 32]

; Declare constants used for creating a multiboot header.
%define ALIGN     (1 << 0)
%define MEMINFO   (1 << 1)
%define FLAGS     (ALIGN | MEMINFO)
%define MAGIC     0x1BADB002
%define CHECKSUM  - (MAGIC + FLAGS)

; Declare a header as in the Multiboot Standard.
; You don't need to understand all these details as it is just magic values that
; is documented in the multiboot standard. The bootloader will search for this
; magic sequence and recognize us as a multiboot kernel.
; The grub multiboot v1 header MUST be 12 bytes before the _start entry point

; LD script section:
;	.boot BLOCK(4K) : ALIGN(4K)
;	{
;		*(*.multiboot)
;	}
;
; OUTPUT elf file:
;	desassembly of section .boot:
;
;	00100000 <_start-0xc>:
;	100000:02 b0 ad 1b 03 00    add    0x31bad(%eax),%dh
;	100006:00 00                add    %al,(%eax)
;	100008:fb                   sti
;	100009:4f                   dec    %edi
;	10000a:52                   push   %edx
;	10000b:e4 e9                in     $0xe9,%al
;
;	0010000c <_start>:
;	10000c:e9 ef 0f 00 00       jmp    101000 <init>

section .multiboot
align 4
	dd MAGIC
	dd FLAGS
	dd CHECKSUM

extern _init
GLOBAL _start
_start:
	; usage of absolute jump to avoid nasty side effect in high memory model
	; normally, the instruction 'jmp _init' will be a relative jump, witch is safe, but we don't know ...
	; be very carefull about that instruction, IT MUST BE A RELATIVE JUMP !
	jmp _init
