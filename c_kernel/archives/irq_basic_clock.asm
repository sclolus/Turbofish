
[BITS 32]
    mov al,00110100b                  ;channel 0, lobyte/hibyte, rate generator
    out 0x43, al

    mov ax, 65535
    out 0x40,al                       ;Set low byte of PIT reload value
    mov al,ah                         ;ax = high 8 bits of reload value
    out 0x40,al                       ;Set high byte of PIT reload value
