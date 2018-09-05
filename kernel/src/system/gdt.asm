[BITS 32]

segment .data

%if 0
; synospys of gdt segment
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
    dw 0  ; limit
    dd 0  ; base
;--------------------------------------------------------------------

segment .text

GLOBAL init_GDT

%define BASE_LOCATION 0x800
%define REBASE(x) BASE_LOCATION + x - gdt

; *** This GDT is rebased at 0x800 ***
init_GDT:
    push ebp
    mov ebp, esp

; copying of GDT at BASE_LOCATION, here 0x800
    mov edi, BASE_LOCATION
    mov eax, gdtend
    sub eax, gdt
    mov ecx, eax
    mov esi, gdt
	rep movsb

; initialization of GDT limit: eq gdtptr.limit = sizeof(GDT)
    mov eax, gdtend
    mov edx, gdt
    sub eax, edx
    mov word [gdtptr], ax

; initialization of GDT base: eq gdtptr.base = &GDT
    mov eax, REBASE(gdt)
    mov dword [gdtptr + 2], eax

; writing of Lineat Frame Buffer address into LFB segment
    mov eax, [ebp + 8]
    mov [REBASE(gdt_lfb + 2)], ax
    shr eax, 16
    mov [REBASE(gdt_lfb + 4)], al
    shr eax, 8
    mov [REBASE(gdt_lfb + 7)], al

    lgdt [gdtptr]    ; Load GDT

    jmp .next        ; reinit CS segment
.next:
    mov ax, 0x10     ; reinit DATA segments, DS, ES, FS et GS
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    pop ebp
ret
