%define BASE    0x200  ; 0x0100:0x0 = 0x1000
%define KSIZE   50     ; nombre de secteurs a charger

[BITS 16]
[ORG 0x0]

jmp start                    ;Saut nécessaire pour l'inclusion de fichier tière en début de fichier. L'execution du code commencera ainsi à Start et le programme ne plantera pas du coup !
%include "../tools/16b_system.asm"
%include "../tools/16b_screen.asm"
%include "UTIL.INC"
start:

cli
    mov ax, cs              ; Réinitialise les segments de données sur celui de code actuel.        Le nouveau segment de code est le 0x7E00.
    mov ds, ax
    mov es, ax              ; La pile précédente déclarée entre 8000:0000 et 8000:2000 convient encore très bien, innutile de la redéfinir ici.
sti

; recuparation de l'unite de boot
    mov [bootdrv], dl

; affiche un msg
    mov si, msgDebut
    call afficher

; charger le noyau
    xor ax, ax
    int 0x13

    push es
    mov ax, BASE
    mov es, ax
    mov bx, 0
    mov ah, 2
    mov al, KSIZE
    mov ch, 0
    mov cl, 17
    mov dh, 0
    mov dl, [bootdrv]
    int 0x13
    pop es

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
    mov cr0, eax        ; PE mis a 1 (CR0)

    jmp next
next:
    mov ax, 0x10        ; segment de donne
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov es, ax
    mov ss, ax
    mov esp, 0x9F000

    jmp dword 0x8:0x2000    ; reinitialise le segment de code

;--------------------------------------------------------------------
bootdrv:  db 0
msgDebut: db "Chargement du kernel", 13, 10, 0
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

times 7168-($-$$) db 144        ; NOP
