[BITS 16]
[ORG 0x7C00]

%define VESA_GLOBAL_INFO_PTR        0x800
%define VESA_MODE_INFO_PTR          0x810
%define VESA_MODE_LIST_PTR          0x820

%define KERNEL_OFFSET_ON_DISK       10
%define KERNEL_BASE_MEMORY          0x1000              ; on 16bits 0x1000:0x0 = 0x1000:0000. on 32bits 0x0001:0000
%define KERNEL_NB_SECTORS           128                 ; number of sectors for kernel to load, one sector is 512o size

%define FONT_OFFSET_ON_DISK         2
%define FONT_BASE_MEMORY            0x900               ; on 16bits 0x900:0x0 = 0x900:0000. on 32bits 0x0000:9000
%define FONT_NB_SECTORS             8

; Memory emplacement of SVGA dada between 0x0000 8000 to 0x0000 8300
%define vesa_Global_Info            0                   ; Général VESA capability, la structure sera écrite juste à la fin du code ! placées en 0x0A00:0000 (16b) ou 0x0000:A000
%define vesa_Signature              (vesa_Global_Info)  ; Général VESA capability, la structure sera écrite juste à la fin du code ! placées en 0x1200:0000 (16b) ou 0x0001:2000;%define vesa_Version                (vesa_Global_Info + 0x04)
%define vesa_Compatibity_flag       (vesa_Global_Info + 0x0A)
%define vesa_Graph_Mode_Pointer     (vesa_Global_Info + 0x0E)

; Cette seconde structure, renseignements sur le mode utilisé, sera écrite 256 octets après VESA capability, soit 256o après la fin du code en mémoire.
%define vesa_Mode_Info              0                   ; Ce pointeur pointe "au delà" du programme en 0x0000:8000 + 8192 => 0x0000:A000 -> On écira & lira ici le buffer VESA ici!
%define vesa_Granulosity            (vesa_Mode_Info + 0x04)  ; granulosité
%define videoModePtr                (vesa_Mode_Info + 14)    ; Liste de pointeurs vers les modes supportés en VESA.
%define mode_Attributes             (vesa_Mode_Info + 0xFF)  ; Encore au délà du programme, en 0x7C00 + 512 + 256 => 0x7C00 + 768
%define flat_Memory                 (vesa_Mode_Info + 40)    ; Variable de 4 octects (double mot) contenant l'emplacement mémoire de la FLB vesa
                                                        ; Une info capitale est "FlatMemory" en "Mode_Attributes + 40" qui désigne l'espace de mémoire "linéaire" lfb de la carte graphique

%define graphic_modes_list          0                   ; zone de 128 0xFFFF pour acceuillir le tampon des modes graphiques disponibles.

jmp start

; Demande de renseignements sur les capacités graphiques. 0x4F00
check_vesa_capability:
    push es
    mov ax, VESA_GLOBAL_INFO_PTR
    mov es, ax
    xor di, di
    mov ax, 0x4F00
    int 0x10
    pop es
ret

no_vesa_card:
    mov si, no_vesa_issue
    call afficher
    jmp $

disk_fatal_error:
    mov si, cannot_load_from_disk
    call afficher
    jmp $

copy_graphic_modes_buffer:
    push ds
    push es
    mov dx, ds

; Fill 128 16bits fields in graphic_mode_list location with 0xFFFF data
    mov ax, VESA_MODE_LIST_PTR
    mov es, ax
    xor di, di
    mov ax, 0xFFFF
    mov cx, 128
    rep stosw

    mov ax, VESA_GLOBAL_INFO_PTR
    mov ds, ax
    mov si, vesa_Graph_Mode_Pointer              ; ATTENTION ! VideoModePtr est une liste de pointeurs exprimés en OFFSET:SEGMENT

    lodsw                                        ; charge le "mot" pointé par SI dans AX et incrémente SI de 2 si le drapeau de direction DF est à 0 : "LOaD Si Word"
    mov bx, ax

    lodsw                                        ; Le premier "lodsw" a chargé l'offset à appliquer à SI, le second charge le segment !
    mov ds, ax
    mov si, bx                                   ; Association de pointeur de données DS:SI sur l'endroit ou se trouve les différents modes graphiques.

; copie de tous les différents modes trouvés dans la variable de 256 octets _graphic_modes_list
    mov ax, VESA_MODE_LIST_PTR
    mov es, ax
    xor di, di
    mov cx, 126                                  ; Bloque le processus si 127 modes ont été trouvé. Celà évite un dépassement de _graphic_modes_list. (la dernière valeur doit rester 0xFFFF)

.cp_one_graph_mode:
    cmp cx, 0
    je .break_research_modes                     ; TROP DE MODES, on sort
    lodsw
    stosw

    dec cx
    cmp ax, 0xFFFF                               ; Le dernier mot dans la liste des modes doit être FFFF -> (end of list)
    jne .cp_one_graph_mode

.break_research_modes:
    mov ds, dx
    pop es
    pop ds
ret

query_vesa_mode_info:
; Demande de renseignements sur la capacité graphique 0x105, L'information de la granularité et du LFB est très importante ici
    push es
    mov ax, VESA_MODE_INFO_PTR
    mov es, ax
    xor di, di
    mov ax, 0x4F01
    mov cx, 0x4105               ; Ajoute au bit 14 de CX, la valeur 1 pour "être sur de tenir compte de "Linéar Frame Buffer"
    int 0x10
    pop es
ret

set_vesa_graphic:
;SWITCH TO VGA MODE NOW
    mov ax, 0x4F02
    mov bx, 0x105                ; 105H     1024x768     256  packed pixel
    int 0x10
ret

;---------------------------------------------------------
; Synopsis: Affiche une chaine de caracteres se terminant par 0x0
; Entree:   DS:SI -> pointe sur la chaine a afficher
;---------------------------------------------------------
afficher:
    push ax
    push bx
.debut:
    lodsb           ; ds:si -> al
    cmp al, 0       ; fin chaine ?
    jz .fin
    mov ah, 0x0E    ; appel au service 0x0e, int 0x10 du bios
    mov bx, 0x07    ; bx -> attribut, al -> caractere ascii
    int 0x10
    jmp .debut

.fin:
    pop bx
    pop ax
    ret

start:
    mov ax, 0x8000  ; stack en 0xFFFF
    mov ss, ax
    mov sp, 0xf000

; recuparation de l'unite de boot
    mov [bootdrv], dl

; affiche un msg
    mov si, msgDebut
    call afficher

; preparation du SVGA
    call check_vesa_capability
    cmp al, 0x4F
    jne no_vesa_card

    call copy_graphic_modes_buffer

    call query_vesa_mode_info

; reset drive
    xor ax, ax
    int 0x13

; loading of kernel
    push es
    mov ax, KERNEL_BASE_MEMORY
    mov es, ax
    xor bx, bx
    mov ah, 2
    mov al, KERNEL_NB_SECTORS
    xor ch, ch
    mov cl, KERNEL_OFFSET_ON_DISK
    xor dh, dh
    mov dl, [bootdrv]
    int 0x13
    jc disk_fatal_error
    pop es

; loading of fonts
    push es
    mov ax, FONT_BASE_MEMORY
    mov es, ax
    xor bx, bx
    mov ah, 2
    mov al, FONT_NB_SECTORS
    xor ch, ch
    mov cl, FONT_OFFSET_ON_DISK
    xor dh, dh
    mov dl, [bootdrv]
    int 0x13
    jc disk_fatal_error
    pop es

; bascule en SVGA
    call set_vesa_graphic

; initialisation du pointeur sur la GDT
    mov ax, gdtend    ; calcule la limite de GDT
    mov bx, gdt
    sub ax, bx
    mov word [gdtptr], ax

    xor eax, eax      ; calcule l'adresse lineaire de GDT
    xor ebx, ebx
    mov ax, ds
    mov ecx, eax
    shl ecx, 4
    mov bx, gdt
    add ecx, ebx
    mov dword [gdtptr+2], ecx

; CONFIGURATION VESA pour la gdt: Nous utiliserons le LFB. Il est donc necessaire de renseigner le descripteur de segment LFB de la gdt.
    push ds
    mov ax, VESA_MODE_INFO_PTR
    mov ds, ax
    mov si, flat_Memory
    lodsw
    mov dx, ax
    lodsw
    mov cx, ax
    pop ds

 ; Extraction des 16 bits de poid faible de la LFB et inscription dans BASE 0-->15 de la gtd_lfb [flat_Memory]
    mov [gdt_lfb + 2], dx

; Extraction des 16 bits de poid fort de la LFB: [flat_Memory + 2]
    mov [gdt_lfb + 4], cl           ;   - Mise des 8 bits faibles dans BASE 16 ---> 23 de la gtd_lfb
    mov [gdt_lfb + 7], ch           ;   - Mise des 8 bits forts dans BASE 24 ----> 31 de la gtd_lfb

; passage en modep
    cli
    lgdt [gdtptr]    ; charge la gdt
    mov eax, cr0
    or  ax, 1
    mov cr0, eax     ; PE mis a 1 (CR0)

    jmp next
next:
    mov ax, 0x10     ; segment de donne
    mov ds, ax
    mov fs, ax
    mov gs, ax
    mov es, ax
    mov ss, ax
    mov esp, 0x9F000

    jmp dword 0x8:0x1000 << 4    ; reinitialise le segment de code

;--------------------------------------------------------------------
gdt:
    db 0, 0, 0, 0, 0, 0, 0, 0
gdt_cs:
    db 0xFF, 0xFF, 0x0, 0x0, 0x0, 10011011b, 11011111b, 0x0
gdt_ds:
    db 0xFF, 0xFF, 0x0, 0x0, 0x0, 10010011b, 11011111b, 0x0
gdt_lfb:
    dw 0xFFFF, 0x0000
    db 0x00, 10010011b, 11011111b, 0xFC
gdtend:
;--------------------------------------------------------------------
gdtptr:
    dw 0  ; limite
    dd 0  ; base
;--------------------------------------------------------------------
bootdrv:  db 0
msgDebut: db "Loadig kernel...", 13, 10, 0
no_vesa_issue: db "No SVGA mode", 13, 10, 0
cannot_load_from_disk: db "Cannot load disk", 13, 10, 0
;--------------------------------------------------------------------

;; NOP jusqu'a 510
times 510-($-$$) db 144
dw 0xAA55
