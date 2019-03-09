[BITS 16]

segment .text

; Detect the memory device physical map:

; sources: https://wiki.osdev.org/Detecting_Memory_(x86)#Getting_an_E820_Memory_Map

; By far the best way to detect the memory of a PC is by using the INT 0x15, EAX = 0xE820 command.
; This function is available on all PCs built since 2002, and on most existing PCs before then.
; It is the only BIOS function that can detect memory areas above 4G. It is meant to be the ultimate memory detection BIOS function.

; In reality, this function returns an unsorted list that may contain unused entries and (in rare/dodgy cases)
; may return overlapping areas. Each list entry is stored in memory at ES:DI, and DI is not incremented for you.
; The format of an entry is 2 uint64_t's and a uint32_t in the 20 byte version, plus one additional
; uint32_t in the 24 byte ACPI 3.0 version (but nobody has ever seen a 24 byte one). It is probably best to always
; store the list entries as 24 byte quantities -- to preserve uint64_t alignments, if nothing else.
; (Make sure to set that last uint64_t to 1 before each call, to make your map compatible with ACPI

%define BIOS_MAGIC_A        0x534d4150
%define BIOS_MAGIC_B        0xe820
%define DEVICE_MAP_PTR_SEG  0x4000 ; linear 0x40000

GLOBAL payload_get_mem_map
payload_get_mem_map:
	; stack on 0x84000
	mov ax, 0x8000
	mov ss, ax
	mov bp, 0x4000
	mov sp, bp

	; initial ES:DI at 0x40000 (segment 5. 256ko -> 320ko)
	mov ax, DEVICE_MAP_PTR_SEG
	mov es, ax
	xor di, di

	; assign values for the first call
	mov edx, BIOS_MAGIC_A
	mov eax, BIOS_MAGIC_B
	mov ecx, 24
	xor ebx, ebx

.first_calling:
	int 15h
	; check if Carry Flag is Clear and EAX
	jc .l_error
	cmp eax, BIOS_MAGIC_A
	jne .l_error

	; set the result length to 1 and push it
	mov eax, 1
	push eax

.next_calling:
	mov eax, BIOS_MAGIC_B
	mov ecx, 24
	add edi, 32

	int 15h

	; add result by 1
	pop eax
	inc eax
	push eax

	; check if Carry Flag is Clear and EAX
	jc .l_success
	cmp ebx, 0
	jne .next_calling

.l_success:
	pop eax
	jmp .end
.l_error:
	mov eax, -1
.end:

segment .data

GLOBAL payload_get_mem_map_len
payload_get_mem_map_len: dd (payload_get_mem_map.end - payload_get_mem_map)
