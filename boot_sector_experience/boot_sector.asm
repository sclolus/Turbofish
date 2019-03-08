[BITS 16]
[ORG 0x7C00]

jmp start

;---------------------------------------------------------
; Synopsis: Affiche une chaine de caracteres se terminant par 0x0
; Entree:   DS:SI -> pointe sur la chaine a afficher
;---------------------------------------------------------
afficher:
    push ax
    push bx
.debut:
    lodsb           ; ds:si -> al
    cmp al, 0       ; fin chaine ?
    jz .fin
    mov ah, 0x0E    ; appel au service 0x0e, int 0x10 du bios
    mov bx, 0x07    ; bx -> attribut, al -> caractere ascii
    int 0x10
    jmp .debut

.fin:
    pop bx
    pop ax
    ret

start:
    sti                ; normally STI es already activated by bios boot
    mov ax, 0x8000  ; stack en 0xFFFF
    mov ss, ax
    mov sp, 0xf000

    mov si, msgDebut
    call afficher

; initialisation du pointeur sur la GDT
    mov ax, gdtend    ; calcule la limite de GDT
    mov bx, gdt
    sub ax, bx
    mov word [gdtptr], ax

    xor eax, eax      ; calcule l'adresse lineaire de GDT
    xor ebx, ebx
    mov ax, ds
    mov ecx, eax
    shl ecx, 4
    mov bx, gdt
    add ecx, ebx
    mov dword [gdtptr+2], ecx

; passage en modep
    cli
    lgdt [gdtptr]    ; charge la gdt
    mov eax, cr0
    or  ax, 1
    mov cr0, eax     ; PE mis a 1 (CR0)

    jmp next
next:
    mov ax, 0x10     ; segment de donne
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov es, ax
    mov ss, ax
    mov esp, 0x9F000

    jmp dword 0x8:end    ; reinitialise le segment de code
end:

[BITS 32]
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
    mov al, 0x08
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
    mov al, 0x0  ; 0xF9 => 11111000b. IRQ0(PIT channel 0 (clock)) IRQ1(keyboard) and IRQ2(slave connexion)
    out 0x21, al  ; store IMR


;   in al, 0xA1   ; get Interrupt Mask Register (IMR)
    mov al, 0x0  ; 0xFF => 11111111b. All slave interrupt are masked
    out 0xA1, al  ; store IMR

    jmp .end_init_pic
.end_init_pic:

; initialise temporary GDT
    mov eax, gdt_16_end
    sub eax, gdt_16
    mov word [gdt_16_ptr], ax

; store linear address of GDT
    mov eax, gdt_16
    mov dword [gdt_16_ptr + 2], eax

; revover bios_idt location; XXX It's useless here !
    lidt [bios_idt]

; load new 16 bits protected GDT
    lgdt [gdt_16_ptr]

; jump to CS of 16 bits selector
    jmp 0x8:.protected_16
.protected_16:

; code is now in 16bits, because we are in 16 bits mode

[BITS 16]
; set 16 bits protected mode data selector
    mov  ax, 0x10
    mov  ds, ax
    mov  es, ax
    mov  fs, ax
    mov  gs, ax
    mov  ss, ax

; disable protected bit
    mov eax, cr0
    and ax, 0xfffe
    mov cr0, eax

; configure CS in real mode
    jmp 0x0:.real_16
.real_16:

; configure DS, ES and SS in real mode
    xor ax, ax
    mov ds, ax
    mov es, ax

    mov ax, 0x8000  ; stack en 0xFFFF
    mov ss, ax
    mov sp, 0xf000

    sti

    mov si, return_16bits_real_msg
    call afficher
    jmp $

;--------------------------------------------------------------------
gdt:
    db 0, 0, 0, 0, 0, 0, 0, 0
gdt_cs:
    db 0xFF, 0xFF, 0x0, 0x0, 0x0, 10011011b, 11011111b, 0x0
gdt_ds:
    db 0xFF, 0xFF, 0x0, 0x0, 0x0, 10010011b, 11011111b, 0x0
gdtend:
;--------------------------------------------------------------------
gdtptr:
    dw 0  ; limite
    dd 0  ; base
;--------------------------------------------------------------------
bios_idt:
    dw 0x3ff ; limit
    dd 0     ; base


gdt_16:
    db 0, 0, 0, 0, 0, 0, 0, 0
gdt_16b_cs:
    dw 0xFFFF, 0x0000
    db 0x00, 0x9A, 0x0, 0x0
gdt_16b_ds:
    dw 0xFFFF, 0x0000
    db 0x00, 0x92, 0x0, 0x0
gdt_16_end:

gdt_16_ptr:
    dw 0  ; limite
    dd 0  ; base


msgDebut: db "Loading", 13, 10, 0
return_16bits_real_msg: db "16b real recovered", 13, 10, 0
;--------------------------------------------------------------------

;; NOP jusqu'a 510
times 510-($-$$) db 144
dw 0xAA55
