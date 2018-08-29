[BITS 32]

segment .data

segment .text

global iddle_mode

iddle_mode:
    hlt                 ; Cette instruction permet de faire passer le microprocesseur en mode d'arrêt.
JMP iddle_mode          ; Toutefois, le processeur peut quitter cet état lorsqu'une ligne matérielle RESET ou lorsqu'une interruption non-masquable (NMI) reçoit un signal.
