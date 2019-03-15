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
    u8      flags       : 4;
    u8      base_31_24;
} __attribute__ ((packed));
%endif

; Definition of access field:
%define AC (1 << 0) ; Accessed bit. Just set to 0. The CPU sets this to 1 when the segment is accessed
%define RW (1 << 1) ; Readable bit/Writable bit.
%define DC (1 << 2) ; Direction bit/Conforming bit.
; Direction bit for data selectors: Tells the direction. 0 the segment grows up. 1 the segment grows down
; If 0 code in this segment can only be executed from the ring set in privl
%define EX (1 << 3) ; Executable bit. If 1 code in this segment can be executed, ie. a code selector. If 0 it is a data selector.
%define U0 (1 << 4) ; always need to be set to 1
%define PR_RING3 ((1 << 5) | (1 << 6)) ; Privilege, 2 bits. Contains the ring level, 0 = highest (kernel), 3 = lowest (user applications).
%define PR_RING0 0
%define PR (1 << 7) ; Present bit. This must be 1 for all valid selectors.

; Definition of flags
; the first 4 bits define the limit_19_16
%define AVL (1 << 4) ; Make whatever you want
%define SZ (1 << 6) ; Sz: Size bit. If 0 the selector defines 16 bit protected mode. If 1 it defines 32 bit protected mode. You can have both 16 bit and 32 bit selectors at once.
%define GR (1 << 7) ; Granularity bit. If 0 the limit is in 1 B blocks (byte granularity), if 1 the limit is in 4 KiB blocks (page granularity).

;--------------------------------------------------------------------
gdt:
    db 0, 0, 0, 0, 0, 0, 0, 0
gdt_cs:
    dw 0xFFFF, 0x0000
    db 0x0
    db EX | U0 | PR
    db 0xF | SZ | GR
    db 0x0
gdt_ds:
    dw 0xFFFF, 0x0000
    db 0x0
    db RW | U0 | PR
    db 0xF | SZ | GR
    db 0x0
gdt_lfb:
    dw 0xFFFF, 0x0000
    db 0x0
    db RW | U0 | PR
    db 0xF | SZ | GR
    db 0x0
gdt_sp:
    dw 0xFFFF, 0x0000
    db 0x0
    db RW | U0 | PR
    db 0xF | SZ | GR
    db 0x0
gdt_user_cs:
    dw 0xFFFF, 0x0000
    db 0x80
    db EX | U0 | PR | PR_RING3
    db 0xF | SZ | GR
    db 0x0
gdt_user_ds:
    dw 0xFFFF, 0x0000
    db 0x80
    db RW | U0 | PR | PR_RING3
    db 0xF | SZ | GR
    db 0x0
gdt_user_sp:
    dw 0xFFFF, 0x0000
    db 0x80
    db RW | U0 | PR | PR_RING3
    db 0xF | SZ | GR
    db 0x0
gdtend:
;--------------------------------------------------------------------
gdtptr:
    dw 0  ; limit
    dd 0  ; base
;--------------------------------------------------------------------

segment .text

GLOBAL init_gdt

%define BASE_LOCATION 0x800
%define REBASE(x) BASE_LOCATION + x - gdt

; *** This GDT is rebased at 0x800 ***
init_gdt:
    push ebp
    mov ebp, esp

    push esi
    push edi

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

    jmp 0x8:.next    ; reinit CS segment
.next:
    mov ax, 0x10     ; reinit DATA segments, DS, ES, FS et GS
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov ax, 0x20
    mov ss, ax

    pop edi
    pop esi

    pop ebp
ret
