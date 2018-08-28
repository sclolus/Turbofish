
[BITS 32]

GLOBAL init_PIC

; Initialisation des PICs maitre et esclave gérant les interruptions, une temporisation via jump est nécessaire quand on initialise ICW1 ... 2 .. 3 et 4 en chaîne.
init_PIC:											; Le registre ICW1 est le tout premier vecteur d'initialisation
    mov al, 0x11  ; Initialisation de ICW1					; (7...5) reserved -> mis à 0		(4) OBLIGATOIRE 1
    out 0x20, al  ; maître								; (3) LITM mode déclenchement des interruption, 0 pour front (utilisé ici) et 1 déclenchement par niveau
    out 0xA0, al  ; esclave								; (2) ADI interval d'addresage (spécifique au 8085) 0 ici donc	(1) SNGL 0 -> 1 seul pic ET 1 -> 2 pics maitre et esclave.
jmp ICW2        ; temporisation							; (0) IC4, 1 si ICW4 à venir -> 1
ICW2:
    mov al, 0x20										; Le registre ICW2 informe de l'index de la première interruption vers la table IDT
    out 0x21, al  ; maître, vecteur de départ = 32				; Les 32 premières interruptions étant reservées au processeur, IRQ0 sera traité par le vecteur 32 dans la tablez IDT
    mov al, 0x70
    out 0xA1, al  ; esclave, vecteur de départ = 112				; Les IRQ8 -> 15 activeront les vecteurs 112 à 119 dans la tabke d'interruption IDT
jmp ICW3        ; temporisation
ICW3:
    mov al, 0x04 ; maître								; l'ICW3 renseigne les Pics sur comment ils sont connectés entres-eux.
    out 0x21, al
    mov al, 0x02 ; esclave								; Le standard est de mettre l'esclave sur l'IRQ2 et le maitre sur l'IRQ4
    out 0xA1, al
jmp ICW4        ; temporisation							; l'ICW4 informe les pics sur comment ils vont fonctionner
ICW4:												; (7..6) reserved -> mis à 0		(4) SFNM 0 = Fully Nested Mode mode de fonctionnement normal à 1
    mov al, 0x05										; (3)	BUF reserved -> mis à 0		(2) M/S -> 1 pour le maitre et 0 pour l'esclave1
    out 0x21, al										; (1) AEOI Auto End of Interrupt (fin automatique d'interruption), 0 = Normal (0) 0 = 8085, 1 = 8086
    mov al, 0x01
    out 0xA1, al
jmp OCW1        ; temporisation							; registre de masquage: une interruption est masqué si son masque est à 1, se lit comme IRQ7 -> IRQ6 [...] -> IRQ1 -> IRQ0
OCW1:
    in  al, 0x21  ; lecture de l'Interrupt Mask Register (IMR)
    or al, 0xF8   ; 0xEF => 11101111b. débloque l'IRQ 4					keyboard only: 1111 1101 	=> 0xFD		Keyboard & clock: 1111 1100	=> 0xFC     keyboard, clock, slave 1111 1000 -> 0xF8
    out 0x21, al  ; recharge l'IMR

    in  al, 0xA1  ; lecture de l'Interrupt Mask Register (IMR)		; contrôleur esclave: IRQ15 -----> IRQ8
    or al, 0xEF   ; master -> 1110 1111 -> 0xEF
    out 0xA1, al  ; recharge l'IMR
ret
