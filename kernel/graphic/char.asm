
[BITS 32]

segment .data
hexvalue32: times 4 db "0"
          db ":"
          times 4 db "0"
          db 0

hexvalue16: times 4 db "0"
          db 0

colour: db 1
cursor_color: db 10

edy: dd 0

decim_output: times 12 db 0

_graphical_char_test_O:	;47
times 100 db 0b10001001
db 0b00000000
db 0b00000000
db 0b00000000
db 0b00000010
db 0b00000110
db 0b00001100
db 0b00011000
db 0b00110000
db 0b01100000
db 0b11000000
db 0b10000000
db 0b00000000
db 0b00000000
db 0b00000000
db 0b00000000

gogogo: db "azerty", 10, 0

test_meuh: dd 0xAABBCCDD

segment .text
GLOBAL print                ; ->  Ecrit sur l'écran une chaine de caractère passée en argument.
GLOBAL setTextColor         ; ->  Modifie la couleur du texte.
GLOBAL setCursorPosition    ; ->  Modifie la position du curseur de texte.
GLOBAL query_old_cursor_position    ; -> Initialise la position du curseur de texte en fonction des anciennes valeurs dans le 16b_screen.asm du Kernel 16 bits, on lui donne en argmuent l'addresse de ces valeurs.
GLOBAL getCursorPosition
GLOBAL putchar
GLOBAL putchar_f
GLOBAL jump_line
GLOBAL backspace
GLOBAL show_cursor
GLOBAL hide_cursor

;%include "../polices/alpha.asm"

; Récupère la position en lignes/colones du curseur.
getCursorPosition:
    push ebp
    mov ebp, esp

    mov eax, [edy]
    mov edx, eax

    and eax, 0x03FF
    shr eax, 3

    mov ecx, [ebp+8]
    mov [ecx], al

    shr edx, 14

    mov ecx, [ebp+12]
    mov [ecx], dl

    mov esp, ebp
    pop ebp
ret

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

; Récupère les anciennes coordonnées du curseur venant du kernel 16 bits, elles sont traduites dans EDY.
query_old_cursor_position:
    push ebp
    mov ebp, esp

    xor eax, eax
    xor edx, edx

    mov esi, [ebp+8]
    mov al, [esi]
    shl eax, 14             ; Multiplication du nombre de ligne par 16384       (16 lignes de 1024 pixels)

    inc esi
    mov dx, [esi]
    and dx, 0x03FF

    add eax, edx
    mov [edy], eax          ; Sauvegarde de la valeur Pixel dans EDY

    mov esp, ebp
    pop ebp
ret

setTextColor:
    push ebp
    mov ebp, esp

    mov eax, [ebp+8]
    mov [colour], al

    mov esp, ebp
    pop ebp
ret

; Print a les capacités d'afficher des substring, mais pas des substrings recursifs dans les substrings. Le caractère % ne sera pas affiché tel quel s'il est suivi de i, x, ou s (go subtrings)
; TODO Il serait bien que PRINT a l'avenir renvoit le nombres de caractères imprimés ! Ainsi que la mise en place d'un escape char \ avant le symbole %
print:
    push ebp
    mov ebp, esp
    push es                             ; Sauvegarde de ES

    mov ax, 0x18                        ; Modification de ES
    mov es, ax

    mov esi, [ebp + 8]                    ; Récupération de la chaine de caractère dans la pile.

    mov edi, [edy]

    mov edx, 0x000C0000                 ; Mise de 12 dans les 16 bits forts de EDX afin de pouvoir être utilisé dans l'affichage des sous-chaines.
    add dl, [colour]                    ; SET initial color -> registre DL          BUG NE PAS FAIRE "ADD EDX, [COLOUR]" car COLOUR serait pris pour un mot de 4 octets, ce qui créera des problêmes !

; ***************************************** PRELIMINAIRE - OBSERVE SI LE CURSEUR N'EST PAS TOUT A DROITE DE L'ECRAN (ligne suivante) ********************************************
BEGIN_PRINT:
        test edi, 0x0400                ; TEST applique un ET logique sans modifier les opérandes, ici, TEST regarde si les derniers 11 bits de EDI n'exprimeraient pas 1024 ou 0x0400
    je get_current_character
limit_width_jump_line:
        add edi, 15360                  ; Rattrapage de EDI en 1024*768*8 sur la ligne suivante lorsque ses 11 bits faibles valent 1024 (0x400)     1024*16 = 16384 | 16384-(1024)= 15360 (valeur à rajouter)
        ;mov [edy], edi
    jmp BEGIN_PRINT
; ***************************************** EXTRACTION DU CARACTERE COURANT DE LA CHAINE PASSEE EN ARGUMENT ***********************************************************************
get_current_character:
        xor eax, eax
        lodsb

        cmp al, 0x25
        jne end_of_string_test

        test edx, 0x01000000            ; Verrouille la fonction subtring dans les substrins elles-mêmes, si le caractère % est dans une substring, il sera affiché comme tel !
        jne end_of_string_test          ; Ici est consulté le FLAG substring justement.

; ***************************************** TRAITEMENT DU CARACTERE SPECIAL % QUI INDIQUE UNE SOUS-CHAINE A AFFICHER **************************************************************
special_string_char:
        mov ax, 0x10                    ; Remise sur le segment de donnée.
        mov es, ax

        lodsb                           ; Extraction du caractère suivant le % et mise de ce dernier dans CL
        mov cl, al

        mov eax, edx                    ; Le registe EDX est comme un super registre: 31--->24 Drapeau de sous-chaine  23--->16 Hauteur pile des sous chaines 15--->0 Couleur du texte
        and eax, 0x00FF0000
        shr eax, 16                     ; Récupération des 16 bits de poid fort d'EDX, ils expriment l'endoit dans la pile ou chercher les sous-chaines. (1er -> 12, 2eme -> 16 etc...)
        mov eax, [ebp + eax]            ; Recherche de la valeur suivante passée en argument du print principal.

            cmp cl, 'i'                     ; le param I demande un entier décimal
        je decim
            cmp cl, 's'                     ; Le param S demande une sous-chaine de caractère
        je sub_string
            cmp cl, 'x'                     ; Le param X demande un hexadécimal
        je hex32
            cmp cl, 'h'
        je hex16

        mov ax, 0x18                        ; Remise sur le segment du LFB
        mov es, ax

        dec esi                             ; DEFAULT: Aucun des caractères après le symbole % n'a de sens ! SORT DE LE FONCTION, décrémente ESI et remplace AL par %
        mov al, 0x25
jmp end_of_string_test

; **************  Ecrit sur l'écran les valeurs hexadécimales d'un registre 32bis *********************
hex32:
    push edi

    mov ebx, eax

    mov cl, 28
    mov edi, hexvalue32
extract_one_hex32:
    mov eax, ebx
    shr eax, cl
    and eax, 0xF
    cmp al, 10              ; Si CL est > à 9, nous devons afficher un caractère, d'ou la petite augmentation de 7 pour la conversion en char (cf. table ASCII)
jl associate_char32
ajout_pour_caractere_A_F32:
    add al, 7
associate_char32:
    add al, 48

    cmp cl, 12
jne continue_hex32            ; Exception pour le symbole ':' au millieu de la valeur hexadécimale.
    inc edi
continue_hex32:
    stosb
    sub cl, 4
jns extract_one_hex32

    mov eax, hexvalue32

    pop edi
jmp sub_end
; **************  Ecrit sur l'écran les valeurs hexadécimales d'un registre 16bis *********************
hex16:
    push edi

    mov ebx, eax

    mov cl, 12
    mov edi, hexvalue16
extract_one_hex16:
    mov eax, ebx
    shr eax, cl
    and eax, 0xF
    cmp al, 10              ; Si CL est > à 9, nous devons afficher un caractère, d'ou la petite augmentation de 7 pour la conversion en char (cf. table ASCII)
jl associate_char16
ajout_pour_caractere_A_F16:
    add al, 7
associate_char16:
    add al, 48

continue_hex16:
    stosb
    sub cl, 4
jns extract_one_hex16

    mov eax, hexvalue16

    pop edi
jmp sub_end
; *********************  Affiche en décimal la variable passée en argument *****************************
decim:
    push edi
    push edx

    xor edx, edx
    mov ecx, 11
    mov edi, (decim_output+10)

    mov ebx, 10         ; Algorythme des div successives: 321/10 = 32 pop 1 : 32/10 = 3 pop 2 : 3/10 = 0 pop 3 => 123 soit 321 à l'envers -> On divise par 10, on retient le reste, on l'écrit sur la chaine, puis
                        ; le résultat suivant est encore divisé par 10, on réécrit le reste puis on redivise le résultat par 10 etc...
bump_digit:
    div ebx             ; DIV reg/mem32 -> Division naturel de EDX:EAX par le contenu d'un emplacement mémoire ou registre 32 bits et entrepose le quotient dans EAX et restant dans EDX.
                        ; Sauve le reste dans la pile. Par ex, la première opération pour EAX=125 sera 125 div 10 soit reste 5, c'est cette dernière valeure 5 que l'on doit écrire dans la chaine de sortie.

    add dl, '0'         ; Formatage ASCII par le caractère '0'
    mov [es:edi], dl    ; Inscription du chiffre

    dec edi             ; L'inscription se fait à partir de la fin de la chaine decim_output, d'ou la décrémentation d'EDI ensuite !
    dec cl              ; CL représente l'inverse par 10 du nombre de divisions par 10 qu'il y a eu, soit la taille du chiffre à afficher.

    xor edx, edx
    or eax, eax         ; tester s'il reste quelque chose dans ax, si EAX==0, c'est terminé !
jne bump_digit

decim_final:
    lea eax, [decim_output + ecx]   ; Load Effective Address, remplace 2 lignes telles: mov eax, decim_output ET add eax, ecx

    pop edx
    pop edi
jmp sub_end
; *********************  Cas d'une sous-chaine de caractère ********************************************
sub_string:
        ; RIEN À FAIRE, LA LIGNE PRECEDENTE mov eax, [ebp + eax] SUFFIT DEJA POUR PLACER LE POINTEUR ESI SUR LA SOUS-CHAINE !

sub_end:
        push esi                    ; Récupération de ESI indiquant l'endroit ou nous sommes dans la chaine principale, et mise dans la pile de ce dernier avec le push ! (on retournera dessus une fois la sous-chaine finie)
        mov esi, eax                ; Mise en place du nouvel ESI courant de la sous-chaine. Les fonctions ci-dessus renvoyaient tout dans EAX

        add edx, 0x01040000         ; Ajout de 4 sur EDX en 23-->16 pour la profondeur de la pile && 1 en 31--->24 comme un drapeau de sous-chaine présente (afin d'éviter que le 0 de la sous-chaine termine print)

        mov ax, 0x18                ; Remise sur le segment du LFB
        mov es, ax
jmp BEGIN_PRINT

; *************************************** TEST DE FIN DE LA CHAINE - PEUT-ETRE AUSSI UNE FIN DE SOUS CHAINE **********************************************************
end_of_string_test:
        test al, al                     ; TEST applique un ET logique sans modifier les opérandes.
    jne natural_jump_line_test

    test edx, 0x01000000                ; Consulte EDX pour voir si le flag 'sous-chaine' y est present.
je final_end_print
    pop esi                             ; Retrouve la valeur ESI de la chaine principale, (mise dans la pile quand on a basculé sur l'affichage d'une sous-chaine)
    and edx, 0x00FFFFFF                 ; Efface le flag sous-chaine EDX
jmp BEGIN_PRINT                         ; Reprend normalement l'affichage de la chaine principale !

final_end_print:
    pop es                              ; Restauration de ES

    mov [edy], edi

    mov esp, ebp
    pop ebp
ret

; *************************************** TEST DE LA PRESENCE DU CARACTERA 0xA DE SAUT DE LIGNE **********************************************************************
natural_jump_line_test:
        cmp al, 0x0A                    ; test du caractère 0x0A -> saut de ligne
    jne test_EDI_overflow
        mov eax, edi                    ; Chaque ligne de caractère en 1024*768*8 représente 16384 en EDI 0x4000
        and eax, 0x2FFF                 ; Pour passer à la ligne suivante, le décalage EDI par rapport à 16384 est calculé par un soustraction de ses 14 bis faibles. 0x4000 - EDI(masque 0x2FFF)
        mov ebx, 0x4000                 ; Le résultat de cette soustraction est enfin ajouté à EDI, ainsi, la nouvelle valeure EDI representera la ligne de caractère suivante.
        sub ebx, eax
        add edi, ebx
jmp BEGIN_PRINT

; ************************************** TEST SI LES CARACTERES NE SONT PAS AU BAS DE L'ECRAN, ET DONC S'IL NE FAUT PAS FAIRE SCROLLER L'ECRAN ********************
test_EDI_overflow:
        cmp edi, 786432
    jb get_alpha_value
manage_screen:                          ; -> Défilement de l'écran en fonction du nombre de lignes.
        mov ebx, edi                    ; sauvegarde de l'ancienne valeur EDI dans EBX
        mov edi, (786432 - 16384)       ; mise en place du curseur sur la dernière ligne.

        mov dh, al                      ; sauvegarde du registre AL dans DH
        push esi
        push edi

        mov ax, 0x18
        mov ds, ax

        sub ebx, (786432 - 16384)       ; ebx exprime en quelque sorte le nombre de lignes à slider fois 16384
        and ebx, 0xFFFFFC00             ; 16b 0000 0011 1111 1111

        mov esi, ebx                    ; placement sur la mémoire graphique à la ligne désignée à la ligne EBX
        xor edi, edi
        mov ecx, 786432
        sub ecx, ebx
        shr ecx, 2
        rep movsd                       ; Copie des pixels

        mov edi, 786432
        sub edi, ebx
        mov ecx, 786432
        sub ecx, edi
        shr ecx, 2
        xor eax, eax
        rep stosd                       ; L'écran du bas est nettoyé par des pixels noirs !

        mov ax, 0x10
        mov ds, ax

        mov al, dh                      ; restauration du registre AL via DH
        pop edi
        pop esi
; **************************************************** AFFICHAGE D'UN SEUL CARACTERE ***********************************************************
get_alpha_value:
        mov ebx, esi                    ; Sauvegarde du registre ESI dans EBX
 ;       mov esi, _print_graphical_char_begin
        shl eax, 4
        add esi, eax
        mov ch, 16                      ; Compteur HEIGHT à 0, il ira jusqu'à 16
print_char:
            lodsb                       ; La première ligne du caractère est chargée
            mov cl, 8                   ; Compteur WIDTH à 0, il ira jusqu'à 8
print_line:                             ; Dispo EAX, EDX et ECX (16 bits forts) (ESI est armé sur le caractère en cours)
                test al, 0x80
            je return_sequence
draw_pixel:
                mov [es:edi],dl         ; Remplace avantageusement un stosb lorsque l'on a pas interet à incrémenter forcément à ce point la valeur EDI
return_sequence:
                inc edi
                shl al, 1
                dec cl
                test cl, cl
            jne print_line
            add edi, 1016               ; Préparation de EDI pour la prochaine ligne.
            dec ch
            test ch, ch
        jne print_char
        mov esi, ebx                    ;  Restauration du registre ESI via EBX
        sub edi, 16376                  ;  soustraction à EDI de (16*1024) + 8
    jmp BEGIN_PRINT


; --- jump line cause EDY at end                                                                        OK
; --- backspace doit passer ici pour supprimer un caractère (astuce on peut se servir du curseur)       OK
; --- slide screen  (astuce utilisant le curseur aussi)                                                 OK

putchar_f:
putchar:                                ; Utilisable seulement dans le contexte d'une interruption car les registres ne sont pas restaurés dans cette fonction.
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

	lea esi, [_graphical_char_test_O]
	lodsb

;	mov [test_meuh], dword 0xAABBCCDD
	mov eax, [test_meuh]
	cmp eax, 0xAABBCCDD
;	cmp eax, 0xDDCCBBAA
	je end_t

	lodsb
	cmp al, 0b10001001
	je end_t
	lodsb
	cmp al, 0b10001001
	je end_t
	lodsb
	cmp al, 0b10001001
	je end_t
	lodsb
	cmp al, 0b10001001
	je end_t
	lodsb
	cmp al, 0b10001001
	je end_t
	lodsb
	cmp al, 0b10001001
	je end_t
	lodsb
	cmp al, 0b10001001
	je end_t
	lodsb
	cmp al, 0b10001001
	je end_t
	lodsb
	cmp al, 0b10001001
	je end_t
	lodsb
	cmp al, 0b10001001
	je end_t


	lea esi, [_graphical_char_test_O]

    mov dl, [cursor_color]                       ; color

    mov ch, 16                      ; Compteur HEIGHT à 0, il ira jusqu'à 16

_putchar_cycle_heigth:
	;	push eax

		;mov al, 0b10001001

      	lodsb                       ; La première ligne du caractère est chargée

		cmp al, 0b10001001
		je end_t

      	;mov al, 0b11011011

        mov cl, 8                   ; Compteur WIDTH à 0, il ira jusqu'à 8
_putchar_cycle_width:                             ; Dispo EAX, EDX et ECX (16 bits forts) (ESI est armé sur le caractère en cours)
            test al, 0x80
        je tmp
;_putcher_draw_pixel:
            ;mov [es:edi],dl         ; Remplace avantageusement un stosb lorsque l'on a pas interet à incrémenter forcément à ce point la valeur EDI

			push eax
			mov al, 5
            stosb
            pop eax
            jmp _putchar_return_sequence
 tmp:
			push eax
			mov al, 3
            stosb
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

jump_line:
    mov edi, [edy]
    shr edi, 14
    inc edi
    shl edi, 14
_jump_line_test_edy_overflow:
    cmp edi, 786432
jb _jump_line_REG
    mov ax, 0x18
    mov ds, ax
    mov es, ax

    mov ebx, edi                     ; sauvegarde de l'ancienne valeur EDI dans EBX

    sub ebx, (786432 - 16384)       ; ebx exprime en quelque sorte le nombre de lignes à slider fois 16384
    and ebx, 0xFFFFFC00             ; 16b 0000 0011 1111 1111

    mov esi, ebx                    ; placement sur la mémoire graphique à la ligne désignée à la ligne EBX
    xor edi, edi
    mov ecx, 786432
    sub ecx, ebx
    shr ecx, 2
    rep movsd                       ; Copie des pixels

    mov edi, 786432
    sub edi, ebx
    mov ecx, 786432
    sub ecx, edi
    shr ecx, 2
    xor eax, eax
    rep stosd                       ; L'écran du bas est nettoyé par des pixels noirs !

    mov ax, 0x10
    mov ds, ax
    mov es, ax

    mov edi, (786432 - 16384)       ; mise en place du curseur sur la dernière ligne.
_jump_line_REG:
    mov [edy], edi
ret

backspace:
    mov edi, [edy]
    sub edi, 8

    test edi, 0x0400
je _backspace_return_line
    sub edi, 15360
_backspace_return_line:

    mov [edy], edi
ret

show_cursor:                    ; Les registres EBX et EDX sont volontairement pas utilisés afin de pouvoir le laisser aux fonctions appelantes.
    mov edi, [edy]

    test edi, 0x0400
je _show_cursor_test_edy_overflow
    add edi, 15360

_show_cursor_test_edy_overflow:
    cmp edi, 786432
jb _show_cursor_new_line
    mov ax, 0x18
    mov ds, ax
    mov es, ax

    mov ebx, edi                    ; sauvegarde de l'ancienne valeur EDI dans EBX

    sub ebx, (786432 - 16384)       ; ebx exprime en quelque sorte le nombre de lignes à slider fois 16384
    and ebx, 0xFFFFFC00             ; 16b 0000 0011 1111 1111

    mov esi, ebx                    ; placement sur la mémoire graphique à la ligne désignée à la ligne EBX
    xor edi, edi
    mov ecx, 786432
    sub ecx, ebx
    shr ecx, 2
    rep movsd                       ; Copie des pixels

    mov edi, 786432
    sub edi, ebx
    mov ecx, 786432
    sub ecx, edi
    shr ecx, 2
    xor eax, eax
    rep stosd                       ; L'écran du bas est nettoyé par des pixels noirs !

    mov ax, 0x10
    mov ds, ax

    mov edi, (786432 - 16384)       ; mise en place du curseur sur la dernière ligne.

_show_cursor_new_line:

    mov ax, 0x18
    mov es, ax

    mov al, [cursor_color]      ; groupe d'instruction pour placer la couleur en AL sur tous les groupes de 8 octets de EAX afin de pouvoir copier EAX entier en double via CL=2
    mov ah, al
    mov cx, ax
    shl eax, 16
    add ax, cx

    mov ch, 16
_show_cursor_cycle_heigth:
        mov cl, 2
_show_cursor_cycle_width:
            stosd
            dec cl
            test cl, cl
        jne _show_cursor_cycle_width
        add edi, 1016
        dec ch
        test ch, ch
    jne _show_cursor_cycle_heigth

    mov ax, 0x10
    mov es, ax

    sub edi, 16384
    mov [edy], edi
ret

hide_cursor:                     ; Les registres EBX et EDX sont volontairement pas utilisés afin de pouvoir le laisser aux fonctions appelantes.
    mov edi, [edy]

    test edi, 0x0400
je _hide_cursor_new_line
    add edi, 15360
_hide_cursor_new_line:

    mov ax, 0x18
    mov es, ax

    xor eax, eax

    mov ch, 16
_hide_cursor_cycle_heigth:
        mov cl, 2
_hide_cursor_cycle_width:
            stosd
            dec cl
            test cl, cl
        jne _hide_cursor_cycle_width
        add edi, 1016
        dec ch
        test ch, ch
    jne _hide_cursor_cycle_heigth

    mov ax, 0x10
    mov es, ax

    sub edi, 16384
    mov [edy], edi
ret

;segment .data


