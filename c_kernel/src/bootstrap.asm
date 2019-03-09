[BITS 32]

extern virtual_offset

segment .bootstrap.text

extern alt_gdt_new
extern alt_bootstrap_main

extern _init

GLOBAL bootstrap_begin
bootstrap_begin:
    ; SET the first stack
    ; This mechanism is for Panic handler. See details on 'panic.rs' file
    ; dont worry about overflow for stack, the first push will be at [temporary_stack - 4], not in [temporary_stack]
    mov [temporary_stack - 4], dword 0x0
    mov esp, temporary_stack - 8
    mov ebp, esp

    ; SET the first GDT
    ; reserve 8 bytes for structure return
    sub esp, 8
    ; this function return a structure on stack and pop four bytes
    call alt_gdt_new
    lgdt [eax]
    add esp, 4

    mov ax, 0x10
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ax, 0x18
    mov ss, ax

    jmp 0x8: .set_cs
.set_cs:

    ; TODO SET THE IDT (IT COULD BE A GOOD IDEA)

    sub esp, 8
    ; push the pointer to the multiboot grub structure
    push ebx
    call alt_bootstrap_main
    add esp, 8

    ; TODO SET PAGING HERE INSTEAD OF INTO THE CALLED FUNCTION

    lea eax, [_init]
    sub eax, dword virtual_offset

    call eax

segment .bootstrap.data
align 16

; 4ko for a temporary stack
times 1 << 12 db 0
temporary_stack:
