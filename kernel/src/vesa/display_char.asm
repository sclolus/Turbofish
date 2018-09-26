[BITS 32]
segment .data

extern g_edi_offset

text_color: dd 0x00FFFFFF              ; default to blank

%include "fonts/alpha.asm"

segment .text

GLOBAL set_text_color
set_text_color:
    push ebp
    mov ebp, esp

    mov eax, [ebp + 8]
    mov [text_color], eax

    pop ebp
ret

%define CHAR_HEIGHT 16
%define CHAR_WIDTH 8
%define CHAR_SHL 4

GLOBAL display_char_24
display_char_24:
    push ebp
    mov ebp, esp

    push esi
    push edi

.putchar_init:
    mov eax, [ebp + 8]
    mov edi, [ebp + 12]

    shl eax, CHAR_SHL
    lea esi, [_print_graphical_char_begin + eax]

    mov ch, CHAR_HEIGHT         ; loop whith height

.putchar_cycle_heigth:
    lodsb                       ; 8 bits line of char is loaded
    mov dl, al
    mov cl, CHAR_WIDTH          ; loop with width

.putchar_cycle_width:
    test dl, 0x80
    je .putchar_blank

    mov eax, [text_color]
    stosw
    shr eax, 16
    stosb
    jmp .putchar_return_sequence

 .putchar_blank:
    add edi, 3

 .putchar_return_sequence:
    shl dl, 1
    dec cl
    test cl, cl
    jne .putchar_cycle_width
    add edi, dword [g_edi_offset] ; Prepare EDI for the next line
    dec ch
    test ch, ch
    jne .putchar_cycle_heigth

    pop edi
    pop esi

    pop ebp
ret

GLOBAL display_char_32
display_char_32:
    push ebp
    mov ebp, esp

    push esi
    push edi

.putchar_init:
    mov eax, [ebp + 8]
    mov edi, [ebp + 12]

    shl eax, CHAR_SHL
    lea esi, [_print_graphical_char_begin + eax]

    mov ch, CHAR_HEIGHT         ; loop whith height

.putchar_cycle_heigth:
    lodsb                       ; 8 bits line of char is loaded
    mov dl, al
    mov cl, CHAR_WIDTH          ; loop with width

.putchar_cycle_width:
    test dl, 0x80
    je .putchar_blank

    mov eax, [text_color]
    stosd
    jmp .putchar_return_sequence

 .putchar_blank:
    add edi, 4

 .putchar_return_sequence:
    shl dl, 1
    dec cl
    test cl, cl
    jne .putchar_cycle_width
    add edi, dword [g_edi_offset] ; Prepare EDI for the next line
    dec ch
    test ch, ch
    jne .putchar_cycle_heigth

    pop edi
    pop esi

    pop ebp
ret
