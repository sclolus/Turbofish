
[BITS 32]

PCI_ADDR_IOPORT		    EQU	0x0CF8
PCI_DATA_IOPORT         EQU 0x0CFC

PCI_Classes:
dd 00h, PCI_BASE_NOT_DEFINED
dd 01h, PCI_BASE_CLASS_STORAGE
dd 02h, PCI_BASE_CLASS_NETWORK
dd 03h, PCI_BASE_CLASS_DISPLAY
dd 04h, PCI_BASE_CLASS_MULTIMEDIA
dd 05h, PCI_BASE_CLASS_MEMORY
dd 06h, PCI_BASE_CLASS_BRIDGE
dd 07h, PCI_BASE_CLASS_COMMUNICATION
dd 08h, PCI_BASE_CLASS_SYSTEM
dd 09h, PCI_BASE_CLASS_INPUT
dd 0Ah, PCI_BASE_CLASS_DOCKING
dd 0Ch, PCI_BASE_CLASS_PROCESSOR
dd 0Ch, PCI_BASE_CLASS_SERIAL

dd 0xFF, PCI_BASE_NOT_DEFINED

PCI_BASE_NOT_DEFINED            db "INTERN", 0
PCI_BASE_CLASS_STORAGE		    db "STORAGE",0
PCI_BASE_CLASS_NETWORK		    db "NETWORK",0
PCI_BASE_CLASS_DISPLAY		    db "DISPLAY",0
PCI_BASE_CLASS_MULTIMEDIA	    db "MULTIMEDIA",0
PCI_BASE_CLASS_MEMORY		    db "MEMORY",0
PCI_BASE_CLASS_BRIDGE		    db "BRIDGE",0
PCI_BASE_CLASS_COMMUNICATION    db "COMMUNICATION",0
PCI_BASE_CLASS_SYSTEM		    db "SYSTEM",0
PCI_BASE_CLASS_INPUT		    db "INPUT",0
PCI_BASE_CLASS_DOCKING		    db "DOCKING",0
PCI_BASE_CLASS_PROCESSOR	    db "PROCESSOR",0
PCI_BASE_CLASS_SERIAL		    db "SERIAL",0


PCI_Sub_Classes:
dd 0000h, PCI_CLASS_NOT_DEFINED
dd 0001h, PCI_CLASS_NOT_DEFINED_VGA
dd 0100h, PCI_CLASS_STORAGE_SCSI
dd 0101h, PCI_CLASS_STORAGE_IDE
dd 0102h, PCI_CLASS_STORAGE_FLOPPY
dd 0103h, PCI_CLASS_STORAGE_IPI
dd 0104h, PCI_CLASS_STORAGE_RAID
dd 0180h, PCI_CLASS_STORAGE_OTHER

dd 0200h, PCI_CLASS_NETWORK_ETHERNET
dd 0201h, PCI_CLASS_NETWORK_TOKEN_RING
dd 0202h, PCI_CLASS_NETWORK_FDDI
dd 0203h, PCI_CLASS_NETWORK_ATM
dd 0280h, PCI_CLASS_NETWORK_OTHER

dd 0300h, PCI_CLASS_DISPLAY_VGA
dd 0301h, PCI_CLASS_DISPLAY_XGA
dd 0380h, PCI_CLASS_DISPLAY_OTHER

dd 0400h, PCI_CLASS_MULTIMEDIA_VIDEO
dd 0401h, PCI_CLASS_MULTIMEDIA_AUDIO
dd 0480h, PCI_CLASS_MULTIMEDIA_OTHER

dd 0500h, PCI_CLASS_MEMORY_RAM
dd 0501h, PCI_CLASS_MEMORY_FLASH
dd 0580h, PCI_CLASS_MEMORY_OTHER

dd 0600h, PCI_CLASS_BRIDGE_HOST
dd 0601h, PCI_CLASS_BRIDGE_ISA
dd 0602h, PCI_CLASS_BRIDGE_EISA
dd 0603h, PCI_CLASS_BRIDGE_MC
dd 0604h, PCI_CLASS_BRIDGE_PCI
dd 0605h, PCI_CLASS_BRIDGE_PCMCIA
dd 0606h, PCI_CLASS_BRIDGE_NUBUS
dd 0607h, PCI_CLASS_BRIDGE_CARDBUS
dd 0680h, PCI_CLASS_BRIDGE_OTHER

dd 0700h, PCI_CLASS_COMM_SERIAL
dd 0701h, PCI_CLASS_COMM_PARALLEL
dd 0780h, PCI_CLASS_COMM_OTHER

dd 0800h, PCI_CLASS_SYSTEM_PIC
dd 0801h, PCI_CLASS_SYSTEM_DMA
dd 0802h, PCI_CLASS_SYSTEM_TIMER
dd 0803h, PCI_CLASS_SYSTEM_RTC
dd 0880h, PCI_CLASS_SYSTEM_OTHER

dd 0900h, PCI_CLASS_INPUT_KEYBOARD
dd 0901h, PCI_CLASS_INPUT_PEN
dd 0902h, PCI_CLASS_INPUT_MOUSE
dd 0980h, PCI_CLASS_INPUT_OTHER

dd 0A00h, PCI_CLASS_DOCKING_GENERIC
dd 0A01h, PCI_CLASS_DOCKING_OTHER

dd 0B00h, PCI_CLASS_PROCESSOR_386
dd 0B01h, PCI_CLASS_PROCESSOR_486
dd 0B02h, PCI_CLASS_PROCESSOR_PENTIUM
dd 0B10h, PCI_CLASS_PROCESSOR_ALPHA
dd 0B20h, PCI_CLASS_PROCESSOR_POWERPC
dd 0B40h, PCI_CLASS_PROCESSOR_CO

dd 0C00h, PCI_CLASS_SERIAL_FIREWIRE
dd 0C01h, PCI_CLASS_SERIAL_ACCESS
dd 0C02h, PCI_CLASS_SERIAL_SSA
dd 0C03h, PCI_CLASS_SERIAL_USB
dd 0C04h, PCI_CLASS_SERIAL_FIBER

dd 0xFFFF, PCI_CLASS_NOT_DEFINED

PCI_CLASS_NOT_DEFINED		    db "NOT_DEFINED", 0
PCI_CLASS_NOT_DEFINED_VGA	    db "NOT_DEFINED_VGA", 0
PCI_CLASS_STORAGE_SCSI          db "STORAGE_SCSI", 0
PCI_CLASS_STORAGE_IDE           db "STORAGE_IDE", 0
PCI_CLASS_STORAGE_FLOPPY        db "STORAGE_FLOPPY", 0
PCI_CLASS_STORAGE_IPI           db "STORAGE_IP",0
PCI_CLASS_STORAGE_RAID          db "STORAGE_RAID",0
PCI_CLASS_STORAGE_OTHER         db "STORAGE_OTHER",0

PCI_CLASS_NETWORK_ETHERNET      db "NETWORK_ETHERNET",0
PCI_CLASS_NETWORK_TOKEN_RING    db "NETWORK_TOKEN_RING",0
PCI_CLASS_NETWORK_FDDI          db "NETWORK_FDDI",0
PCI_CLASS_NETWORK_ATM           db "NETWORK_ATM",0
PCI_CLASS_NETWORK_OTHER         db "NETWORK_OTHER",0

PCI_CLASS_DISPLAY_VGA           db "DISPLAY_VGA",0
PCI_CLASS_DISPLAY_XGA           db "DISPLAY_XGA",0
PCI_CLASS_DISPLAY_OTHER         db "DISPLAY_OTHER",0

PCI_CLASS_MULTIMEDIA_VIDEO      db "MULTIMEDIA_VIDEO",0
PCI_CLASS_MULTIMEDIA_AUDIO      db "MULTIMEDIA_AUDIO",0
PCI_CLASS_MULTIMEDIA_OTHER      db "MULTIMEDIA_OTHER",0

PCI_CLASS_MEMORY_RAM            db "MEMORY_RAM",0
PCI_CLASS_MEMORY_FLASH          db "MEMORY_FLASH",0
PCI_CLASS_MEMORY_OTHER          db "MEMORY_OTHER",0

PCI_CLASS_BRIDGE_HOST           db "BRIDGE_HOST",0
PCI_CLASS_BRIDGE_ISA            db "BRIDGE_ISA",0
PCI_CLASS_BRIDGE_EISA           db "BRIDGE_EISA",0
PCI_CLASS_BRIDGE_MC             db "BRIDGE_MC",0
PCI_CLASS_BRIDGE_PCI            db "BRIDGE_PCI",0
PCI_CLASS_BRIDGE_PCMCIA         db "BRIDGE_PCMCIA",0
PCI_CLASS_BRIDGE_NUBUS          db "BRIDGE_NUBUS",0
PCI_CLASS_BRIDGE_CARDBUS        db "BRIDGE_CARDBUS",0
PCI_CLASS_BRIDGE_OTHER          db "BRIDGE_OTHER",0

PCI_CLASS_COMM_SERIAL           db "COMM_SERIAL",0
PCI_CLASS_COMM_PARALLEL         db "COMM_PARALLEL",0
PCI_CLASS_COMM_OTHER            db "COMM_OTHER",0

PCI_CLASS_SYSTEM_PIC            db "SYSTEM_PIC",0
PCI_CLASS_SYSTEM_DMA            db "SYSTEM_DMA",0
PCI_CLASS_SYSTEM_TIMER          db "SYSTEM_TIMER",0
PCI_CLASS_SYSTEM_RTC            db "SYSTEM_RTC",0
PCI_CLASS_SYSTEM_OTHER          db "SYSTEM_OTHER",0

PCI_CLASS_INPUT_KEYBOARD        db "INPUT_KEYBOARD",0
PCI_CLASS_INPUT_PEN             db "INPUT_PEN",0
PCI_CLASS_INPUT_MOUSE           db "INPUT_MOUSE",0
PCI_CLASS_INPUT_OTHER           db "INPUT_OTHER",0

PCI_CLASS_DOCKING_GENERIC       db "DOCKING_GENERIC",0
PCI_CLASS_DOCKING_OTHER         db "DOCKING_OTHER",0

PCI_CLASS_PROCESSOR_386         db "PROCESSOR_386",0
PCI_CLASS_PROCESSOR_486         db "PROCESSOR_486",0
PCI_CLASS_PROCESSOR_PENTIUM     db "PROCESSOR_PENTIUM",0
PCI_CLASS_PROCESSOR_ALPHA       db "PROCESSOR_ALPHA",0
PCI_CLASS_PROCESSOR_POWERPC     db "PROCESSOR_POWERPC",0
PCI_CLASS_PROCESSOR_CO          db "PROCESSOR_CO",0

PCI_CLASS_SERIAL_FIREWIRE       db "SERIAL_FIREWIRE",0
PCI_CLASS_SERIAL_ACCESS         db "SERIAL_ACCESS",0
PCI_CLASS_SERIAL_SSA            db "SERIAL_SSA",0
PCI_CLASS_SERIAL_USB            db "SERIAL_USB",0
PCI_CLASS_SERIAL_FIBER          db "SERIAL_FIBER",0


GLOBAL lspci

extern print

bus dd 0
slot dd 0
function dd 0
command dd 0

data db "PCI bus:%i slot:%i function:%i VENDOR_ID %x DEVICE_ID %x -> %s", 10, 0
result db "founded", 10, 0

GLOBAL  assign_pci_class
GLOBAL  assign_pci_subclass


assign_pci_class:
    push ebp
    mov ebp, esp
    mov eax, [ebp + 8]

    mov esi, PCI_Classes
class_search:
        cmp al, [esi]
je class_founded
        add esi, 8
        cmp byte [esi], 0xFF
jne class_search

class_founded:
    mov eax, [esi + 4]
    mov esp, ebp
    pop ebp
ret


assign_pci_subclass:
    push ebp
    mov ebp, esp
    mov eax, [ebp + 8]

    mov esi, PCI_Sub_Classes
subclass_search:
        cmp eax, [esi]
je subclass_founded
        add esi, 8
        cmp word [esi], 0xFFFF
jne subclass_search

subclass_founded:
    mov eax, [esi + 4]

    mov esp, ebp
    pop ebp
ret


lspci:

_cycle:
        xor eax, eax
        mov eax, 0x80000000

        mov ebx, [bus]
        shl ebx, 16
        add eax, ebx

        mov ebx, [slot]
        shl ebx, 11
        add eax, ebx

        mov ebx, [function]
        shl ebx, 8
        add eax, ebx

        mov ebx, [command]
        shl ebx, 2
        add eax, ebx

        mov ecx, eax                    ; Sauvegarde de EAX dans ECX

        mov dx, PCI_ADDR_IOPORT
        out dx, eax

        mov dx, PCI_DATA_IOPORT
        in  eax, dx

        cmp eax, 0xFFFFFFFF
je test_function

        xchg eax, ebx                   ; Sauvegarde du resultat dans EBX

        mov eax, ecx                    ; récupération du ponteur EAX
        add eax, 0x08

        mov dx, PCI_ADDR_IOPORT
        out dx, eax

        mov dx, PCI_DATA_IOPORT
        in  eax, dx

        mov ecx, eax                    ; Sauvegarde du réslutat dans ECX
        shr eax, 16

        mov esi, PCI_Sub_Classes
search:
        cmp eax, [esi]
        je founded
        add esi, 8
        cmp word [esi], 0xFFFF
jne search

founded:
        push dword [esi + 4]

        push ecx
        push ebx

        push dword [function]
        push dword [slot]
        push dword [bus]
        push data
        call print
        add esp, 28

incremente_function:
        inc dword [function]
        cmp dword [function], 8
jne _cycle

incremente_slot:
        mov dword [function], 0
        inc dword [slot]
        cmp dword [slot], 32
jne _cycle

incremente_bus:
        mov dword [slot], 0
        inc dword [bus]
        cmp dword [bus], 16
jne _cycle
ret

test_function:
    cmp dword [function], 0
je incremente_slot
jmp incremente_function



;val32 =
;      0x80000000
;    | bus << 16
;    | slot << 11
;    | function << 8
;    | register << 2
;
;
;                         |  |      PCI BUS       |  |                  32 bits
;                    __________________________________________
;                   | |||| |||| |||| |||| |||| |||| |||| |||| |
;                     |||| |||| |||| |||| |||| |||| |||| ||||
; 0x8000        -> 0x 1000 0000 ---- ---- ---- ---- ---- ----
; bus << 16     -> 0x ---- ---- BBBB BBBB ---- ---- ---- ----   -> 256 values   0x00 -> 0xFF
; slot << 11    -> 0x ---- ---- ---- ---- BBBB B--- ---- ----   ->  32 values   0x00 -> 0x20
; function << 8 -> 0x ---- ---- ---- ---- ---- -BBB ---- ----   ->   7 values   0x00 -> 0x08
; register << 2 -> 0x ---- ---- ---- ---- ---- ---- BBBB BB--   ->  64 values   0x00 -> 0x40
