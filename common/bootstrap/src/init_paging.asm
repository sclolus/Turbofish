[BITS 32]

%include "src/early_paging.asm"

segment .text

%define KERNEL_MAX_SIZE 64

; from https://web.archive.org/web/20130609073242/http://www.osdever.net/tutorials/rm_addressing.php?the_id=50

; The x86-16/32 CPU Runs in Two Modes - RealMode (16-Bit) and ProtectedMode (32-Bit),
; (There are a few big Differences Between these modes - The HUGE Difference
; is the way Memory is handled - We will not explain the other differences:
;     This information is beyond the purpose of this tutorial, We will also not explain
;     anything about ProtectedMode in this tutorial - As this tutorial is all about RealMode.)
; NOTE: On the x86 Platform, the CPU (Central Processing Unit) Boots into RealMode.

; On the x86 Platform there is something called Address Space
; This is space that maps out Memory on various devices
; It mostly maps out RAM (Randon Acess Memory - The Computer's Main Memory)
; But it also maps other Memory such as: Video Memory (Video RAM),
; The BIOS ROM's (Read Only Memory) - There are some other things aswell.
; In RealMode you can only acess 1mb of Address Space
; This 1mb Maps out RAM, Video RAM, and the BIOS ROM's - Some of the memory is also taken up.

; Below is a Memory-Map of RealMode Memory, it shows what is located at each Memory Address (In RealMode)
; It also shows if that Memory Address is free for system use
; (When i say "System Use" i am referring to the State of the Memory when the System "Boots"
; You might not Understand what this Map means right now - But it will help later on, So you might want to come back to it later);

; Mapping of the first mo

; 0x0000:0x0000 -> 0x0000:0x03FF = IVT (Interrupt Vector Table)
; 0x0000:0x0400 -> 0x0000:0x04FF = BDA (BIOS Data Area)

; 0x0000:0x0500 -> 0x0000:0x7BFF = Free Useable Memory!

; 0x0000:0x7C00 -> 0x0000:0x7DFF = Operating System BootSector - This is where the BIOS
; Loads your Operating System's BootSector at Boot Time (You can use this Memory, as long as
; your BootSector isn't executing and you don't need your BootSector anymore!)

; 0x0000:0x7E00 -> 0x9000:0xFFFF = Free Useable Memory!

; 0xA000:0x0000 -> 0xB000:0xFFFF = VGA Video RAM
; 0xC000:0x0000 -> 0xF000:0xFFFF = BIOS ROM Memory Area

; 0xFFFF:0x0010 -> 0xFFFF:0xFFFF = Free Useable Memory (If the A20 Gate is Enabled) - This
; Memory is Above the 1mb Mark, 0xFFFF:0x0010 = 0x00100000(1mb)

; The grub bootloader let the kernel in 32 bits protected mode here:

global init_paging
init_paging:
	push ebp
	mov ebp, esp
	; block interrupts

	; Paginate kernel in half high memory (do also identity mapping)
	INIT_PAGING_ENV

	; 0x00000000 -> 0x04000000 mapped to phy 0x00000000 -> 0x04000000

%define l0_virt_offset 0
%define l0_physic_addr 0
%define l0_len KERNEL_MAX_SIZE

	PAGINATE_ADDR l0_virt_offset, l0_physic_addr, l0_len

	; 0xC0000000 -> 0xC4000000 mapped to phy 0x00000000 -> 0x04000000

%define l1_virt_offset 768
%define l1_physic_addr 0
%define l1_len KERNEL_MAX_SIZE

	PAGINATE_ADDR l1_virt_offset, l1_physic_addr, l1_len

	; Active paging
	mov eax, page_directory_alpha_area
	mov cr3, eax ; fill CR3 with physic mem pointer to page directory

	mov eax, cr0
	or eax, 0x80000001          ; enable Paging bit (PG). Protection bit must be also recall here
	mov cr0, eax

	pop ebp
	ret

%define VIRTUAL_LINEAR_FB_LOCATION 0xF0000000

; 0xF0000000 -> ... mapped to phy ??? -> ??? + LFB_SIZE
; hack for LFB allocation
; CAUTION: Usable only when high memory is initialized
global _allocate_linear_frame_buffer
_allocate_linear_frame_buffer:
	push ebp
	mov ebp, esp

	push dword [ebp + 12]               ; len
	push dword [ebp + 8]                ; physical address
	push (1024 - 64)                    ; virt addr offset. eq 0xF0000000

	call _dynamic_map

	add esp, 12

	mov eax, VIRTUAL_LINEAR_FB_LOCATION

	pop ebp
	ret
