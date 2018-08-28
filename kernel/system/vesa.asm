
[BITS 32]

GLOBAL copy_Vesa_Info

copy_Vesa_Info:
    push ebp
    mov ebp, esp

    mov esi, [ebp+8]            ; Contient l'addresse source du buffer VESA généré en mode ASM 16 bits via les interruptions BIOS.
    mov edi, [ebp+12]           ; Contient l'addresse de destination de la structure VESA codée en C

    mov ecx, 0x80
    rep movsd

    mov esp, ebp
    pop ebp
ret
