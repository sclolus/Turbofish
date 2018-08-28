[BITS 16]  ; indique a nasm que l'on travaille en 16 bits
[ORG 0x7C00]                ; (GRUB2-CMP) CONFORME                  memory_sector=64ko : DIV 16 = 4ko && DIV 16 = 256o -> Le secteur de boot est chargé entre 7C00 et 7E00  SEGMENT 0X0

jmp pre_amorce              ; (GRUB2-CMP) CONFORME                  jump obligatoire au démarage pour l'allignement avec le code.
                                                                ;   au moment du démarage, le registre DL est chargé par le numéro du disque actuellement lu (celui-ci !)
fatal_disk_error:
    mov si, disk_load_error

_begin_print_string:
    lodsb               ; ds:si -> al
    cmp al, 0           ; fin chaine ? (une chaine normale doit finir par 0)
jz _freeze_process

    mov ah, 0x0E		; Teletype Mode :appel au service 0x0e
    mov bx, 0x07		; Black /  bx -> attribut, al -> caractere ascii
    INT 0x10    		; Print Character
jmp _begin_print_string

_freeze_process:
    mov ah, 0x86
    mov cx, 0xFFFF
    int 15h
    jmp _freeze_process

; requis par VIEW_AX_HEXADECIMAL, PRINT_STRING
_print_char:            ; AFFICHE LE CARACTERE PRESENT DANS LE REGISTRE AL
    mov ah, 0x0E		; Teletype Mode :appel au service 0x0e
    mov bx, 0x07		; Black /  bx -> attribut, al -> caractere ascii
    INT 0x10    		; Print Character
RET

pre_amorce:                 ; (GRUB2-CMP) CONFORME
cli                         ; (GRUB2-CMP) CONFORME                  ; Cette instruction met l'indicateur d'état IF à 0. Après avoir exécuter cette instruction, aucune interruption ne sera admise tant que l'instruction STI n'est pas rencontrée.
;jmp 0x0000:amorce          ; (GRUB2-CMP) suppression
;amorce:                    ; (GRUB2-CMP) suppression
    mov ax, cs              ; (GRUB2-CMP) remplacement xor ax, ax   ; CS pointe vers le segment actuel du code du programme, la valeur est placée dans AX
    mov ds, ax              ; (GRUB2-CMP) CONFORME                  ; DS et ES prennent la valeur du segement CS, ainsi les segment de données sont sur ce même fichier
    mov es, ax              ; (GRUB2-CMP) rajout

    mov ax, 0x8000          ; (GRUB2-CMP) rajout
    mov ss, ax              ; (GRUB2-CMP) rajout                    ; SS désigne le segment utilisé par la pile. ici ce sera le 0x8000 (un segment fait 64ko)
    mov sp, 0x2000          ; (GRUB2-CMP) CONFORME                  ; SP désigne le sommet de la pile.                              Pile: 8000:0000 à (8200:0000 or 8000:2000)
sti                         ; (GRUB2-CMP) CONFORME                  ; STI rend les interruptions BIOS disponibles à nouveau !

; Lecture disque du programme noyau. ES;BX          : 4 secteurs de 512o chargés à partir de 0x0000:7E00 finiront à 0x0000:8600, nous restons insi dans les 64 premiers Ko de mémoire.
    mov ax, 0x3000          ; Passage sur le segment (16b) 0x1000:0000 (32b) 0x0001:0000
    mov es, ax

    xor bx, bx              ; Segment exacte du noyau ES:BX Soit 0000:0000 ici !
    mov ah, 2               ; Fonction lecture fichier VERS mémoire, disk TO mémory
    mov al, 14 	   	        ; Nombre de secteurs de 512o à charger en mémoire.
    xor ch, ch              ; Premier cylindre, soit noté ZERO.
    mov cl, 3               ; Copie à partir du second secteur, le premier étant occupé par le programme d'amorcage de 512o (celui ci)
    xor dh, dh              ; Spécification de la tête de lecture 0, (le mode LBA (très répandu) évite de se soucier des tetes/cylindre)
    int 0x13                ; ORDDRE DE LECTURE DISK & D'ECRITURE MEMOIRE
jc fatal_disk_error         ; gestion erreur -> JC :Saut si drapeau de retenue vaut 1 (CF = 1).

JMP 0x3000:0000 ; Et on donne la main au programme que nous venons de charger

disk_load_error db "BOOTSECT WARNING: Unable to load anything ! Inhibit CPU.", 13, 10, 0

times 510-($-$$) db 0       ;formate le fichier sur 510 octets                                     BUG Pour une raison que j'ingore, ca ne boot pas sur mon PC si je met des 144 (nop), je met donc des zéros et ca fonctionne !
dw 0xAA55                   ;ajoute le code "magique" d'amorcage au 511e et 512e octet. AA55

times 1024-($-$$) db 144    ; 512 octets de padding sont mis après le secteur d'amorcage.
