[BITS 32]

extern kmain
extern _align_stack

extern _set_sse
extern _set_avx
extern _set_fpu

%include "src/boot/early_gdt.asm"
%include "src/boot/early_paging.asm"

segment .text

global _init
_init:
	; block interrupts
	cli

.low_memory_area:
	; set temporary stack
	; set up the stack pointer for a temporary stack

	TRANSLATE_ADDR temporary_stack
	mov esp, eax
	mov ebp, esp

; INITIALIZE GDT
.init_gdt:
	TRANSLATE_ADDR gdt_start
	mov esi, eax

	mov edi, GDT_DESTINATION
	mov ecx, gdt_end - gdt_start

	cld
	rep movsb

	TRANSLATE_ADDR gdt_info
	lgdt [eax]

	; DS, ES, FS and GS ARE DATA SEGMENT REGISTER
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; SS IS STACK SEGMENT REGISTER
	mov ax, 0x18
	mov ss, ax

	; Paginate kernel in half high memory (do also identity mapping)

	; 0x00000000 -> 0x04000000 mapped to phy 0x00000000 -> 0x04000000

%define l0_virt_offset 0
%define l0_page_offset 0
%define l0_physic_addr 0
%define l0_len 64

	PAGINATE_ADDR identity_mapping, l0_virt_offset, l0_page_offset, l0_physic_addr, l0_len

	; 0xC0000000 -> 0xC4000000 mapped to phy 0x00000000 -> 0x04000000

%define l1_virt_offset 768
%define l1_page_offset 16
%define l1_physic_addr 0
%define l1_len 64

	PAGINATE_ADDR half_high_mem_mapping, l1_virt_offset, l1_page_offset, l1_physic_addr, l1_len

	; Active paging
	TRANSLATE_ADDR page_directory_alpha_area
	mov edx, eax
	mov cr3, eax 				; fill CR3 with physic mem pointer to page directory

	mov eax, cr0
	or eax, 0x80000001          ; enable Paging bit (PG). Protection bit must be also recall here
	mov cr0, eax

	; Jump to high memory, init code segment
	jmp 0x8: .high_memory_area

.high_memory_area:
	call .disable_cursor

	; set the base EIP on stack at 0x0, prevent infinite loop for backtrace

	; set up the main kernel stack
	;      stack frame 2             | stack frame 1             | stack frame 0
	; <--- (EBP EIP ARGx ... VARx ...) (EBP EIP ARGx ... VARx ...) ((ptr - 8) 0x0) | *** kernel_stack SYMBOL *** |
	;                                  <----     ESP EXPANSION   |    *ebp

	; This mechanism is for Panic handler. See details on 'panic.rs' file
	; dont worry about overflow for stack, the first push will be at [temporary_stack - 4], not in [temporary_stack]
	mov [kernel_stack - 4], dword 0x0
	mov esp, kernel_stack - 8
	mov ebp, esp

	call _set_sse
	call _set_avx
	call _set_fpu

	; EBX contain pointer to GRUB multiboot information (preserved register)
	push ebx
	push 4
	; kmain is called with EBX param
	push kmain

	call _align_stack

	add esp, 12

.idle:
	hlt
	jmp .idle

.disable_cursor:
	push eax
	push edx

	mov dx, 0x3D4
	; low cursor shape register
	mov al, 0xA
	out dx, al

	inc dx
	; bits 6-7 unused, bit 5 disables the cursor, bits 0-4 control the cursor shape
	mov al, 0x20
	out dx, al

	pop edx
	pop eax
	ret

%define VIRTUAL_LINEAR_FB_LOCATION 0xF0000000

; 0xF0000000 -> ... mapped to phy ??? -> ??? + LFB_SIZE
; hack for LFB allocation
; CAUTION: Usable only when high memory is initialized
GLOBAL _allocate_linear_frame_buffer
_allocate_linear_frame_buffer:
	push ebp
	mov ebp, esp

	push dword [ebp + 12]               ; len
	push dword [ebp + 8]                ; physical address
	push 32                             ; offset in page table
	push (1024 - 64)                    ; virt addr offset. eq 0xF0000000

	call _dynamic_map

	add esp, 16

	mov eax, VIRTUAL_LINEAR_FB_LOCATION

	pop ebp
	ret

segment .bss
align 16

; 1mo for the main kernel stack
resb 1 << 20
kernel_stack:

; 4kb for temporary stack
resb 1 << 12
temporary_stack:
