
[BITS 32]

segment .data

edy: dd 0

pix_color: db 9

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

segment .text

global iddle_mode

iddle_mode:
    hlt                 ; Cette instruction permet de faire passer le microprocesseur en mode d'arrêt.
JMP iddle_mode          ; Toutefois, le processeur peut quitter cet état lorsqu'une ligne matérielle RESET ou lorsqu'une interruption non-masquable (NMI) reçoit un signal.

global draw

draw:
    push ebp
    mov ebp, esp
    push ebx
    push esi
    push edi

    mov ecx, [ebp + 8]       ; Lit la valeur X1
    mov ebx, [ebp + 12]      ; Lit la valeur Y1
    shl ebx, 16
    add ecx, ebx

    mov edx, [ebp + 16]      ; Lit la valeur X2
    mov ebx, [ebp + 20]      ; Lit la valeur Y2
    shl ebx, 16
    add edx, ebx

    mov ax, 0x18
    mov es, ax

begin_draw:
    cmp cx, dx       ;CONTROLE ORDRE X1 & X2 // Cas de symétrie, fait en sorte que x1 soit toujours supèrieur à x2
    JBE no_symetric_mode_AXX                                      ;Le prix à payer est aussi une inversion Y1 & Y2.
    xchg ecx, edx
    no_symetric_mode_AXX:

; PHASE I : COMPARAISON ENTRE Y1 & Y2 ( Y2 & Y1 sont sur les bits de poid fort !)
    cmp edx, ecx
ja Y2_sup_a_Y1

; Y2 EST INFERIEUR A Y1 !                                                    |
; ----------------------------------------------------------------------------
    mov ebx, edx    ;Mise dans EBX de [Y2-Y1]:[X2-X1] soit DiffY:DiffX (pure)
    sub ebx, ecx

    mov eax, ebx    ;Extraction de [Y2-Y1] dans AX en vue d'une comparaison rapide.
    shr eax, 16

    not ax          ;Pour le calcul suivant DiffX doit être positif pour etre comparé d'ou l'inversion not & inc
    inc ax

    cmp ax, bx      ;Comparaison de DiffY avec DiffX
Jbe DX_superieur_a_DY_CAS_Y2_INFERIEUR_A_Y1

        shl ebx, 17     ;dX=dX*2

        dec ax
        not ax

        mov bx, ax      ;e=DiffY()
        shr edx, 16     ;Mise de Y2 dans le bit du poid faible EDX à la place de X2 (qui ne sera pas utilisé ici)
        shl eax, 17     ;dY=e*2
        or  edx, eax

    jmp BEGIN_LINE_Octant_3_7

DX_superieur_a_DY_CAS_Y2_INFERIEUR_A_Y1:
    ;Création de DiffX "pure" X2-X1, ici associé à E en BX  ;e = (x2 - x1)
    mov ax, dx
    sub ax, cx
    mov bx, ax

    ;Création du poid fort de ebx, ici e*2 soit (X2-X1)*2   ;DiffY = (e*2)
    shl ebx, 17
    or  ebx, eax

    ;Création de DiffY (y2-y1)*2    ;DY = (Y2 - Y1) X 2
    mov eax, edx
    sub edx, ecx
    shl edx, 1
    and eax, 0x0000FFFF
    and edx, 0xFFFF0000
    or  edx, eax

JMP BEGIN_LINE_Octant_4_8

; Y2 EST SUPERIEUR A Y1 !                                                     |
; -----------------------------------------------------------------------------
    Y2_sup_a_Y1:
    mov ebx, edx    ;Mise dans EBX de [Y2-Y1]:[X2-X1] soit DiffY:DiffX (pure)
    sub ebx, ecx

    mov eax, ebx    ;Extraction de [Y2-Y1] dans AX en vue d'une comparaison rapide.
    shr eax, 16

    cmp ax, bx      ;Comparaison de DiffY avec DiffX
Jbe DX_superieur_a_DY_CAS_Y2_SUPERIEUR_A_Y1

        shl ebx, 17  ;dX=dX*2
        mov bx, ax   ;e=DiffY()
        shr edx, 16  ;Mise de Y2 dans le bit du poid faible EDX à la place de X2 (qui ne sera pas utilisé ici)
        shl eax, 17 ;dY=e*2
        or  edx, eax

    Jmp BEGIN_LINE_Octant_2_6

DX_superieur_a_DY_CAS_Y2_SUPERIEUR_A_Y1:
;Création de DiffX "pure" X2-X1, ici associé à E en BX
    mov ax, dx
    sub ax, cx
    mov bx, ax

;Création du poid fort de ebx, ici e*2 soit (X2-X1)*2
    shl ebx, 17
    or  ebx, eax

;Création de DiffY (y2-y1)*2
    mov eax, edx
    sub edx, ecx
    shl edx, 1
    and eax, 0x0000FFFF
    and edx, 0xFFFF0000
    or  edx, eax

; -----------------------------------------------------------------------------------------------------------------
; Dans ce cas, Y2 est occulté et X1, X2 prennent des registres directes.
BEGIN_LINE_Octant_1_5:
    cmp cx, dx
ja end_of_line
        CALL pixel
        ; Incrémentation logique de X1
        inc cx
        ; e = e -DY
        mov eax, edx
        shr eax, 16
        sub bx, ax
    jg BEGIN_LINE_Octant_1_5
            ; Incrémentation eceptionnelle de Y1
            add ecx, 0x00010000
            ; e = e +DX
            mov eax, ebx
            shr eax, 16
            add bx, ax
        jmp BEGIN_LINE_Octant_1_5

; -----------------------------------------------------------------------------------------------------------------
; Dans ce cas, EDX[faible] = Y2 pour faciliter la comparaison.
BEGIN_LINE_Octant_2_6:
    ; Comparaison de Y1 avec Y2, Arret de la fonction si Y1 atteind Y2
    mov eax, ecx
    shr eax, 16
    cmp ax, dx
ja end_of_line
        CALL pixel
        ; Incrémentation logique de Y1
        add ecx, 0x00010000
        ; e = e -DX
        mov eax, ebx
        shr eax, 16
        sub bx, ax
    jg BEGIN_LINE_Octant_2_6
            ; Incrémentation eceptionnelle de X1
            inc cx
            ; e = e +DY
            mov eax, edx
            shr eax, 16
            add bx, ax
        jmp BEGIN_LINE_Octant_2_6

; -----------------------------------------------------------------------------------------------------------------
BEGIN_LINE_Octant_4_8:
    cmp cx, dx
ja end_of_line
        CALL pixel
        ; Incrémentation logique de X1
        inc cx
        ; e = e -DY
        mov eax, edx
        shr eax, 16
        add bx, ax
    jg BEGIN_LINE_Octant_4_8
            ; Décrémentation eceptionnelle de Y1
            sub ecx, 0x00010000
            ; e = e +DX
            mov eax, ebx
            shr eax, 16
            add bx, ax
        jmp BEGIN_LINE_Octant_4_8

; -----------------------------------------------------------------------------------------------------------------
BEGIN_LINE_Octant_3_7:
    ; Comparaison de Y1 avec Y2, Arret de la fonction si Y1 atteint Y2
    mov eax, ecx
    shr eax, 16
    cmp ax, dx
jb end_of_line
;{
        CALL pixel
        ; Décrémentation logique de Y1
        sub ecx, 0x00010000
        ; e = e -DX
        mov eax, ebx
        shr eax, 16
        sub bx, ax
    jg BEGIN_LINE_Octant_3_7
        ;{
            ; Incrémentation eceptionnelle de X1
            inc cx
            ; e = e -DY (DY semble négatif !)
            mov eax, edx
            shr eax, 16
            sub bx, ax
        jmp BEGIN_LINE_Octant_3_7
         ;}
;}
; -----------------------------------------------------------------------------------------------------------------
end_of_line:
    pop esi
    pop edi
	pop ebx
    mov esp, ebp
    pop ebp
ret

; -----------------------------------------------------------------------------------------------------------------
pixel:
    push ebx

    mov eax, ecx

    shr eax, 16
    shl eax, 10

    mov ebx, ecx
    and ebx, 0x0000FFFF
    add eax, ebx

    mov edi, eax
    mov esi, pix_color

    mov al, byte [pix_color]
 ;;   mov al, 9

    stosb   ; Grave la couleur AL à l'endroit graphique pointé par ES:EDI

    pop ebx
ret


GLOBAL setCursorPosition    ; ->  Modifie la position du curseur de texte.
GLOBAL putchar_f

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

putchar_f:
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

end_t:
	jmp $
