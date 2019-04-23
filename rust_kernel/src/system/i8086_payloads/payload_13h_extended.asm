[BITS 16]

%define PACKET_SEGMENT 0x8000
%define PACKET_OFFSET 0x0

%macro INIT_STACK 0
	; Put stack on 0x84000
	mov ax, 0x8000
	mov ss, ax
	mov bp, 0x4000
	mov sp, bp
%endmacro

; Memory space at 0x8000:0000 (0x80000 in protected mode)
; 0x80000 |  PACKET |
;         |    |    |
;         |    v    |
;         |         |
;         |         |
;         |    ^    |
;         |    |    |
;         |  STACK  |
; 0x84000 +---------+

; INT 13h AH=41h: Check Extensions Present

; @Parameters
; Registers     Description
; AH            41h = function number for extensions check[8]
; DL            drive index (e.g. 1st HDD = 80h)
; BX            55AAh

; @Results Registers Description
; CF Set On Not Present, Clear If Present
; AH Error Code or Major Version Number
; BX AA55h
; CX Interface support bitmask:
; 1 – Device Access using the packet structure
; 2 – Drive Locking and Ejecting
; 4 – Enhanced Disk Drive Support (EDD)

; DL must be provided by caller

segment .text

GLOBAL payload_13h_check_extension_present
payload_13h_check_extension_present:
	INIT_STACK

	mov ax, 0x4100
	mov bx, 0x55AA

	int 13h
	jnc .end
.error:
	mov eax, -1
.end:

segment .data

GLOBAL payload_13h_check_extension_present_len
payload_13h_check_extension_present_len: dd (payload_13h_check_extension_present.end - payload_13h_check_extension_present)

; INT 13h AH=48h: Extended Read Drive Parameters

; Parameters Registers Description
; AH 48h = function number for extended_read_drive_parameters
; DL drive index (e.g. 1st HDD = 80h)
; DS:SI segment:offset pointer to Result Buffer, see below
;    Result Buffer offset range size description
;    00h..01h 2 bytes size of Result Buffer (set this to 1Eh)
;    02h..03h 2 bytes information flags
;    04h..07h 4 bytes physical number of cylinders = last index + 1 (because index starts with 0)
;    08h..0Bh 4 bytes physical number of heads = last index + 1 (because index starts with 0)
;    0Ch..0Fh 4 bytes physical number of sectors per track = last index (because index starts with 1)
;    10h..17h 8 bytes absolute number of sectors = last index + 1 (because index starts with 0)
;    18h..19h 2 bytes bytes per sector
;    1Ah..1Dh 4 bytes optional pointer to Enhanced Disk Drive (EDD) configuration parameters which may be used for subsequent interrupt 13h Extension calls (if supported)

; Results Registers Description
; CF Set On Error, Clear If No Error
; AH Return Code

; DL must be provided by caller
; buffer is fixed at 8000:0000
; Result Buffer (size_field) must be provided by caller

segment .text

GLOBAL payload_13h_extended_read_drive_parameters
payload_13h_extended_read_drive_parameters:
	INIT_STACK

	mov ax, 0x4800

	mov cx, PACKET_SEGMENT
	mov ds, cx
	mov si, PACKET_OFFSET

	int 13h
	jnc .resume
.error:
	mov eax, -1
.resume:
	xor cx, cx
	mov ds, cx
.end:

segment .data

GLOBAL payload_13h_extended_read_drive_parameters_len
payload_13h_extended_read_drive_parameters_len: dd (payload_13h_extended_read_drive_parameters.end - payload_13h_extended_read_drive_parameters)

; INT 13h AH=42h: Extended Read Sectors From Drive

; Parameters Registers Description
; AH 42h = function number for extended read
; DL drive index (e.g. 1st HDD = 80h)
; DS:SI segment:offset pointer to the DAP, see below
; DAP : Disk Address Packet offset range size description
;     00h 1 byte size of DAP (set this to 10h)
;     01h 1 byte unused, should be zero
;     02h..03h 2 bytes number of sectors to be read, (some Phoenix BIOSes are limited to a maximum of 127 sectors)
;     04h..07h 4 bytes segment:offset pointer to the memory buffer to which sectors will be transferred (note that x86 is little-endian: if declaring the segment and offset separately, the offset must be declared before the segment)
;     08h..0Fh 8 bytes absolute number of the start of the sectors to be read (1st sector of drive has number 0) using logical block addressing.

; Results Registers Description
; CF Set On Error, Clear If No Error
; AH Return Code

; DL must be provided by caller
; buffer is fixed at 8000:0000
; DAP : Disk Address Packet Must be provided by caller

segment .text

GLOBAL payload_13h_extended_read_sectors
payload_13h_extended_read_sectors:
	INIT_STACK

	mov ax, 0x4200

	mov cx, PACKET_SEGMENT
	mov ds, cx
	mov si, PACKET_OFFSET

	int 13h
	jnc .resume
.error:
	mov eax, -1
.resume:
	xor cx, cx
	mov ds, cx
.end:

segment .data

GLOBAL payload_13h_extended_read_sectors_len
payload_13h_extended_read_sectors_len: dd (payload_13h_extended_read_sectors.end - payload_13h_extended_read_sectors)

; INT 13h AH=43h: Extended Write Sectors to Drive

; Parameters Registers Description
; AH 43h = function number for extended write
; AL
; bit 0 = 0: close write check,
; bit 0 = 1: open write check,
; bit 1-7:reserved, set to 0

; DL drive index (e.g. 1st HDD = 80h)
; DS:SI segment:offset pointer to the DAP
;     Results Registers Description
;     CF Set On Error, Clear If No Error
;     AH Return Code

; DL must be provided by caller
; buffer is fixed at 8000:0000
; DAP : Disk Address Packet Must be provided by caller

segment .text

GLOBAL payload_13h_extended_write_sectors
payload_13h_extended_write_sectors:
	INIT_STACK

	mov ax, 0x4300

	mov cx, PACKET_SEGMENT
	mov ds, cx
	mov si, PACKET_OFFSET

	int 13h
	jnc .resume
.error:
	mov eax, -1
.resume:
	xor cx, cx
	mov ds, cx
.end:

segment .data

GLOBAL payload_13h_extended_write_sectors_len
payload_13h_extended_write_sectors_len: dd (payload_13h_extended_write_sectors.end - payload_13h_extended_write_sectors)
