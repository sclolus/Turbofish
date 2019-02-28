; see https://wiki.osdev.org/Paging
; This file defines constants and methods to set kernel in half high memory
; It must be include in all boot/init sources files before switch to half high memory

; early paging
extern virtual_offset
extern kernel_physical_start
extern kernel_physical_end

; Some usefull paging const
%define READ_WRITE (1 << 1)
%define PRESENT (1 << 0)

; only the first 20 bits is signifiant for pages
%define PAGE_MASK 0xfffff000
%define PAGE_SIZE 4096
%define PAGE_TABLE_PER_DIRECTORY 1024

; Translate high mem address to low address: fn foo(address) -> eax
%macro TRANSLATE_ADDR 1
	lea eax, [%1]
	sub eax, dword virtual_offset
%endmacro

; Get the kernel size: fn foo(eax) -> eax
%macro GET_KERNEL_LENGTH 0
	push edx
	mov eax, dword kernel_physical_end
	mov edx, dword kernel_physical_start
	sub eax, edx
	pop edx
%endmacro

; Convert a size in bytes to a number of pages: fn foo(eax) -> eax
%macro BYTES_TO_PAGES 0
	push edx
	mov edx, eax
	shr eax, 12
	and edx, PAGE_MASK
	cmp edx, 0
	je %%next
	inc eax
%%next:
	pop edx
%endmacro

; Manual pagination system:
; %1 Area name
; %2 Offset in page directory: value ∈ [0..1024] -> virt_addr ∈ [0x0..0x1_00_00_00_00]
; %3 location in page table: value ∈ [0..2^6] (limitation due to 1go max of addressable mem)
; %4 Physical area associated
; %5 Len of Physical area in mo (BE CAREFULL: MUST BE MULTIPLE OF 2^2)

%macro PAGINATE_ADDR 5
	; EDI = (page_directory_alpha_area - virtual_offset) + (%2 * 4)
	mov edx, %2 * 4
	TRANSLATE_ADDR page_directory_alpha_area
	add eax, edx
	mov edi, eax

	; EDX = (page_table_alpha_area - virtual_offset) + (PAGE_TABLE_PER_DIRECTORY * %3 * 4)
	mov edx, PAGE_TABLE_PER_DIRECTORY * %3 * 4
	TRANSLATE_ADDR page_table_alpha_area
	add edx, eax

	mov ecx, %5
	shr ecx, 2          ; -> initialize counter of len / 4 (paquets of 4mb
.%1_a:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	loop .%1_a

	; Mapping of physical address
	; EDI = (page_table_alpha_area - virtual_offset) + (PAGE_TABLE_PER_DIRECTORY * %3 * 4)
	mov edx, PAGE_TABLE_PER_DIRECTORY * %3 * 4
	TRANSLATE_ADDR page_table_alpha_area
	add eax, edx
	mov edi, eax

	mov edx, %4 				; -> beginning of physical area associated
.%1_b:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	cmp edx, %4 + (1 << 20) * %5 ; -> rep until cur_phy_addr != base_phy_addr + len in mb
	jne .%1_b
%endmacro

; Functionnal pagination system:
; %1 Offset in page directory: value ∈ [0..1024] -> virt_addr ∈ [0x0..0x1_00_00_00_00]
; %2 location in page table: value ∈ [0..2^6] (limitation due to 1go max of addressable mem)
; %3 Physical area associated
; %4 Len of Physical area in octet

; CAUTION: Usable only when high memory is initialized
segment .text
GLOBAL _dynamic_map:
_dynamic_map:
	push ebp
	mov ebp, esp

	push edi
	push ecx
	push edx

	mov eax, [ebp + 8]
	mov edx, 4
	mul edx

	mov edx, eax
	TRANSLATE_ADDR page_directory_alpha_area
	add eax, edx
	mov edi, eax

	mov eax, [ebp + 12]
	mov edx, (PAGE_TABLE_PER_DIRECTORY * 4)
	mul edx
	mov edx, eax

	TRANSLATE_ADDR page_table_alpha_area
	add edx, eax

	push edx

	mov ecx, [ebp + 20]          ; len
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

	pop edx
	mov edi, edx

	mov edx, [ebp + 16]          ; phy_addr
	mov ecx, [ebp + 20]          ; len
	add ecx, edx                 ; phy_addr + len = end_addr

.l2_b:
	mov eax, edx
	and eax, PAGE_MASK
	or eax, READ_WRITE | PRESENT
	stosd
	add edx, PAGE_SIZE
	cmp edx, ecx
	jb .l2_b

	pop edx
	pop ecx
	pop edi

	xor eax, eax

	pop ebp
	ret

GLOBAL _get_kernel_length
_get_kernel_length:
	GET_KERNEL_LENGTH
	ret

segment .data
current_page: dd 0

segment .bss
align 4096

; 1mo reserved for alpha pages tables Can allocate 512 mo: Kernel_low: 256 mo. Kernel_High: 256 Mo. Custom, -?-.
; KERNEL SIZE CANNOT EXCEED 64 MO !
GLOBAL page_table_alpha_area
page_table_alpha_area:
	resb 1 << 19

; 4kb reserved for alpha pages table directory: Can allocate 4 go
GLOBAL page_directory_alpha_area
page_directory_alpha_area:
	resb 1 << 12
