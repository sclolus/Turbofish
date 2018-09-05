[BITS 32]

segment .text
GLOBAL init_PIC
init_PIC:
    push ebp
    mov ebp, esp

    jmp .icw_1

; |0|0|0|1|x|0|x|x|
;          |   | +--- avec ICW4 (1) ou sans (0)
;          |   +----- un seul contrôleur (1), ou cascadés (0)
;          +--------- declenchement par niveau (level) (1) ou par front (edge) (0)
.icw_1:
    mov al, 0x11
    out 0x20, al  ; master
    out 0xA0, al  ; slave
    jmp .icw_2

; |x|x|x|x|x|0|0|0|
; | | | | |
; +----------------- base address of interrupt vectors
.icw_2:           ; Set vector offset. IRQ below 32 are processor reserved IRQ
    mov al, 0x20
    out 0x21, al  ; master, begin at 32 (to 39)
    mov al, 0x70
    out 0xA1, al  ; slave, begin at 112 (to 119)
    jmp .icw_3

.icw_3:           ; set how are connected pic master and slave
; |x|x|x|x|x|x|x|x|  pour le maître
;  | | | | | | | |
;  +------------------ contrôleur esclave rattaché à la broche d'interruption (1), ou non (0)
    mov al, 0x04  ; master is connector 3 of slave
    out 0x21, al

; |0|0|0|0|0|x|x|x|  pour l'esclave
;            | | |
;            +-------- Identifiant de l'esclave, qui correspond au numéro de broche IR sur le maître
    mov al, 0x02  ; slave is connector 2 of master
    out 0xA1, al
    jmp .icw_4

; |0|0|0|x|x|x|x|1|
;        | | | +------ mode "automatic end of interrupt" AEOI (1)
;        | | +-------- mode bufferisé esclave (0) ou maître (1)
;        | +---------- mode bufferisé (1)
;        +------------ mode "fully nested" (1)
.icw_4:
    mov al, 0x01
    out 0x21, al
    out 0xA1, al
    jmp .ocw_1

; |x|x|x|x|x|x|x|x|
;  | | | | | | | |
;  +-+-+-+-+-+-+-+---- pour chaque IRQ : masque d'interruption établi (1) ou non (0)
.ocw_1:           ; Interrupt mask
    in al, 0x21   ; get Interrupt Mask Register (IMR)
    and al, 0xF8  ; 0xF8 => 11111000b. unlock IRQ0, IRQ1 and IRQ2(slave connexion)
    out 0x21, al  ; store IMR

    in al, 0xA1   ; get Interrupt Mask Register (IMR)
    and al, 0xEF  ; 0xEF => 11101111b. unlock IRQ12 (master connexion)
    out 0xA1, al  ; store IMR
    jmp .end_init_pic
.end_init_pic:

    pop ebp
ret
