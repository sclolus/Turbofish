
[BITS 32]

segment .data

cursor_location: dd 0
text_color: db 2              ; default to green

segment .text

GLOBAL set_cursor_position
GLOBAL set_text_color
GLOBAL putchar
GLOBAL asm_printk

; Indique une nouvelle position en ligne et en colone pour le curseur.
set_cursor_position:
    push ebp
    mov ebp, esp

    mov eax, [ebp + 8]
    mov edx, [ebp + 12]

    shl eax,  3
    shl edx, 14

    add eax, edx
    mov [cursor_location], eax

    mov esp, ebp
    pop ebp
ret

set_text_color:
    push ebp
    mov ebp, esp
    mov eax, [ebp + 8]
    mov [text_color], al
    pop ebp
ret

putchar:
    push ebp
    mov ebp, esp
    push ebx
    push esi
    push edi

    mov edi, [cursor_location]

    mov ax, 0x18
    mov es, ax

    test edi, 0x0400
    je .putchar_init
    add edi, 15360

.putchar_init:
    mov eax, [ebp + 8]

    shl eax, 4
    lea esi, [0x00009000 + eax]

    mov ax, 0x10
    mov ds, ax

    mov dl, 3
    mov ch, 16                  ; Compteur HEIGHT à 0, il ira jusqu'à 16

.putchar_cycle_heigth:
    lodsb                       ; La première ligne du caractère est chargée
    mov cl, 8                   ; Compteur WIDTH à 0, il ira jusqu'à 8

.putchar_cycle_width:           ; Dispo EAX, EDX et ECX (16 bits forts) (ESI est armé sur le caractère en cours)
    test al, 0x80
    je .tmp
    push eax
    mov al, byte [text_color]
    stosb
    pop eax
    jmp .putchar_return_sequence

 .tmp:
    inc edi

 .putchar_return_sequence:
    shl al, 1
    dec cl
    test cl, cl
    jne .putchar_cycle_width
    add edi, 1016               ; Préparation de EDI pour la prochaine ligne.
    dec ch
    test ch, ch
    jne .putchar_cycle_heigth

    sub edi, 16376
    mov [cursor_location], edi

    pop edi
    pop esi
    pop ebx
    mov esp, ebp
    pop ebp
ret

asm_printk:
    push ebp
    mov ebp, esp
    push esi
    mov esi, [ebp + 8]
.print_loop:
    xor eax, eax
    lodsb
    cmp al, 0x0
    je .end_print
    push eax
    call putchar
    add esp, 4
    jmp .print_loop
.end_print:
    pop esi
    pop ebp
ret
