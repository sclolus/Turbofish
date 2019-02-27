; see https://wiki.osdev.org/Paging

[BITS 32]

segment .text

extern kmain
extern _init_gdt
extern _align_stack

extern _set_sse
extern _set_avx
extern _set_fpu

; early paging
%define virtual_offset 0xC0000000
%define VIRT2PHY_ADDR(x) (x - virtual_offset)

; Some usefull paging const
%define READ_WRITE (1 << 1)
%define PRESENT (1 << 0)

; only the first 20 bits is signifiant for pages
%define PAGE_MASK 0xfffff000
%define PAGE_SIZE 4096
%define PAGE_TABLE_PER_DIRECTORY 1024

; 0x00000000 -> 0x04000000 mapped to phy 0x00000000 -> 0x04000000
; 0xC0000000 -> 0xC4000000 mapped to phy 0x00000000 -> 0x04000000
; 0xF0000000 -> ... mapped to phy ??? -> ??? + LFB_SIZE

global _init
_init:
	; block interrupts
	cli


	; STORING KERNEL IN HIGH MEMORY by setting a early pagination
	; -----------------------------------------------------------------------
	; FIRST STEP => IDENTITY MAPPING OF THE FIRST 64 MO: MAPPING of 0x0 => 0x4000000
	; create the first heap of page directory for mapping of 64 mo Kernel Size
	; a page directory can contains 1024 pages entries witch allocated at most 4mo
	mov edi, VIRT2PHY_ADDR(page_directory_alpha_area)

	; pointer to the first set of page table
	mov edx, VIRT2PHY_ADDR(page_table_alpha_area)

	; prepare to assign 16 * 4 mo of memory -> 64 mo
	mov ecx, 16
.l0_a:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	loop .l0_a

	; created for IDENTITY MAPPING of the first 64 mo
	mov edi, VIRT2PHY_ADDR(page_table_alpha_area)
	xor edx, edx
.l0_b:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	cmp edx, (1 << 20) * 64 ; limit at 64 mo
	jne .l0_b


	; --------------------------------------------------------------------------
	; SECOND STEP => MAP VIRTUAL HIGH MEMORY IN 0xC0000000 to PHYSICAL 0xC4000000
	mov edi, VIRT2PHY_ADDR(page_directory_alpha_area) + 768 * 4                        ; high memory, correspond to 0xC0000000 in virtual space
	mov edx, VIRT2PHY_ADDR(page_table_alpha_area) + PAGE_TABLE_PER_DIRECTORY * 16 * 4  ; next 1024 * 16 pages (SHL by 2)

	; TODO need to calc the kernel size ! count of 4mo block (16 * 4mo = 64 mo)
	mov ecx, 16
.l1_a:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	loop .l1_a

	; create the corresponding pages for tranlation from high virt memory to low phy memory
	mov edi, VIRT2PHY_ADDR(page_table_alpha_area) + PAGE_TABLE_PER_DIRECTORY * 16 * 4	; 1024 * 16 pages are currently allocated
	xor edx, edx
.l1_b:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	cmp edx, (1 << 20) * 64 ; limit -> Relative to Kernel size (1mo + (1mo * 64) = 0x0 -> 0x4000000 range 64 mo)
	jne .l1_b


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

%define VIRTUAL_LINEAR_FB_LOCATION 0xF0000000

; hack for LFB allocation
GLOBAL _allocate_linear_frame_buffer
_allocate_linear_frame_buffer:
	push ebp
	mov ebp, esp

	push edi
	push ecx
	push edx

	mov edi, VIRT2PHY_ADDR(page_directory_alpha_area) + (1024 - 64) * 4                 ; placement at 0xf0000000
	mov edx, VIRT2PHY_ADDR(page_table_alpha_area) + PAGE_TABLE_PER_DIRECTORY * 32 * 4   ; next pages

	mov ecx, [ebp + 12]          ; len
	mov eax, ecx
	shr ecx, 12                  ; len = len / 4096
	and eax, 0xfff               ; if reste. len + 1
	cmp eax, 0
	je .l2_a
	add ecx, 1

.l2_a:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	loop .l2_a

	; create the corresponding pages for tranlation from high virt memory to low phy memory
	mov edi, VIRT2PHY_ADDR(page_table_alpha_area) + PAGE_TABLE_PER_DIRECTORY * 32 * 4	; 1024 * 16 pages are currently allocated

	mov edx, [ebp + 8]           ; phy_addr
	mov ecx, [ebp + 12]          ; len
	add ecx, edx                 ; phy_addr + len = end_addr

.l2_b:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	cmp edx, ecx
	jb .l2_b

	mov eax, VIRTUAL_LINEAR_FB_LOCATION

	pop edx
	pop ecx
	pop edi
	pop ebp
	ret

segment .bss
align 4096

; 1mo reserved for alpha pages tables Can allocate 1 go: Kernel_low: 64. Kernel_High: 64. Custom, 768.
; KERNEL SIZE CANNOT EXCEED 64 MO !
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
