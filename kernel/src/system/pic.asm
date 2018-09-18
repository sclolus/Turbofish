; IRQ 	Description
; 0 Programmable Interrupt Timer Interrupt
; 1 Keyboard Interrupt
; 2 Cascade (used internally by the two PICs. never raised)
; 3 COM2 (if enabled)
; 4 COM1 (if enabled)
; 5 LPT2 (if enabled)
; 6 Floppy Disk
; 7 LPT1
; 8 CMOS real-time clock (if enabled)
; 9 Free for peripherals / legacy SCSI / NIC
; 10 Free for peripherals / SCSI / NIC
; 11 Free for peripherals / SCSI / NIC
; 12 PS2 Mouse
; 13 FPU / Coprocessor / Inter-processor
; 14 Primary ATA Hard Disk
; 15 Secondary ATA Hard Disk


; ICW (Initialization Command Word): reinit the controller
; OCW (Operation Control Word): configure the controller once initialized (used to mask/unmask the interrupts)

; We have to init ICW1, ICX2, ICW3, ICW4 and OCW0 step by step

[BITS 32]
segment .text
GLOBAL init_pic
init_pic:
    push ebp
    mov ebp, esp

    jmp .icw_1

; |0|0|0|1|x|0|x|x|
;        |   | +--- with ICW4 (1) or without (0)
;        |   +----- one controller (1), or cascade (0)
;        +--------- triggering by level (level) (1) or by edge (edge) (0)
.icw_1: ; ICW1 (port 0x20 / port 0xA0)
    mov al, 0x11
    out 0x20, al  ; master
    out 0xA0, al  ; slave
    jmp .icw_2

; |x|x|x|x|x|0|0|0|
;  | | | | |
; +----------------- base address for interrupts vectors
.icw_2: ; ICW2 (port 0x21 / port 0xA1) Set vector offset. IRQ below 32 are processor reserved IRQ
    mov al, 0x20
    out 0x21, al  ; master, begin at 32 (to 39)
    mov al, 0x70
    out 0xA1, al  ; slave, begin at 112 (to 119)
    jmp .icw_3

.icw_3: ; ICW3 (port 0x21 / port 0xA1) set how are connected pic master and slave
; |x|x|x|x|x|x|x|x|  for master
;  | | | | | | | |
;  +------------------ slave controller connected to the port yes (1), or no (0)
    mov al, 0x04  ; master is connector 3 of slave
    out 0x21, al

; |0|0|0|0|0|x|x|x|  for slave
;            | | |
;            +-+-+----- Slave ID which is equal to the master port
    mov al, 0x02  ; slave is connector 2 of master
    out 0xA1, al
    jmp .icw_4

; |0|0|0|x|x|x|x|1|
;        | | | +------ mode "automatic end of interrupt" AEOI (1)
;        | | +-------- mode buffered slave (0) or master (1)
;        | +---------- mode buffered (1)
;        +------------ mode "fully nested" (1)
.icw_4: ; ICW4 (port 0x21 / port 0xA1)
    mov al, 0x01
    out 0x21, al
    out 0xA1, al
    jmp .ocw_1

; |x|x|x|x|x|x|x|x|
;  | | | | | | | |
;  +-+-+-+-+-+-+-+---- for each IRQ : interrupt mask actif (1) or not (0)

.ocw_1:           ; Interrupt mask
;   in al, 0x21   ; get Interrupt Mask Register (IMR)
    mov al, 0xF8  ; 0xF9 => 11111000b. IRQ0(PIT channel 0 (clock)) IRQ1(keyboard) and IRQ2(slave connexion)
    out 0x21, al  ; store IMR


;   in al, 0xA1   ; get Interrupt Mask Register (IMR)
    mov al, 0xFF  ; 0xFF => 11111111b. All slave interrupt are masked
    out 0xA1, al  ; store IMR

    jmp .end_init_pic
.end_init_pic:

    pop ebp
ret
