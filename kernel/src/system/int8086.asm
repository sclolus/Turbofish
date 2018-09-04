; Cette fonction permet d'acceder aux instructions BIOS 16 bits réelles depuis le mode protégé 32 bits.
; La stratégie consiste à copier une partie du code dans un endroit mémoire en deca des 1mo puis de l'executer.
; D'abord, on passe au mode 16 bits protégé avec une GDT intégrée ici, puis enfin on passe au mode réel pour déclencher l'INT.
; Enfin, l'on retourne au 32bits protégé.

; Limitations: seul le numero de l'interuption ainsi que les registres AX, BX, CX, DX, Si et DI seront passés.
; La GDT de l'appelant doit avoir son secteur de code en 0x8 et doit etre bien entendu sur un descripteur 32 bits.
; La pagination ne doit pas etre activée !

[BITS 32]
%define BASE_LOCATION 0x7C00
%define REBASE(x) BASE_LOCATION + x - begin_sub_sequence

GLOBAL int8086
int8086:
    push ebp
    mov ebp, esp

; store all registers values
    pushad

; copy of content at BASE_LOCATION
    mov eax, end_sub_sequence
    sub eax, begin_sub_sequence
    mov ecx, eax
    mov esi, begin_sub_sequence
    mov edi, BASE_LOCATION
    rep movsb

; initialise temporary GDT
    mov eax, gdt_16_end
    sub eax, gdt_16
    mov word [REBASE(gdt_16_ptr)], ax

; store linear address of GDT
    mov eax, gdt_16
    mov dword [REBASE(gdt_16_ptr + 2)], eax

; fill the number of the interupt to launch
    mov al, [ebp + 8]
    mov byte [REBASE(int_nb_location)], al

; push a address to join after execution with instruction ret
    push end_int8086

    jmp BASE_LOCATION

end_int8086:
; restore all registers values
    popad

    pop ebp
ret

; -------------------------------------------------
; *** This part is copied in BASE_LOCATION area ***
; -------------------------------------------------
begin_sub_sequence:
; saving of segment register
    mov [REBASE(_ds)], ds
    mov [REBASE(_es)], es
    mov [REBASE(_fs)], fs
    mov [REBASE(_gs)], gs
    mov [REBASE(_ss)], ss

; store caller idt and load BIOS idt
    sidt [REBASE(saved_idtptr)]
    lidt [REBASE(bios_idt)]

; store caller gdt and load custom 16 bits gdt
    sgdt [REBASE(saved_gdtptr)]
    lgdt [REBASE(gdt_16_ptr)]

; take parameters registers values
    mov edi, [ebp + 12]
    mov esi, [ebp + 16]
    mov ebx, [ebp + 28]
    mov edx, [ebp + 32]
    mov ecx, [ebp + 36]
    mov eax, [ebp + 40]
    mov [REBASE(_eax)], eax

; jump to CS of 16 bits selector
    jmp 0x8:REBASE(.protected_16)
.protected_16:

; code is now in 16bits, because we are in 16 bits mode
[BITS 16]
; disable protected bit
    mov eax, cr0
    and ax, 0xfffe
    mov cr0, eax

; configure CS in real mode
    jmp 0x0:REBASE(.real_16)
.real_16:

; configure DS, ES and SS in real mode
    xor ax, ax
    mov ds, ax
    mov es, ax
    mov ss, ax

; take saved eax
    mov eax, [REBASE(_eax)]

; enable interupts
    sti

; launch interupt 0xCD is the opcode of INT
    db 0xCD
int_nb_location:
    db 0x0

; disable interupt
    cli

; load caller idt and caller gdt
    lidt [REBASE(saved_idtptr)]
    lgdt [REBASE(saved_gdtptr)]    ; charge la gdt courante

; entering in protected mode
    mov eax, cr0
    or  ax, 1
    mov cr0, eax     ; PE mis a 1 (CR0)

; configure CS in protected mode
    jmp 0x8:REBASE(.protected_32)
.protected_32:

; code is now in 16bits
[BITS 32]
; restore all segments registers
    mov ds, [REBASE(_ds)]
    mov es, [REBASE(_es)]
    mov fs, [REBASE(_fs)]
    mov gs, [REBASE(_gs)]
    mov ss, [REBASE(_ss)]

; return to base function
    ret

bios_idt:
    dw 0x3ff ; limit
    dd 0     ; base
saved_idtptr:
    dw 0
    dd 0
saved_gdtptr:
    dw 0  ; limit
    dd 0  ; base

_eax: dd 0
_ds: dw 0
_es: dw 0
_fs: dw 0
_gs: dw 0
_ss: dw 0

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

end_sub_sequence:
; -------------------------------------------------
