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
    mov si, msgDebut
    call afficher

    mov ax, 0x8000  ; stack en 0xFFFF
    mov ss, ax
    mov sp, 0xf000

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
    mov ss, ax

; enable interupts
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
