[BITS 32]

segment .data

%if 0
; description d'un segment de la gdt
struct  gdt_seg {
    u16     limit_15_0;
    u16     base_15_0;
    u8      base_23_16;
    u8      access;
    u8      limit_19_16 : 4;
    u8      other       : 4;
    u8      base_31_24;
} __attribute__ ((packed));
%endif

;--------------------------------------------------------------------
gdt:
    db 0, 0, 0, 0, 0, 0, 0, 0
gdt_cs:
    dw 0xFFFF, 0x0000
    db 0x0, 10011011b, 11011111b, 0x0
gdt_ds:
    dw 0xFFFF, 0x0000
    db 0x0, 10010011b, 11011111b, 0x0
gdt_lfb:
    dw 0xFFFF, 0x0000
    db 0x00, 10010011b, 11011111b, 0x0
gdt_sp:
    dw 0x0000, 0x0000
    db 0x0, 10010011b, 11011111b, 0x0
gdtend:
;--------------------------------------------------------------------
gdtptr:
    dw 0  ; limite
    dd 0  ; base
;--------------------------------------------------------------------

segment .text

GLOBAL init_GDT

init_GDT:
    push ebp
    mov ebp, esp

; initialisation du pointeur sur la GDT
; -------------------------------------
    mov eax, gdtend    ; calcule la limite de GDT gdtptr->limite, sizeof(GDT)
    mov edx, gdt
    sub eax, edx
    mov word [gdtptr], ax

; recuperation de l'addresse linaire de la variable GDT pour gdtptr->base (c'est bien plus simple en 32 bits)
    mov eax, gdt
    mov dword [gdtptr + 2], eax
; -------------------------------------

; inscription du lfb dans le segment correspondant, l'addresse linéaire du LFB est passé en argument:
; une simple succession de décalages de la valeurs stockée dans eax permet de remplir les champs du segment
    mov eax, [ebp + 8]
    mov [gdt_lfb + 2], ax
    shr eax, 16
    mov [gdt_lfb + 4], al
    shr eax, 8
    mov [gdt_lfb + 7], al

; passage en modep
    lgdt [gdtptr]    ; charge la gdt

    jmp .next        ; reinitialisation du segment de code CS
.next:
    mov ax, 0x10     ; reinitialisation des segments de donnees DS, ES, FS et GS
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    pop ebp
ret
