
[BITS 16]

%define vesa_Global_Info            8192                        ; Général VESA capability, la structure sera écrite juste à la fin du code ! placées en 0x0A00:0000 (16b) ou 0x0000:A000
%define vesa_Signature              (vesa_Global_Info)          ; Général VESA capability, la structure sera écrite juste à la fin du code ! placées en 0x1200:0000 (16b) ou 0x0001:2000
%define vesa_Version                (vesa_Global_Info + 0x04)
%define vesa_Compatibity_flag       (vesa_Global_Info + 0x0A)
%define vesa_Graph_Mode_Pointer     (vesa_Global_Info + 0x0E)

; Cette seconde structure, renseignements sur le mode utilisé, sera écrite 256 octets après VESA capability, soit 256o après la fin du code en mémoire.
%define vesa_Mode_Info (8192+256)                    ; Ce pointeur pointe "au delà" du programme en 0x0000:8000 + 8192 => 0x0000:A000 -> On écira & lira ici le buffer VESA ici!
%define vesa_Granulosity (vesa_Mode_Info + 0x04)     ; granulosité
%define videoModePtr (vesa_Mode_Info + 14)           ; Liste de pointeurs vers les modes supportés en VESA.
%define mode_Attributes (vesa_Mode_Info + 0xFF)      ; Encore au délà du programme, en 0x7C00 + 512 + 256 => 0x7C00 + 768
%define flat_Memory (vesa_Mode_Info + 40)            ; Variable de 4 octects (double mot) contenant l'emplacement mémoire de la FLB vesa
                                                     ; Une info capitale est "FlatMemory" en "Mode_Attributes + 40" qui désigne l'espace de mémoire "linéaire" lfb de la carte graphique

%define graphic_text_line_for_32b_kernel    (8192+512)                                      ; Utilisé pour le passage de la ligne courante de texte au noyau 32bits.
%define graphic_text_colomn_for_32b_kernel  (graphic_text_line_for_32b_kernel + 1)          ; Utilisé pour le passage de la colomn courante de texte au noyau 32bits.


_print_graphic_text_line: db 0                       ; utilisés par la fonction print principale.
_print_graphic_text_colomn: dw 0
_print_color: db 0

_graphic_modes_list:
times 128 dw 0xFFFF                                  ; zone de 128 0xFFFF pour acceuillir le tampon des modes graphiques disponibles.

_print_granulosity_coeff: db 0                       ; généré après l'obtention des informations sur le mode VESA selectionné

_view_hex_one_char: db 0, 0                          ; utilisé par view_hex_register

    %include "../../polices/alpha.asm"               ; inclusion des belles polices LATINO

; YES   check_vesa_capability        ; Renseigne sur les possibilités du mode VESA
; YES   copy_graphic_modes_buffer    ; Copie le buffer des différents modes vidéos disponibles dans un espace protégé. IMPORTANT car la méthode 4F01 peut écraser le buffer naturel !
; YES   set_vesa_graphic             ; Active le mode VESA
; YES   clear_screen                 ; Efface l'écran, remet le curseur à 0:0 et met aussi la banque 0 active.
; YES   print                        ; display a string
; YES   set_text_color               ; set an other color
; YES   view_hex_register            ; Permet de consulter la valeur du registre AX (elle rend AX dans le même état)
; YES   print_Text_Mode              ; Permet d'écrire du texte dans le cas ou le mode SVGA n'a pas été initialisé...
; YES   write_cursor_position_for_32b_kernel    ; Met la ligne et la colonne courante de print à un endroit connu pour permettre au noyau 32 bits de les consulter.

write_cursor_position_for_32b_kernel:
    mov al, [_print_graphic_text_line]
    mov di, graphic_text_line_for_32b_kernel
    mov [di], al

    mov ax, [_print_graphic_text_colomn]
    mov di, graphic_text_colomn_for_32b_kernel
    mov [di], ax
ret


check_vesa_capability:
    mov ax, vesa_Global_Info       ; Demande de renseignements sur les capacités graphiques. 0x4F00
    mov di, ax
    mov ax, 0x4F00
    int 0x10
ret

copy_graphic_modes_buffer:
    mov dx, ds

    mov si, vesa_Graph_Mode_Pointer            ; ATTENTION ! VideoModePtr est une liste de pointeurs exprimés en OFFSET:SEGMENT

    lodsw                                      ; charge le "mot" pointé par SI dans AX et incrémente SI de 2 si le drapeau de direction DF est à 0 : "LOaD Si Word"
    mov bx, ax

    lodsw                                      ; Le premier "lodsw" a chargé l'offset à appliquer à SI, le second charge le segment !
    mov ds, ax
    mov si, bx                                 ; Association de pointeur de données DS:SI sur l'endroit ou se trouve les différents modes graphiques.

    mov ax, cs
    mov es, ax

    mov di, _graphic_modes_list                 ; copie de tous les différents modes trouvés dans la variable de 256 octets _graphic_modes_list
    mov cx, 126                                 ; Bloque le processus si 127 modes ont été trouvé. Celà évite un dépassement de _graphic_modes_list. (la dernière valeur doit rester 0xFFFF)

_cp_one_graph_mode:
    cmp cx, 0
je break_research_modes                         ; TROP DE MODES, on sort ;)
    lodsw
    stosw

    dec cx
    cmp ax, 0xFFFF                              ; Le dernier mot dans la liste des modes doit être FFFF -> (end of list)
jne _cp_one_graph_mode

break_research_modes:
    mov ds, dx
ret

set_vesa_graphic:
    mov ax, vesa_Mode_Info       ; Demande de renseignements sur la capacité graphique 0x105
    mov di, ax                   ; L'information de la granularité est très importante ici.
    mov ax, 0x4F01
    mov cx, 0x4105               ; Ajoute au bit 14 de CX, la valeur 1 pour "être sur de tenir compte de "Linéar Frame Buffer"
    int 0x10

    mov si, vesa_Granulosity
    mov bx, [si]
    mov ax, 0x40
    xor cl, cl
determine_coefficent_de_granulosite:
    mov dx, ax
    shr dx, cl
    cmp dx, bx
    je granulosity_founded
    inc cl
jmp determine_coefficent_de_granulosite
granulosity_founded:
    mov di, _print_granulosity_coeff
    mov [di], cl

;SWITCH TO VGA MODE NOW
    mov ax, 0x4F02
    mov bx, 0x105           ; 105H     1024x768     256  packed pixel
    int 0x10
ret

clear_screen:
    xor ax, ax
    mov [_print_graphic_text_line], ax
    mov [_print_graphic_text_colomn], ax

    mov cl, [_print_granulosity_coeff]  ; Le coefficient de granularité sera necessaire pour le calcul de banque.

    mov ax, 0xA000
    mov es, ax

; Mise de tous les pixels à 0 pour toutes les banques.
    mov dx, 12
_clear_screen_per_bank:
    mov ax, 0x4F05      ; Fonction 0x4F05, Change de banque selon la valeur de DX (retenu de la multiplication)
    xor bx, bx          ; BX DOIT être mis à zéro.
    shl dx, cl          ; Multiplication de la valeure DX obtenue par le coefficient de granularité
    int 0x10            ; Ordonne le changement de BANK !

    xor ax, ax
    mov bl, cl
    mov cx, 0x8FFF
    rep stosw

    mov cl, bl

    dec dx
    jns _clear_screen_per_bank

    mov ax, cs
    mov es, ax
ret

print:
    push bp
    mov bp, sp

    mov bx, [_print_graphic_text_colomn]    ; BX dans cette fonction exprime l'endroit en pixel ou commencer la ligne sur Width. Il est important de le retrouver pour ca.
    mov dl, [_print_color]                  ; DL contiendra la couleur

    mov si, [bp+4]                          ; Recherche dans la pile d'appel de la chaine de caractère à afficher, mise dans SI

_print_begin:
    mov al, [si]
    cmp al, 0
je _print_endprint                          ; Le caractère 0x00 indique la fin de la chaîne.

    mov cx, bx                              ; BX (colomn number) prend respectivement comme limite 0, 1024, 2048, 3072 et 4096, grace à CX on extrait les 10 derniers bits pour voir si c'est un multiple de 1024.
    and cx, 0x04FF                          ; Dans ce cas, il faudra passer à la ligne suivante !

    cmp cx, 1024                            ; Si CX est égal à 1024, il faut passer à la ligne suivante.
je _print_jump_line

    inc si                                  ; L'on prépare l'incrémentation SI de 1 afin de passer au caractère suivant à la prochaine itération de _print_begin !
                                            ; Une incrémentation de SI ne pouvait pas être faite au cas ou BX était égal à 1024, sinon le saut de ligne 'casserait' un caractère !

    cmp al, 0x0A                            ; Le caractère 0x0A indique un saut de ligne inscrit dans la chaine elle-même.
je _print_jump_line

    push si

    xor ah, ah

    shl ax, 4
    mov si, _print_graphical_char_begin
    add si, ax

    mov ax, 0xA000
    mov es, ax

    mov di, bx                          ; BX exprime ici la position Width du prochain caractère ( 0 ---> 65535 ) - Une banque entière quoi :p

    mov cl, 16                          ; Initialisation d'un compteur de 16 pour un affichage d'un caractère sur 16 pixels de hauteur.

_print_draw_character:
    mov ah, [si]                        ; Récupération d'un octet du dessin du caractère.                   ---> AH
    inc si                              ; saut au prochain octet du dessin du caractère dans l'alphabet.
    mov ch, 8                           ; Initialisation d'un compteur de 8 pour l'affichage horizontal du caractère. (8pixels)
_print_draw_alpha_line:
    mov al, ah                          ; copie du morceau du motif du caractère dans AL en vue de test de la présence d'un bit 1 ou 0
    and al, 0x80
    cmp al, 0x80                        ; TEST le bit de poid fort, 0->pas de couleur 1->Affiche pixel
jne _print_skip_colorize
    mov al, dl                          ; Association de la couleure
_print_skip_colorize:
    stosb                               ; Cette instruction permet de copier le registre AL dans la cellule mémoire à l'adresse ES:[DI] et incrémente/décrémente le registre DI de 1 en fonction de l'état du drapeau de direction.
    shl ah, 1                           ; préparation au test du bit suivant.
    dec ch                              ; dec compteur WIDTH
jns _print_draw_alpha_line
;next_line
    add di, 1015                        ; Prépare DI pour la prochaine ligne de pixels
    dec cl                              ; dec compteur HEIGTH
jns _print_draw_character
;end_of_putchar

    add bx, 8                           ; BX (piwel width) est incrémenté de 8 pixels avant de passer au prochaine caractère.
    pop si
jmp _print_begin


_print_jump_line:
    push es
    push dx                                 ; registre DX sauvegardé, car DL contient le numéro de la couleur.

    mov bx, cs
    mov es, bx                          ; L'accès au segment graphique modifie ES, ici, puisque nous écrivons sur le code, il faut le réaligner avec CS.

    mov [_print_graphic_text_colomn], byte 0    ; réinitialisation à la colonne Zéro

    xor bx, bx
    mov bl, [_print_graphic_text_line]
    inc bl                              ; Récupération du numéro de la ligne courant & incrémentation de cette dernière.
    mov [_print_graphic_text_line], bl  ; BL doit garder temporairement le numéro de la ligne, il sera utilisé quelques instructions plus loin.

    mov ch, bl                          ; Sauvegarde du numéro de colone, necessaire pour le calcul final de l'emplacement du nouveau caractère en pixels.
    shr bx, 2                           ; Division du numéro de ligne par 4, il y a 12 banques et 48 lignes, ainsi, l'on peut determiner quelle banque doit être activée !
    mov cl, [_print_granulosity_coeff]  ; Le coefficient de granularité sera necessaire pour le calcul de banque.

    mov dx, bx
    mov ax, 0x4F05      ; Fonction 0x4F05, Change de banque selon la valeur de DX (retenu de la multiplication)
    xor bx, bx          ; BX DOIT être mis à zéro.
    shl dx, cl          ; Multiplication de la valeure DX obtenue par le coefficient de granularité
    int 0x10            ; Ordonne le changement de BANK !

    mov bl, ch                          ; Restauration du numéro de Ligne
    shl bx, 14                          ; multiplication de BX par 16384 (16 lignes graphiques de 1024 pixels)

    pop dx
    pop es
jmp _print_begin


_print_endprint:
mov ax, cs                                              ; La fonction PRINT modifie ES puisque ce dernier y pointe sur la mémoire graphique, A la fin, ES est remis sur CS
mov es, ax

mov [_print_graphic_text_colomn], bx                    ; Ecriture finale des nouvelles données de BX, position sur Width du curseur !

mov sp, bp
pop bp
ret


set_text_color:
    push bp
    mov bp, sp
    mov ax, [bp+4]
    mov [_print_color], ax
    mov sp, bp
    pop bp
ret


;Fonction  d'affichage des variables en hexadécimal 16bits, doit être chargée dans le registre AX
view_hex_register:
    push bp
    mov bp, sp

    push ax

    mov ax, [bp+4]
    push ax
    call print          ; Envoie des données à afficher à la fonction PRINT
    add sp, 2

    pop ax

    mov cl, 12  ; Initialisation de cl à 12, servant au décalage shr

_view_hex_register_begin_hex_register:                              ; COMMENCEMENT de l'affichage utile des valeurs du registre.
    push ax         ; sauvegarde de la valeure originale ax

    shr ax, cl      ; Décalage de bit sur la droite
    and ax, 0x000F  ; Masque AND pour ne garder que les valeurs utiles

    cmp al, 10      ; Teste si le chiffre à afficher est supèrieur ou infèrieur à 10
jl _view_hex_register_continue_hex_print
    add al, 7       ; mise en forme supplémentaire si c'est une lettre entre A & F
_view_hex_register_continue_hex_print:
    add al, 48      ; mise en forme chiffre conventionnel

call _view_hex_register_display_hex_register

    pop ax      ; restauration de la valeure originale ax

    cmp cl, 0   ; demande si les 16 bits ont été "couverts"
Je _view_hex_register_end_hex_register

    sub cl, 4   ; Prépare un décalage moindre pour la prochaibe boucle.
jmp _view_hex_register_begin_hex_register

_view_hex_register_display_hex_register:
    mov di, _view_hex_one_char
    mov [di], al
    push cx                     ; CX est lié au compteur principal de la fonction appelante, il doit être IMPRATIVEMENT sauvegardé !
    push _view_hex_one_char
    call print                  ; Envoie des données à afficher à la fonction PRINT
    add sp, 2
    pop cx
ret

_view_hex_register_end_hex_register:
    mov sp, bp
    pop bp
RET


print_Text_Mode:
    push bp
    mov bp, sp

    mov ax, [bp+4]
    mov si, ax

_beginprint:
    lodsb         ; ds:si -> al
    cmp al, 0     ; fin chaine ? (une chaine normale doit finir par 0)
je _end_print

CALL _print_char
jmp _beginprint

_end_print:
    mov sp, bp
    pop bp
    ret

_print_char:            ; AFFICHE LE CARACTERE PRESENT DANS LE REGISTRE AL              ; AH=0x0E       AL = Character, BH = Page Number, BL = Color (only in graphic mode)
    mov ah, 0x0E		; Teletype Mode :appel au service 0x0e
    mov bx, 0x0109		; Black /  bx -> attribut, al -> caractere ascii
    INT 0x10    		; Print Character
RET
