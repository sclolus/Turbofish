
[BITS 32]

segment .data

pix_color: db 11

segment .text

GLOBAL draw_line
draw_line:
    push ebp
    mov ebp, esp

    push ebx
    push esi
    push edi
    push es

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

.begin_draw:
    cmp cx, dx       ;CONTROLE ORDRE X1 & X2 // Cas de symétrie, fait en sorte que x1 soit toujours supèrieur à x2
    JBE .no_symetric_mode_axx                                      ;Le prix à payer est aussi une inversion Y1 & Y2.
    xchg ecx, edx
.no_symetric_mode_axx:

; PHASE I : COMPARAISON ENTRE Y1 & Y2 ( Y2 & Y1 sont sur les bits de poid fort !)
    cmp edx, ecx
    ja .y2_sup_y1

; Y2 EST INFERIEUR A Y1 !                                                    |
; ----------------------------------------------------------------------------
    mov ebx, edx    ;Mise dans EBX de [Y2-Y1]:[X2-X1] soit DiffY:DiffX (pure)
    sub ebx, ecx

    mov eax, ebx    ;Extraction de [Y2-Y1] dans AX en vue d'une comparaison rapide.
    shr eax, 16

    not ax          ;Pour le calcul suivant DiffX doit être positif pour etre comparé d'ou l'inversion not & inc
    inc ax

    cmp ax, bx      ;Comparaison de DiffY avec DiffX
    jbe .dx_sup_dy_y2_inf_y1

    shl ebx, 17     ;dX=dX*2

    dec ax
    not ax

    mov bx, ax      ;e=DiffY()
    shr edx, 16     ;Mise de Y2 dans le bit du poid faible EDX à la place de X2 (qui ne sera pas utilisé ici)
    shl eax, 17     ;dY=e*2
    or  edx, eax

    jmp .begin_line_octant_3_7

.dx_sup_dy_y2_inf_y1:
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

    jmp .begin_line_octant_4_8

; Y2 EST SUPERIEUR A Y1 !                                                     |
; -----------------------------------------------------------------------------
.y2_sup_y1:
    mov ebx, edx    ;Mise dans EBX de [Y2-Y1]:[X2-X1] soit DiffY:DiffX (pure)
    sub ebx, ecx

    mov eax, ebx    ;Extraction de [Y2-Y1] dans AX en vue d'une comparaison rapide.
    shr eax, 16

    cmp ax, bx      ;Comparaison de DiffY avec DiffX
    jbe .dx_sup_dy_y2_sup_y1

    shl ebx, 17  ;dX=dX*2
    mov bx, ax   ;e=DiffY()
    shr edx, 16  ;Mise de Y2 dans le bit du poid faible EDX à la place de X2 (qui ne sera pas utilisé ici)
    shl eax, 17 ;dY=e*2
    or  edx, eax

    jmp .begin_line_octant_2_6

.dx_sup_dy_y2_sup_y1:
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
.begin_line_octant_1_5:
    cmp cx, dx
    ja .end_of_line
    call pixel
    ; Incrémentation logique de X1
    inc cx
    ; e = e -DY
    mov eax, edx
    shr eax, 16
    sub bx, ax
    jg .begin_line_octant_1_5
    ; Incrémentation eceptionnelle de Y1
    add ecx, 0x00010000
    ; e = e +DX
    mov eax, ebx
    shr eax, 16
    add bx, ax
    jmp .begin_line_octant_1_5

; -----------------------------------------------------------------------------------------------------------------
; Dans ce cas, EDX[faible] = Y2 pour faciliter la comparaison.
.begin_line_octant_2_6:
    ; Comparaison de Y1 avec Y2, Arret de la fonction si Y1 atteind Y2
    mov eax, ecx
    shr eax, 16
    cmp ax, dx
    ja .end_of_line
    call pixel
    ; Incrémentation logique de Y1
    add ecx, 0x00010000
    ; e = e -DX
    mov eax, ebx
    shr eax, 16
    sub bx, ax
    jg .begin_line_octant_2_6
    ; Incrémentation eceptionnelle de X1
    inc cx
    ; e = e +DY
    mov eax, edx
    shr eax, 16
    add bx, ax
    jmp .begin_line_octant_2_6

; -----------------------------------------------------------------------------------------------------------------
.begin_line_octant_4_8:
    cmp cx, dx
    ja .end_of_line
    call pixel
    ; Incrémentation logique de X1
    inc cx
    ; e = e -DY
    mov eax, edx
    shr eax, 16
    add bx, ax
    jg .begin_line_octant_4_8
    ; Décrémentation eceptionnelle de Y1
    sub ecx, 0x00010000
    ; e = e +DX
    mov eax, ebx
    shr eax, 16
    add bx, ax
    jmp .begin_line_octant_4_8

; -----------------------------------------------------------------------------------------------------------------
.begin_line_octant_3_7:
    ; Comparaison de Y1 avec Y2, Arret de la fonction si Y1 atteint Y2
    mov eax, ecx
    shr eax, 16
    cmp ax, dx
    jb .end_of_line
    ;{
    call pixel
    ; Décrémentation logique de Y1
    sub ecx, 0x00010000
    ; e = e -DX
    mov eax, ebx
    shr eax, 16
    sub bx, ax
    jg .begin_line_octant_3_7
    ;{
    ; Incrémentation eceptionnelle de X1
    inc cx
    ; e = e -DY (DY semble négatif !)
    mov eax, edx
    shr eax, 16
    sub bx, ax
    jmp .begin_line_octant_3_7
    ;}
;}
; -----------------------------------------------------------------------------------------------------------------
.end_of_line:
    pop es
    pop esi
    pop edi
    pop ebx

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
    stosb   ; Grave la couleur AL à l'endroit graphique pointé par ES:EDI

    pop ebx
ret
