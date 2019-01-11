[BITS 32]
segment .data

pit_time_sec: dd 0
pit_time_usec: dd 0
pit_period_usec: dd 0
pit_frequency: dd 0

segment .text

; PIT I/O ports
; I/O port     Usage
; 0x40         Channel 0 data port (read/write)
; 0x41         Channel 1 data port (read/write)
; 0x42         Channel 2 data port (read/write)
; 0x43         Mode/Command register (write only, a read is ignored)

; The channel 0 is connected to IRQ0

; Command register.
; Bits         Usage
; 6 and 7      Select channel :
;                 0 0 = Channel 0
;                 0 1 = Channel 1
;                 1 0 = Channel 2
;                 1 1 = Read-back command (8254 only)
; 4 and 5      Access mode :
;                 0 0 = Latch count value command
;                 0 1 = Access mode: lobyte only
;                 1 0 = Access mode: hibyte only
;                 1 1 = Access mode: lobyte/hibyte
; 1 to 3       Operating mode :
;                 0 0 0 = Mode 0 (interrupt on terminal count)
;                 0 0 1 = Mode 1 (hardware re-triggerable one-shot)
;                 0 1 0 = Mode 2 (rate generator)
;                 0 1 1 = Mode 3 (square wave generator)
;                 1 0 0 = Mode 4 (software triggered strobe)
;                 1 0 1 = Mode 5 (hardware triggered strobe)
;                 1 1 0 = Mode 2 (rate generator, same as 010b)
;                 1 1 1 = Mode 3 (square wave generator, same as 011b)
; 0            BCD/Binary mode: 0 = 16-bit binary, 1 = four-digit BCD

; The square wave generator is perfect for a base timer
; 1193182 / divisor = frequency
; divisor = 1193182 / frequency

%define CHANNEL0      (0)
%define LOBYTE_HIBYTE ((1 << 4) | (1 << 5))
%define MODE_3        (1 << 2)
%define BINARY_MODE   (0)

%define US_IN_SEC     1000000

GLOBAL asm_pit_init ; void asm_pit_init(u32 frequency)
asm_pit_init:
    push ebp
    mov ebp, esp

    mov al, CHANNEL0 | LOBYTE_HIBYTE | MODE_3 | BINARY_MODE
    out 0x43, al

    mov ecx, [ebp + 8]
    mov edx, 0
    mov eax, 1193182

    div ecx      ; the result of edx:eax / ecx is stored in eax
    push eax

    mov ecx, eax
    mov edx, 0
    mov eax, 1193182

    div ecx
    mov dword [pit_frequency], eax

    mov ecx, eax
    mov edx, 0
    mov eax, US_IN_SEC

    div ecx
    mov dword [pit_period_usec], eax

    pop eax

    out 0x40, al ; set low byte divisor
    shr ax, 8
    out 0x40, al ; set high byte divisor

    pop ebp
    ret

GLOBAL clock_gettime ; void clock_gettime(struct timeval *tv)
clock_gettime:
    push ebp
    mov ebp, esp

    mov eax, [ebp + 8]
    mov edx, [pit_time_sec]
    mov [eax], edx
    mov edx, [pit_time_usec]
    mov [eax + 4], edx

    xor eax, eax

    pop ebp
    ret

GLOBAL asm_pit_isr
asm_pit_isr:
    push eax

    mov eax, dword [pit_time_usec]
    add eax, dword [pit_period_usec]

    cmp eax, US_IN_SEC
    jb .next

    sub eax, US_IN_SEC
    add dword [pit_time_sec], 1

.next:
    mov dword [pit_time_usec], eax

    mov al, 0x20
    out 0x20, al

    pop eax
    iret
