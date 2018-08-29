
[BITS 32]

segment .data

edy: dd 0

test_meuh: dd 0xAABBCCDD

_graphical_char_paragraph:	;182
db 0b00000000
db 0b00000000
db 0b01111111
db 0b11011011
db 0b11011011
db 0b11011011
db 0b01111011
db 0b00011011
db 0b00011011
db 0b00011011
db 0b00011011
db 0b00011011
db 0b00000000
db 0b00000000
db 0b00000000
db 0b00000000

msg: db "Un message en memoire", 10, 0

segment .text

GLOBAL setCursorPosition    ; ->  Modifie la position du curseur de texte.
GLOBAL putchar
GLOBAL print

; Indique une nouvelle position en ligne et en colone pour le curseur.
setCursorPosition:
    push ebp
    mov ebp, esp

    mov eax, [ebp + 8]
    mov edx, [ebp + 12]

    shl eax,  3
    shl edx, 14

    add eax, edx
    mov [edy], eax

    mov esp, ebp
    pop ebp
ret

putchar:
    push ebp
    mov ebp, esp
    push ebx
    push esi
    push edi

    mov edi, [edy]

    mov ax, 0x18
    mov es, ax

    test edi, 0x0400
je _putchar_init
    add edi, 15360

_putchar_init:
    mov eax, [ebp + 8]

;    shl eax, 4
;    lea esi, [_print_graphical_char_begin + eax]

	mov ax, 0x10
	mov ds, ax

	mov [_graphical_char_paragraph + 0], byte 0b00000000
	mov [_graphical_char_paragraph + 1], byte 0b00000000
	mov [_graphical_char_paragraph + 2], byte 0b01111111
	mov [_graphical_char_paragraph + 3], byte 0b11011011
	mov [_graphical_char_paragraph + 4], byte 0b11011011
	mov [_graphical_char_paragraph + 5], byte 0b11011011
	mov [_graphical_char_paragraph + 6], byte 0b01111011
	mov [_graphical_char_paragraph + 7], byte 0b00011011
	mov [_graphical_char_paragraph + 8], byte 0b00011011
	mov [_graphical_char_paragraph + 9], byte 0b00011011
	mov [_graphical_char_paragraph + 10], byte 0b00011011
	mov [_graphical_char_paragraph + 11], byte 0b00011011
	mov [_graphical_char_paragraph + 12], byte 0b00000000
	mov [_graphical_char_paragraph + 13], byte 0b00000000
	mov [_graphical_char_paragraph + 14], byte 0b00000000
	mov [_graphical_char_paragraph + 15], byte 0b00000000

	lea esi, [_graphical_char_paragraph]

	mov eax, [ebp + 8]

    shl eax, 4
    lea esi, [0x00020000 + eax]



    mov dl, 3
    mov ch, 16                      ; Compteur HEIGHT à 0, il ira jusqu'à 16

_putchar_cycle_heigth:
      	lodsb                       ; La première ligne du caractère est chargée
        mov cl, 8                   ; Compteur WIDTH à 0, il ira jusqu'à 8

_putchar_cycle_width:                             ; Dispo EAX, EDX et ECX (16 bits forts) (ESI est armé sur le caractère en cours)
            test al, 0x80
        je tmp
			push eax
			mov al, 5
            stosb
            pop eax
            jmp _putchar_return_sequence

 tmp:
			push eax
			mov al, 3
            ;stosb
            inc edi
            pop eax

 _putchar_return_sequence:
            shl al, 1
            dec cl
            test cl, cl
        jne _putchar_cycle_width
        add edi, 1016               ; Préparation de EDI pour la prochaine ligne.
        dec ch
        test ch, ch
    jne _putchar_cycle_heigth

    sub edi, 16376
    mov [edy], edi

	pop edi
	pop esi
	pop ebx
    mov esp, ebp
    pop ebp
ret

print:
    push ebp
    mov ebp, esp
    push esi

    lea esi, [msg]
;    mov esi, [ebp + 8]
__print_loop__:
	xor eax, eax
   	lodsb
   	cmp al, 0x0
   	je __end_print__
	push eax
	call putchar
	add esp, 4
	jmp __print_loop__
__end_print__:
	pop esi
    pop ebp
ret
