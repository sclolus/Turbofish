
[BITS 16]

extinction_du_pc: ; Plusieurs sous-fonctions dans l'interruption 15h sont necessaires pour une extinction totale !
mov			ax, 0x5300
xor			bx, bx
INT			0x15

mov			ax, 0x5301
xor			bx, bx
INT			0x15

mov			ax, 0x530E
mov			cx, 0x0102
INT			0x15

mov			ax, 0x5307
mov			bx, 0x0001
mov			cx, 0x0003
INT			0x15


freeze_process:
    mov ah, 0x86
    mov cx, 0xFFFF
    int 15h
jmp freeze_process
