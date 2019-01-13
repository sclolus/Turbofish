[BITS 32]

; ACCESS BYTES DESCRIPTION
%DEFINE ACCESSED 1; WAS IT ACCESSED
%DEFINE READ_WRITE 1 << 1; FOR DATA SEGMENT IS WRITING ALLOWED ?
%DEFINE GROWTH_DIRECTION 1 << 2; FOR DATA SEGMENT: TO LOWER OR TO HIGHER ADDRESS. FOR TEXT SEGMENT : CAN IT BE EXECTUDED WITH HIGHER PRIVILEDGE ?
%DEFINE EXECUTABLE 1 << 3
%DEFINE SYSTEM_HOLDER 1 << 4; IS IT DATA/CODE ? (1) OR IS IT JUST SOME SYSTEM INFORMATION HOLDER (0)
%DEFINE DPL 1 << 5 | 1 << 6; DESCRIPTOR PRIVILEGE LEVEL (RING)
%DEFINE PR 1 << 7; PRESENT IN MEMORY RIGHT NOW ?

; LIMIT FLAG DESCRIPTION
%DEFINE V 1 << 4; AVAILABLE TO USE FOR SYSTEM SOFTWARE ?
%DEFINE LONGMODE 1 << 5; IS IT A 64 BIT MODE SEGMENT ?
%DEFINE SIZE 1 << 6; (0) 16 BIT (1) FOR 32 BIT PROTECTED
%DEFINE GRANULARITY 1 << 7; LIMIT IS IN 0 = BYTES, 1 = PAGES OF LIMIT 4096 BYTES EACH

;struc gdt_entry_struct
;	limit_0_15:				resb 2
;	base_0_15:				resb 2
;	base_16_23:				resb 1
;	access_bytes:			resb 1
;	limit_flags:			resb 1
;	base_24_31:				resb 1
;endstruc

segment .data

extern _start_after_init_gdt
gdt_info:
	dw gdt_end - gdt_start
	dd gdt_start

gdt_start:
	; empty selector
	TIMES 8 db 0
	
	; CODE SELECTOR :
;	limit_0_15:
	dw 0xffff
;	base_0_15:
	dw 0
;	base_16_23:
	db 0
;	access_bytes:
	db PR | SYSTEM_HOLDER | EXECUTABLE
;	limit_flags:
	db 0xf | SIZE | GRANULARITY
;	base_24_31:
	db 0
	
	; DATA SELECTOR :
;	limit_0_15:
	dw 0xffff
;	base_0_15:
	dw 0
;	base_16_23:
	db 0
;	access_bytes:
	db PR | SYSTEM_HOLDER | READ_WRITE
;	limit_flags:
	db 0xf | SIZE | GRANULARITY
;	base_24_31:
	db 0
	
	; STACK SELECTOR :
;	limit_0_15:
	dw 0xffff
;	base_0_15:
	dw 0
;	base_16_23:
	db 0
;	access_bytes:
	db PR | SYSTEM_HOLDER | READ_WRITE
;	limit_flags:
	db 0xf | SIZE | GRANULARITY
;	base_24_31:
	db 0
gdt_end: 

segment .text

global init_gdt
init_gdt:
	lgdt [gdt_info]

	; CS IS CODE SEGMENT REGISTER
	jmp 0x8:landing
landing:

	; DS, ES, FS and GS ARE DATA SEGMENT REGISTER
	mov ax, 0x10
	mov ds, ax
	mov es, ax
	mov fs, ax
	mov gs, ax

	; SS IS STACK SEGMENT REGISTER
	mov ax, 0x18
	mov ss, ax

	jmp _start_after_init_gdt
