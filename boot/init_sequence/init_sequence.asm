%define BASE    0x100  ; 0x0100:0x0 = 0x1000
%define KSIZE   50     ; nombre de secteurs a charger

[BITS 16]
[ORG 0x0]

segment .bss  ; données non initialisées.

segment .text ; SEGMENT DE CODE

jmp start                    ;Saut nécessaire pour l'inclusion de fichier tière en début de fichier. L'execution du code commencera ainsi à Start et le programme ne plantera pas du coup !
%include "../tools/16b_system.asm"
%include "../tools/16b_screen.asm"
start:

cli
    mov ax, cs              ; Réinitialise les segments de données sur celui de code actuel.        Le nouveau segment de code est le 0x7E00.
    mov ds, ax
    mov es, ax              ; La pile précédente déclarée entre 8000:0000 et 8000:2000 convient encore très bien, innutile de la redéfinir ici.
sti

; recuparation de l'unite de boot
    mov [bootdrv], dl

	call check_vesa_capability
    cmp al, 0x4F
	jne no_vesa_card

    call copy_graphic_modes_buffer

    call set_vesa_graphic

    push 0x09
    call set_text_color
	add sp, 2

    push welcome_msg
    call print
	add sp, 2

    push 0x03
    call set_text_color
 	add sp, 2

    push vesa_mode_status
    call print
	add sp, 2

    push 0x02
    call set_text_color
	add sp, 2

    push YES_symbol
    call print
	add sp, 2

    push 0x03
    call set_text_color
	add sp, 2

    push checking_mode_msg
    call print
	add sp, 2

    push 0x02
    call set_text_color
	add sp, 2

    push YES_symbol
    call print
	add sp, 2

    push long_regular_line
    call print
	add sp, 2

    mov ax, cs
    mov ds, ax
    mov si, _graphic_modes_list

VIEW_COMPATIBLE_MODES:  ; Le premier mode logique que la CG propose est 0x0101, un 640*480*256colors
    lodsw               ; Idem charge le mode codé en "word" sur 2 bytes, le met dans AX & incrémente SI de 2

    cmp ax, 0xFFFF      ; Si AX = 0xFFFF c'est que nous sommes arrivés à la fin de la liste !
je End_of_research_mode

    push ds
    push si

    push no_text
    call view_hex_register
 	add sp, 2

    push many_spaces
    call print
    add sp, 2

    pop si
    pop ds

jmp VIEW_COMPATIBLE_MODES

End_of_research_mode:
    push jump_line
    call print
 	add sp, 2

    push long_regular_line
    call print
 	add sp, 2

    push 0x03
    call set_text_color
 	add sp, 2

    push linear_frame_buffer_issue
    call print
 	add sp, 2

    push 0x02
    call set_text_color
 	add sp, 2

    mov ax, [flat_Memory]       ; bits de poids faible ?
    mov bx, [flat_Memory +2]    ; bits de poids fort   ?

    push bx

    push no_text
    CALL view_hex_register
    add sp, 2

    pop ax

    push deux_petits_points
    CALL view_hex_register
	add sp, 2

    push jump_line
    call print
    add sp, 2

    push 0x03
    call set_text_color
    add sp, 2

    push loading_kernel
    call print
    add sp, 2

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

; CONFIGURATION VESA pour la gdt: Nous utiliserons le LFB. Il est donc necessaire de renseigner le descripteur de segment LFB de la gdt.
    mov ax, [flat_Memory]
	mov [gdt_lfb + 2], ax           ; Extraction des 16 bits de poid faible de la LFB et inscription dans BASE 0-->15 de la gtd_lfb

	mov ax, [flat_Memory + 2]       ; Extraction des 16 bits de poid fort de la LFB:
	mov [gdt_lfb + 4], al           ;   - Mise des 8 bits faibles dans BASE 16 ---> 23 de la gtd_lfb
	mov [gdt_lfb + 7], ah           ;   - Mise des 8 bits forts dans BASE 24 ----> 31 de la gtd_lfb


    push 0x02
    call set_text_color
    add sp, 2

    push YES_symbol
    call print
	add sp, 2

    call write_cursor_position_for_32b_kernel


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

    jmp dword 0x8:0x1000    ; reinitialise le segment de code

disk_fatal_error:
    push 0x04
    call set_text_color
	add sp, 2

    push error_disk_msg
    call print
    add sp, 2

    call freeze_process

no_vesa_card:
    push no_vesa_issue
    call print_Text_Mode
    add sp, 2

    call freeze_process

segment .data ; données initialisées.

welcome_msg: db "                                                   BURG 2 80x86 MODE", 10, 10, 0
no_vesa_issue: db "Votre carte graphique ne supporte pas le mode VESA: Switch to Iddle Mode", 13, 10, 0
vesa_mode_status: db "Checking for VESA capability:", 0
checking_mode_msg: db "Checking for compatible mode:", 0

linear_frame_buffer_issue: db "Location of Linear Frame Buffer: ", 0
loading_kernel: db "Loading Kernel:", 0
error_disk_msg: db " Unable to load kernel. Switch to Iddle Mode", 10, 0
gdt_loading: db "Initialize bootloader 32B-GDT (Global Description Table):", 0

YES_symbol: db " SUCCESS", 10, 0
no_text: db "",0
deux_petits_points: db ":", 0
jump_line: db 10, 0
many_spaces: db "    ", 0
long_regular_line: db "--------------------------------------------------------------------------------------------------------------------------------", 0

;--------------------------------------------------------------------
bootdrv:  db 0
;--------------------------------------------------------------------

gdt:
    db 0, 0, 0, 0, 0, 0, 0, 0                                       ; DESCRIPTEUR NULL

gdt_cs: ; SEGMENT DE CODE STANDARD
    dw 0xFFFF, 0x0000                                               ; limite 15-0: 0xFFFF    base 15-0: 0x0000                          -> base  (32b) 0x0000:0000
    db 0x0, 0b10011011, 0b11011111, 0x0                             ; base 16-23: 0x00       limite 16-19: 0b1111    base 24-31: 0x00   -> limit (20b) 0xF:FFFF
                  ;type
gdt_ds: ; SEGMENT DE DONNES STANDARD
    dw 0xFFFF, 0x0000                                               ; limite 15-0: 0xFFFF    base 15-0: 0x0000                          -> base  (32b) 0x0000:0000
    db 0x00, 0b10010011, 0b11011111, 0x00                           ; base 16-23: 0x00       limite 16-19: 0b1111    base 24-31: 0x00   -> limit (20b) 0xF:FFFF
                  ;type
gdt_lfb: ; SEGMENT DE LA LFB, COMME UN SEGMENT DE DONNEES sauf que la BASE est mise à celle de la LFB de la carte graphique !
    dw 0xFFFF, 0x0000                                               ; limite 15-0: 0xFFFF    base 15-0: 0x0000                          -> base   (32b) 0xFC00:0000
    db 0x00, 10010011b, 11011111b, 0xFC                             ; base 16-23: 0x00       limite 16-19: 0b1111    base 24-31: 0xFC   -> limite (20b) 0xF:FFFF
;--------------------------------------------------------------------
gdtptr:
    dw 0  ; limite
    dd 0  ; base
;--------------------------------------------------------------------
gdtend:

times 7168-($-$$) db 144        ; NOP
