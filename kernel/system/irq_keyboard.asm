
[BITS 32]

; Ajout à venir:                fleches de directions:
;                   -> Revenir plus bas dans la chaine.
;                   -> Rechercher des chaines précédentes.
GLOBAL irq_keyboard

extern print
extern putchar
extern jump_line
extern backspace
extern show_cursor
extern hide_cursor

extern CURSOR_FLAG

; Le Keyboard flag contient des informations sur les frappes en cours ou passé. Il est chargé dès le début de l'interruption dans le registre DX, décomposable en DH et DL
; Les 8 bits les plus forts contiennent le nombre de touches 'effectives' qui furent frappées, cette valeur sera après appel contenue dans DH
; Les 8 bits les plus faibles dans DL contiennent des FLAGS temporaires pour la touche en cours:
; BIT 0: Touche MAJ appuyée
; BIT 1: Touche ALT appuyée
; BIT 2: Mode spécial char actif -> initialisé quand un scancode 224 arrive, ce bit de remet à 0 lorsque une touche quelquonque est relachée, ainsi que MAJ et ALT relachées.
KEYBOARD_FLAG: dw 0b000000000000000

; Le scan_buffer contient l'historique de toutes les touches effectiement frappés et affichés à l'écran. Il contient donc la chaine courante à traiter après appui de ENTER.
SCAN_BUFFER: times 1024 db 0

;_DEBUG_VALUE:    db "value:%i", 10, 0

; ¡ Il NE FAUT PAS OUBLIER que cette interruption est lancée quand on appuie sur une touche mais aussi lorsque l'on en relache une !
irq_keyboard:
    mov ax, 0x10
    mov ds, ax
    mov es, ax

    xor eax, eax                ; EAX et EDX sont vraissemblablement les seuls registres qui seront utilisés par cette interruption.
    xor edx, edx

_wait_for_keyboard:
    in al, 0x64                 ; lire status du controleur clavier
    test byte al,1              ; si AL vaut 1, une touche est dispo
jz _wait_for_keyboard           ; sinon, l'on boucle jusqu'à ce que quelque chose sont dispo.

    in al, 0x60                 ; Lit la touche pressée. register AL

; TODO TEST DE SPECIAL CHAR -> Quand il est detecté, a pour effet de mettre le bit 2 de KERBOARD_FRAG à 1
    cmp al, 224
jne _begin_test
    or byte [KEYBOARD_FLAG], 0x04
jmp EOI


_begin_test:
    mov dl, al                  ; DL représente temporaire AL sans le bit relaché ou appuyé
    and dl, 0x7F

; Cette fonction se renseigne sur l'état de la touche MAJ, pression ou relacgée, et agit sur le premier bit du registre variable FRAG
_maj_test:
        cmp dl, 54                  ; SCANCODE MAJUSCULE 54
    jne _alt_test
        test al, 0x80
    jne _release_maj
        or byte [KEYBOARD_FLAG], 0x01
    jmp EOI
    _release_maj:
        and byte [KEYBOARD_FLAG], 0xFA                  ; 0b1111 1010           Le release de la MAJ enleve à la fois le bit MAJ et celui de SPECIAL CHAR
    jmp EOI

; Cette fonction se renseigne sur l'état de la touche ALT, pression ou relachée, et agit sur le second bit du registr variable FRAG
_alt_test:
        cmp dl, 56                  ; SCANCODE ALT 56
    jne _release_key_test
        test al, 0x80
    jne release_alt
    push_alt:
        or byte [KEYBOARD_FLAG], 0x02
    jmp EOI
    release_alt:
        and byte [KEYBOARD_FLAG], 0xF9                  ; 0b1111 1001           Le release de la TAB enleve à la fois le bit TAB et celui du scécial char
    jmp EOI

; Est-ce une touche enfoncée ou relachée, dans le cas d'un relachement, termine de suite l'interruption.
_release_key_test:
        test al, 0x80
    je _initialise_putchar
        and byte [KEYBOARD_FLAG], 0xFB      ; Pour TOUTES touches relachées, le flag concernant les SPECIAL CHAR s'annule !
    jmp EOI


_initialise_putchar:
    mov edx, eax
    call hide_cursor                    ; On utilise EDX ici ! (EDX et EBX sont preservés par la fonction hide_cursor)
    mov eax, edx

    shl eax, 2                          ; Multiplication de EAX par 4 pour qu'il pointe correctement dans la Keymap

    mov dx, [KEYBOARD_FLAG]             ; TODO Chargement du KEYBOARD FLAG dans DX


; La touche ALT est-elle enfoncée ?
_alt_assign:
        test dl, 0x02
    je _maj_assign
        add eax, 2
    jmp _load_char

; La touche MAJ est-elle enfoncée ?
_maj_assign:
        test dl, 0x01
    je _sp_char_assign
        add eax, 1
    jmp _load_char

; Est-on face à un SPECIAL CHAR ? Dans ce cas, le bit 2 de KEYBOARD_FLAG sera à 1
_sp_char_assign:
        test dl, 0x04
    je _load_char
        add eax, 3


; Charge le caractère correspondant à partir de la keyMap
_load_char:
    lea esi, [keymap + eax]

    xor eax, eax
    mov al, [esi]

; 0xFF est un code NO-ASSIGNED charactère, don't display anything
_Oxff_test:
        cmp al, 0xFF
    je EOI

; 0x0A exprime un retour à la ligne
_enter_test:
        cmp al, 0x0A
    Jne _backsplace_test
        call jump_line

        test dx, dx
    je _display_cursor

        shr edx, 8
        lea edi, [SCAN_BUFFER + edx]

        ;mov word [es:edi], 0x000A
        mov byte [es:edi], 0x00

        and word [KEYBOARD_FLAG], 0x00FF
        ;sub edi, edx

        ;push edi
        call _parse_command
        ;add esp, 4
    jmp _display_cursor

;0x08 exprime un backspace
_backsplace_test:
        cmp al, 0x08
    jne _write_char
        test dx, dx
    je _display_cursor
        sub word [KEYBOARD_FLAG], 0x0100
        call backspace
    jmp _display_cursor


_write_char:
    ;push eax
    ;mov esi, _DEBUG_VALUE
    ;push esi
    ;call print
    ;add esp, 8

; enregistrement du caractère dans la chaine SCAN_BUFFER, les 8 bits de poid fort KEYBOARD_FLAG -> register DX indiquent le nombre de caractères enregistrés:
    shr edx, 8
    lea edi, [SCAN_BUFFER + edx]
    mov [es:edi], al
    add word [KEYBOARD_FLAG], 0x0100

; Sequence d'affichage des caractères:
    push eax
    call putchar
    add esp, 4

_display_cursor:
    call show_cursor                            ; Affiche le curseur clignotant
    mov word [CURSOR_FLAG], 0x0001              ; Réinitialisation du curseur en position allumé pendant X ms

EOI:
    mov al,0x20
    out 0x20,al
iret


; TODO Ici est contenu un essai de parsing des commandes à entrer ! Relié à ENTER_TEST
parse_msg: db "Arg-> %s", 0
SUBSTRING: times 1024 db 0  ; utilisé pour l'esai temporaire de parse.

_parse_command:                                 ; EDI indique déjà la chaine à parser.
    mov esi, SCAN_BUFFER
    mov edi, SUBSTRING
_new_subchain:
    lodsb
    cmp al, 0
je _end_parse
    cmp al, ' '
je _print_parse
    mov [es:edi], al
    inc edi
jmp _new_subchain

_print_parse:
    cmp edi, SUBSTRING
je skip_step

    mov word [es:edi], 0x000A
    push esi
        push SUBSTRING
        push parse_msg
        call print
        add esp, 8
    pop esi

skip_step:
    mov edi, SUBSTRING
jmp _new_subchain

_end_parse:
    mov word [es:edi], 0x000A
        push SUBSTRING
        push parse_msg
        call print
        add esp, 8
ret

; impr écran et * du clavier numérique [CODE 55] spécial char en print screen
; division clavier numérique et ! [code 53] spécial char en division clavier numérique
; ENTER clavier numérique est spécial char [code 28] équivaut au enter normal.
; touche WIN spécial char [value 91]
; ALT GR est un spécial char, même valeur que ALT normal [code 56]
; CTRL DROIT spécial char [code 29] idem que ctrl gauche en non spécial char
; Entre ALT GR et ctrl droit, touche en spécial char aussi.[code 93]
; Les touches de direction sous le touche MAJ sont toutes des spécial char [code 72, 75, 77, 80]
; Toutes les touches à droite de PAUSE ATTN sont des spécial char
; INSER SPC         [code 82]
; SUPPR SPC         [code 83]
; PLAY/PAUSE SPC    [code 71
; REC SPC           [code 73]
; RET ARR           [code 81]
; FIN spc           [code 79]
; La touche PAUSE ATTN est un spécial char encore plus particulier !

; 4 catégories; NORMAL --- MAJ ---- ALT ----- SPECIAL CHAR

keymap: db 0xFF,0xFF,0xFF,0xFF          ; Il n'y a pas de touche scancode zéro

        db 0x1B, 0x1B, 0x1B, 0xFF	    ; TODO ESCAPE
        db '&', '1', 180, 0xFF          ; symbole ´ -> 180
        db 233, '2', '~', 0xFF          ; symbole é -> 233
        db '"', '3', '#', 0xFF
        db "'", '4', '{', 0xFF
        db '(', '5', '[', 0xFF
        db '-', '6', '|', 0xFF
        db 232, '7', '`', 0xFF          ; symbole è -> 232
        db '_', '8', '\', 0xFF
        db 231, '9', '^', 0xFF          ; (10) symbole ç -> 231
        db 224, '0', '@', 0xFF          ; symbole à -> 224
        db ')', 176, ']', 0xFF          ; symbole ° -> 176
        db '=', '+', '}', 0xFF
        db 0x08, 0x08, 0x08, 0xFF	    ; TODO BACKSPACE

        db 0x09, 0x09, 0x09, 0xFF	    ; TODO TAB
        db 'a', 'A', 'a', 0xFF
        db 'z', 'Z', 'z', 0xFF
        db 'e', 'E', 'e', 0xFF
        db 'r', 'R', 'r', 0xFF
        db 't', 'T', 't', 0xFF          ; (20)
        db 'y', 'Y', 'y', 0xFF
        db 'u', 'U', 'u', 0xFF
        db 'i', 'I', 'i', 0xFF
        db 'o', 'O', 'o', 0xFF
        db 'p', 'P', 'p', 0xFF
        db '^', 168, '~', 0xFF          ; symbole ¨ -> 168
        db '$', 163, 234, 0xFF          ; symbole £ -> 163   symbole ê -> 234
        db 0x0A, 0x0A, 0x0A, 0x0A       ; TODO ENTER                                TODO sp. enter clavier numérique
        db 0xFF, 0xFF, 0xFF, 0xFF       ; TODO CTRL DROIT ET GAUCHE                 TODO sp. CTRL DROIT

        db 'q', 'Q', 'q', 0xFF          ; (30)
        db 's', 'S', 's', 0xFF
        db 'd', 'D', 'd', 0xFF
        db 'f', 'F', 'f', 0xFF
        db 'g', 'G', 'g', 0xFF
        db 'h', 'H', 'h', 0xFF
        db 'j', 'J', 'j', 0xFF
        db 'k', 'K', 'k', 0xFF
        db 'l', 'L', 'l', 0xFF
        db 'm', 'M', 'm', 0xFF
        db 251, '%', 178, 0xFF          ; (40) symbole ù -> 251 symbole ² -> 178
        db 178, 178, 178, 0xFF          ; symbole ² -> 178

        db 0xFF, 0xFF, 0xFF, 0xFF	    ; TODO Left shift (mini maj)
        db '*', 181, 179, 0xFF          ; symbole µ -> 181 symbole ³ -> 179                     BUG confusion entre le symbole MU ey MICRO ? µ
        db 'w', 'W', 'w', 0xFF
        db 'x', 'X', 'x', 0xFF
        db 'c', 'C', 'c', 0xFF
        db 'v', 'V', 'v', 0xFF
        db 'b', 'B', 'b', 0xFF
        db 'n', 'N', 'n', 0xFF
        db ',', '?', 191, 0xFF          ; (50) symbole ¿ -> 191
        db ';', '.', 215, 0xFF          ; symbole × -> 215
        db ':', '/', 247, 0xFF          ; symbole ÷ -> 247
        db '!', 167, 161, '/'           ; symbole § -> 167 symbole ¡ -> 161         TODO sp. symbole / clavier numérique
        db 0xFF, 0xFF, 0xFF, 0xFF	    ; TODO MAJUSCULE                                        TODO NO MOTIF


        db '*', '*', '*', 0xFF	        ;                                           TODO sp. PRINT SCREEN
        db 0xFF, 0xFF, 0xFF, 0xFF	    ; TODO ALT GAUCHE && DROIt                  TODO sp. ALT GR (alt. droit)
        db ' ', ' ', ' ', ' '   	    ; TODO ESPACE
        db 0xFF, 0xFF, 0xFF, 0xFF       ; TODO VEROUILLAGE MAJUSCULE

        db 0xFF, 0xFF, 0xFF, 0xFF       ; F1
        db 0xFF, 0xFF, 0xFF, 0xFF	    ; F2 (60)
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F3
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F4
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F5
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F6
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F7
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F8
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F9
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F10
        db 0xFF, 0xFF, 0xFF, 0xFF	    ; TODO VEROUILLAGE CLAVIER NUM

        db 0xFF, 0xFF, 0xFF, 0xFF       ; (70)
        db '7','7','7', 0XFF            ;                                           TODO sp. PLAY/PAUSE
        db '8','8','8', 0xFF	        ;                                           TODO sp. associé aussi à Fleche HAUT
        db '9','9','9', 0xFF            ;                                           TODO sp. REC SPC
        db '-','-','-', 0xFF
        db '4','4','4', 0xFF	        ;                                           TODO sp. associé aussi à fleche GAUCHE
        db '5','5','5', 0xFF
        db '6','6','6', 0xFF	        ;                                           TODO sp. associé aussi à fleche DROITE
        db '+','+','+', 0xFF
        db '1','1','1', 0xFF            ;                                           TODO sp. FIN

        db '2','2','2', 0xFF	        ; (80)                                      TODO sp. Associé aussi à Fleche BAS
        db '3','3','3', 0xFF            ;                                           TODO sp. RET ARRIERE
        db '0','0','0', 0xFF            ;                                           TODO sp. INSER
        db '.','.','.', 0xFF            ;                                           TODO sp. SUPPR LINE
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db '<', '>', '|', 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F11
        db 0xFF, 0xFF, 0xFF, 0xFF       ; F12
        db 0xFF, 0xFF, 0xFF, 0xFF

        db 0xFF, 0xFF, 0xFF, 0xFF	    ; (90)
        db 0xFF, 0xFF, 0xFF, 0xFF       ;                                           TODO sp. WIN
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF       ;                                           TODO sp. ALT GR
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF

        db 0xFF, 0xFF, 0xFF, 0xFF       ; (100)
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF
        db 0xFF, 0xFF, 0xFF, 0xFF       ; (127)
