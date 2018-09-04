
[BITS 32]

segment .data

extern g_edi_offset

text_color: db 10              ; default to green

%include "fonts/alpha.asm"

segment .text

GLOBAL set_text_color
GLOBAL display_char

set_text_color:
    push ebp
    mov ebp, esp
    mov eax, [ebp + 8]
    mov [text_color], al
    pop ebp
ret

display_char:
    push ebp
    mov ebp, esp

    push esi
    push edi

    mov ax, 0x18
    mov es, ax

.putchar_init:
    mov eax, [ebp + 8]
    mov edi, [ebp + 12]

    shl eax, 4
    lea esi, [_print_graphical_char_begin + eax]

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
    add edi, dword [g_edi_offset]               ; Préparation de EDI pour la prochaine ligne.
    dec ch
    test ch, ch
    jne .putchar_cycle_heigth

    mov ax, 0x10
    mov es, ax

    pop edi
    pop esi

    pop ebp
ret
