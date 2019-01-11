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

[BITS 32]
section .data

GLOBAL keymap
keymap: db 0x00,0x00,0x00,0x00          ; Il n'y a pas de touche scancode zéro

        db 0x1B, 0x1B, 0x1B, 0x00       ; TODO ESCAPE
        db '&', '1', 180, 0x00          ; symbole ´ -> 180
        db 233, '2', '~', 0x00          ; symbole é -> 233
        db '"', '3', '#', 0x00
        db "'", '4', '{', 0x00
        db '(', '5', '[', 0x00
        db '-', '6', '|', 0x00
        db 232, '7', '`', 0x00          ; symbole è -> 232
        db '_', '8', '\', 0x00
        db 231, '9', '^', 0x00          ; (10) symbole ç -> 231
        db 224, '0', '@', 0x00          ; symbole à -> 224
        db ')', 176, ']', 0x00          ; symbole ° -> 176
        db '=', '+', '}', 0x00
        db 0x08, 0x08, 0x08, 0x00       ; TODO BACKSPACE

        db 0x09, 0x09, 0x09, 0x00       ; TODO TAB
        db 'a', 'A', 'a', 0x00
        db 'z', 'Z', 'z', 0x00
        db 'e', 'E', 'e', 0x00
        db 'r', 'R', 'r', 0x00
        db 't', 'T', 't', 0x00          ; (20)
        db 'y', 'Y', 'y', 0x00
        db 'u', 'U', 'u', 0x00
        db 'i', 'I', 'i', 0x00
        db 'o', 'O', 'o', 0x00
        db 'p', 'P', 'p', 0x00
        db '^', 168, '~', 0x00          ; symbole ¨ -> 168
        db '$', 163, 234, 0x00          ; symbole £ -> 163   symbole ê -> 234
        db 0x0A, 0x0A, 0x0A, 0x0A       ; TODO ENTER                                TODO sp. enter clavier numérique
        db 0x00, 0x00, 0x00, 0x00       ; TODO CTRL DROIT ET GAUCHE                 TODO sp. CTRL DROIT

        db 'q', 'Q', 'q', 0x00          ; (30)
        db 's', 'S', 's', 0x00
        db 'd', 'D', 'd', 0x00
        db 'f', 'F', 'f', 0x00
        db 'g', 'G', 'g', 0x00
        db 'h', 'H', 'h', 0x00
        db 'j', 'J', 'j', 0x00
        db 'k', 'K', 'k', 0x00
        db 'l', 'L', 'l', 0x00
        db 'm', 'M', 'm', 0x00
        db 251, '%', 178, 0x00          ; (40) symbole ù -> 251 symbole ² -> 178
        db 178, 178, 178, 0x00          ; symbole ² -> 178

        db 0x00, 0x00, 0x00, 0x00       ; TODO Left shift (mini maj)
        db '*', 181, 179, 0x00          ; symbole µ -> 181 symbole ³ -> 179                     BUG confusion entre le symbole MU ey MICRO ? µ
        db 'w', 'W', 'w', 0x00
        db 'x', 'X', 'x', 0x00
        db 'c', 'C', 'c', 0x00
        db 'v', 'V', 'v', 0x00
        db 'b', 'B', 'b', 0x00
        db 'n', 'N', 'n', 0x00
        db ',', '?', 191, 0x00          ; (50) symbole ¿ -> 191
        db ';', '.', 215, 0x00          ; symbole × -> 215
        db ':', '/', 247, 0x00          ; symbole ÷ -> 247
        db '!', 167, 161, '/'           ; symbole § -> 167 symbole ¡ -> 161         TODO sp. symbole / clavier numérique
        db 0x00, 0x00, 0x00, 0x00       ; TODO MAJUSCULE                            TODO NO MOTIF


        db '*', '*', '*', 0x00          ;                                           TODO sp. PRINT SCREEN
        db 0x00, 0x00, 0x00, 0x00       ; TODO ALT GAUCHE && DROIt                  TODO sp. ALT GR (alt. droit)
        db ' ', ' ', ' ', ' '           ; TODO ESPACE
        db 0x00, 0x00, 0x00, 0x00       ; TODO VEROUILLAGE MAJUSCULE

        db 0x00, 0x00, 0x00, 0x00       ; F1
        db 0x00, 0x00, 0x00, 0x00       ; F2 (60)
        db 0x00, 0x00, 0x00, 0x00       ; F3
        db 0x00, 0x00, 0x00, 0x00       ; F4
        db 0x00, 0x00, 0x00, 0x00       ; F5
        db 0x00, 0x00, 0x00, 0x00       ; F6
        db 0x00, 0x00, 0x00, 0x00       ; F7
        db 0x00, 0x00, 0x00, 0x00       ; F8
        db 0x00, 0x00, 0x00, 0x00       ; F9
        db 0x00, 0x00, 0x00, 0x00       ; F10
        db 0x00, 0x00, 0x00, 0x00       ; TODO VEROUILLAGE CLAVIER NUM

        db 0x00, 0x00, 0x00, 0x00       ; (70)
        db '7','7','7', 0x00            ;                                           TODO sp. PLAY/PAUSE
        db '8','8','8', 0x00            ;                                           TODO sp. associé aussi à Fleche HAUT
        db '9','9','9', 0x00            ;                                           TODO sp. REC SPC
        db '-','-','-', 0x00
        db '4','4','4', 0x00            ;                                           TODO sp. associé aussi à fleche GAUCHE
        db '5','5','5', 0x00
        db '6','6','6', 0x00            ;                                           TODO sp. associé aussi à fleche DROITE
        db '+','+','+', 0x00
        db '1','1','1', 0x00            ;                                           TODO sp. FIN

        db '2','2','2', 0x00            ; (80)                                      TODO sp. Associé aussi à Fleche BAS
        db '3','3','3', 0x00            ;                                           TODO sp. RET ARRIERE
        db '0','0','0', 0x00            ;                                           TODO sp. INSER
        db '.','.','.', 0x00            ;                                           TODO sp. SUPPR LINE
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db '<', '>', '|', 0x00
        db 0x00, 0x00, 0x00, 0x00       ; F11
        db 0x00, 0x00, 0x00, 0x00       ; F12
        db 0x00, 0x00, 0x00, 0x00

        db 0x00, 0x00, 0x00, 0x00       ; (90)
        db 0x00, 0x00, 0x00, 0x00       ;                                           TODO sp. WIN
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00       ;                                           TODO sp. ALT GR
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00

        db 0x00, 0x00, 0x00, 0x00       ; (100)
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00
        db 0x00, 0x00, 0x00, 0x00       ; (127)

segment .text
GLOBAL get_keymap
get_keymap:
    push ebp
    mov ebp, esp

    mov eax, [ebp + 8]
    mov esi, keymap
    add esi, eax
    lodsb
    and eax, 0xFF

    pop ebp
ret
