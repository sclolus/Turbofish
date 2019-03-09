[BITS 32]

; Initialisation methods prefixed by alt_
extern alt_clear_screen
extern alt_gdt_new
extern alt_get_device_mem_map
extern alt_init_paging

%define MULTIBOOT_INFOS_LEN 128

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
; .boot BLOCK(4K) : ALIGN(4K)
;   {
;       *(*.multiboot)
;   }
;
; OUTPUT elf file:
; desassembly of section .boot:
;
; 00100000 <_start-0xc>:
; 100000:02 b0 ad 1b 03 00    add    0x31bad(%eax),%dh
; 100006:00 00                add    %al,(%eax)
; 100008:fb                   sti
; 100009:4f                   dec    %edi
; 10000a:52                   push   %edx
; 10000b:e4 e9                in     $0xe9,%al
;
; 0010000c <_start>:
; 10000c:e9 ef 0f 00 00       jmp    101000 <init>

extern _init_kernel
segment .multiboot
align 4
	dd MAGIC
	dd FLAGS
	dd CHECKSUM

GLOBAL _start
_start:
	cli
	jmp _init

segment .init
_init:
	; Set the first stack
	; Dont worry about overflow for stack, the first push will be at [temporary_stack - 4], not in [temporary_stack]
	mov [temporary_stack - 4], dword 0x0
	mov esp, temporary_stack - 8
	mov ebp, esp

	; Store the multiboot info structure pointed by EBX (may be erased later)
	mov edi, multiboot_infos
	mov esi, ebx
	mov ecx, MULTIBOOT_INFOS_LEN
	cld
	rep movsb

	; Set a early GDT
	; reserve 8 bytes for structure return
	sub esp, 8
	; this function return a structure on stack and pop four bytes
	call alt_gdt_new
	lgdt [eax]
	add esp, 4

	; Set the Data descriptor segments
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; Set the stack segment
	mov ax, 0x18
	mov ss, ax

	; Set the code segment
	jmp 0x8: .set_protected_cs
.set_protected_cs:

	; TODO SET A EARLY IDT HERE (IT COULD BE A GOOD IDEA !)

	call alt_clear_screen

	; Get device map in memory and push a pointer to a generated structure
	call alt_get_device_mem_map
	push eax

	; Set the MMU to prepare switching to high half memory
	call alt_init_paging

	; Push the grub multiboot header
	push multiboot_infos

	; Call _init_kernel stored in high memory
	call _init_kernel

	; A long jump can give a adrenaline boost, i dont understand why ...
	; call 0x8:_init_kernel

align 16
; 4ko for a temporary stack
times 1 << 12 db 0
temporary_stack:

multiboot_infos:
times 128 db 0xff
