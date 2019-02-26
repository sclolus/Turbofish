; see https://wiki.osdev.org/Paging

[BITS 32]

%include "src/boot/virtual_offset.asm"

segment .text

extern kmain
extern _init_gdt
extern _align_stack

extern _set_sse
extern _set_avx
extern _set_fpu

; Some usefull paging const
%define READ_WRITE (1 << 1)
%define PRESENT (1 << 0)

; only the first 20 bits is signifiant for pages
%define PAGE_MASK 0xfffff000
%define PAGE_SIZE 4096
%define PAGE_TABLE_PER_DIRECTORY 1024

; 0x00000000 -> 0x08000000 mapped to phy 0x00000000 -> 0x08000000
; 0xC0000000 -> 0xC8000000 mapped to phy 0x00000000 -> 0x08000000
; 0xE0000000 -> 0xFFFFFFFF mapped to phy 0xE0000000 -> 0xFFFFFFFF (LFB)

global _init
_init:
	; block interrupts
	cli

	; STORING KERNEL IN HIGH MEMORY by setting a early pagination
	; -----------------------------------------------------------------------
	; FIRST STEP => IDENTITY MAPPING OF THE FIRST 128 MO: MAPPING of 0x0 => 0x8000000
	; create the first heap of page directory for mapping of 128mo Kernel Size
	; a page directory can contains 1024 pages entries witch allocated at most 4mo
	mov edi, VIRT2PHY_ADDR(page_directory_alpha_area)

	; pointer to the first set of page table
	mov edx, VIRT2PHY_ADDR(page_table_alpha_area)

	; prepare to assign 32 * 4 mo of memory -> 128 mo
	mov ecx, 32
.l0_a:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	loop .l0_a

	; created for IDENTITY MAPPING of the first 128 mo
	mov edi, VIRT2PHY_ADDR(page_table_alpha_area)
	xor edx, edx
.l0_b:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	cmp edx, (1 << 20) * 128 ; limit at 128 mo
	jne .l0_b

	; --------------------------------------------------------------------------
	; SECOND STEP => MAP VIRTUAL HIGH MEMORY IN 0xC0000000 to PHYSICAL 0xC8000000
	mov edi, VIRT2PHY_ADDR(page_directory_alpha_area) + 768 * 4                        ; high memory, correspond to 0xC0000000 in virtual space
	mov edx, VIRT2PHY_ADDR(page_table_alpha_area) + PAGE_TABLE_PER_DIRECTORY * 32 * 4  ; next 1024 * 32 pages (SHL by 2)

	; TODO need to calc the kernel size ! count of 4mo block (32 * 4mo = 128 mo)
	mov ecx, 32
.l1_a:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	loop .l1_a

	; create the corresponding pages for tranlation from high virt memory to low phy memory
	mov edi, VIRT2PHY_ADDR(page_table_alpha_area) + PAGE_TABLE_PER_DIRECTORY * 32 * 4	; 1024 * 32 pages are currently allocated
	xor edx, edx
.l1_b:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	cmp edx, (1 << 20) * 128 ; limit -> Relative to Kernel size (1mo + (1mo * 128) = 0x0 -> 0x8000000 range 128 mo)
	jne .l1_b

	; --------------------------------------------------------------------------
	; THIRD STEP => (LINEAR FRAME BUFFER) MAP VIRTUAL HIGH MEMORY 0xE0000000 to PHYSICAL 0xE0000000
	mov edi, VIRT2PHY_ADDR(page_directory_alpha_area) + (1024 - 128) * 4                     ; high memory, correspond to 0xE0000000 in virtual space
	mov edx, VIRT2PHY_ADDR(page_table_alpha_area) + (PAGE_TABLE_PER_DIRECTORY * 32 * 4) * 2  ; next 1024 * 32 pages (SHL by 2)

	; count of 4mo block (128 * 4mo = 512 mo)
	mov ecx, 128
.l2_a:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	loop .l2_a

	; create the corresponding pages for tranlation from linear framebuffer to low phy memory
	mov edi, VIRT2PHY_ADDR(page_table_alpha_area) + (PAGE_TABLE_PER_DIRECTORY * 32 * 4) * 2	 ; 1024 * 32 * 4 * 2 pages are currently allocated
	mov edx, 0xE0000000
.l2_b:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	cmp edx, 0x0 ; limit = 0xe0000000 -> 0x00000000 range 512 mo. Using of overflow
	jne .l2_b

	; ACTIVATE PAGING
	; ----------------------------------------------
	mov eax, VIRT2PHY_ADDR(page_directory_alpha_area)
	mov edx, eax
	mov cr3, eax 				; fill CR3 with physic mem pointer to page directory

	mov eax, cr0
	or eax, 0x80000001          ; enable Paging bit (PG). Protection bit must be also recall here
	mov cr0, eax

	; FINALLY, JUMP TO HIGH MEMORY
	; ----------------------------
	lea eax, [.high_jump]
	jmp eax
.high_jump:
	; set up the stack pointer for a temporary stack
	; dont worry about overflow for stack, the first push will be at [temporary_stack - 4], not in [temporary_stack]
	mov esp, temporary_stack
	mov ebp, esp

	call .disable_cursor

	call _init_gdt

	; SS IS STACK SEGMENT REGISTER
	mov ax, 0x18
	mov ss, ax

	; set the base EIP on stack at 0x0, prevent infinite loop for backtrace

	; set up the main kernel stack
	;      stack frame 2             | stack frame 1             | stack frame 0
	; <--- (EBP EIP ARGx ... VARx ...) (EBP EIP ARGx ... VARx ...) ((ptr - 8) 0x0) | *** kernel_stack SYMBOL *** |
	;                                  <----     ESP EXPANSION   |    *ebp

	; This mechanism is for Panic handler. See details on 'panic.rs' file
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

segment .bss
align 4096

; 1mo reserved for alpha pages tables Can allocate 1 go: Kernel_low: 128. Kernel_High: 128. FrameBuffer 512.
; KERNEL SIZE CANNOT EXCEED 128 MO !
page_table_alpha_area:
resb 1 << 20

; 1mo for the main kernel stack
resb 1 << 20
kernel_stack:

; 4kb reserved for alpha pages table directory: Can allocate 4 go
page_directory_alpha_area:
resb 1 << 12

; 4kb for temporary stack
resb 1 << 12
temporary_stack:
