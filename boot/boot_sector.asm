[BITS 16]
[ORG 0x7C00]

%define KERNEL_OFFSET_ON_DISK       10
%define KERNEL_BASE_MEMORY          0x1000              ; on 16bits 0x1000:0x0 = 0x1000:0000. on 32bits 0x0001:0000
%define KERNEL_NB_SECTORS           128                 ; number of sectors for kernel to load, one sector is 512o size

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

disk_fatal_error:
    mov si, cannot_load_from_disk
    call afficher
    jmp $

start:
	cli

	lidt [bios_idt]

	sti
    mov si, msgDebut
    call afficher

    mov ax, 0x8000  ; stack en 0xFFFF
    mov ss, ax
    mov sp, 0xf000

; recuparation de l'unite de boot
    mov [bootdrv], dl

	mov ax, 0x8600
	mov ecx, 10
	mov edx, 0
	int 15h

; affiche un msg
    mov si, msgDebut
    call afficher

    jmp $

; reset drive
    xor ax, ax
    int 0x13

; loading of kernel
    push es
    mov ax, KERNEL_BASE_MEMORY
    mov es, ax
    xor bx, bx
    mov ah, 2
    mov al, KERNEL_NB_SECTORS
    xor ch, ch
    mov cl, KERNEL_OFFSET_ON_DISK
    xor dh, dh
    mov dl, [bootdrv]
    int 0x13
    jc disk_fatal_error
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

    jmp dword 0x8:0x1000 << 4    ; reinitialise le segment de code

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

bootdrv:  db 0
msgDebut: db "Loadig kernel...", 13, 10, 0
cannot_load_from_disk: db "Cannot load disk", 13, 10, 0
;--------------------------------------------------------------------

;; NOP jusqu'a 510
times 510-($-$$) db 144
dw 0xAA55
