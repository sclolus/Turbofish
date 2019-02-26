
	; This function allow to call a BIOS 16bits real mode interrupt from 32 bits protection mode

	; C:    extern u32 real_mode_op(struct BaseRegisters reg, u16 bios_int);
	; RUST: extern "C" { pub fn real_mode_op(reg: BaseRegisters, bios_int: u16) -> u32; }

	; pub struct BaseRegisters {
	;     /*0        |*/ pub edi:u32,
	;     /*4        |*/ pub esi:u32,
	;     /*8        |*/ pub ebp:u32,
	;     /*12       |*/ pub esp:u32,
	;     /*16       |*/ pub ebx:u32,
	;     /*20       |*/ pub edx:u32,
	;     /*24       |*/ pub ecx:u32,
	;     /*28       |*/ pub eax:u32,
	;     /*32       |*/
	; }

	; CAUTION
	; If enabled, The PIC must be disabled before calling this code
	; For the moment, there is undefined behavior where paging is enable before calling this code

	; Rust calling convention i386: The first parameter is close than EIP, obviously, access is EBP + 8
	; for example, fn (x:u32. y:u32) -> u32, after pushing EBP, x is on (EBP + 8) and y is on (EBP + 12)
	; the classical return is the EAX register. Caution: Never return structure between two compiler
	; ---------
	; | ARG 2 |
	; +-------+
	; | ARG 1 |
	; +-------+
	; |  EIP  | (EIP of caller)
	; +-------+
	; |  EBP  | (After pushing EBP by first instruction of ASM function)
	; +-------+

	; 32 bits protected to real 16 bits steps by steps
	; ----------------------------------------------------------------------------------------------------------------------------------------
	; Disable the interrupts:
	;     Turn off maskable interrupts using CLI.
	;     Disable NMI (optional).
	; Turn off paging: IMPORTANT: In this below impementation Paging is shut down at the same moment of protected mode. CR3 fflush is juste before it.
	;     Transfer control to a 1:1 page.
	;     Ensure that the GDT and IDT are in a 1:1 page.
	;     Clear the PG-flag in the zeroth control register.
	;     Set the third control register to 0.
	; Use GDT with 16-bit tables (skip this step if one is already available):
	; Create a new GDT with a 16-bit data and code segment:
	;     Limit: 0xFFFFF
	;     Base: 0x0
	;     16-bit
	;     Privilege level: 0
	;     Granularity: 0
	;     Read and Write: 1
	; Load new GDT ensuring that the currently used selectors will remain the same (index in cs/ds/ss will be copy of original segment in new GDT)
	; Far jump to 16-bit protected mode:
	;     Far jump to 16-bit protected mode with a 16-bit segment index.
	;     Load data segment selectors with 16-bit indexes:
	;     Load ds, es, fs, gs, ss with a 16-bit data segment.
	; Load real mode IDT:
	;     Limit: 0x3FF
	;     Base 0x0
	;     Use lidt
	; Disable protected mode: IMPORTANT: In this below implementation, paging is deactivate with Protected mode, in the same moment.
	;     Set PE bit in CR0 to false.
	;     Far jump to real mode:
	;     Far jump to real mode with real mode segment selector (usually 0).
	;     Reload data segment registers with real mode values:
	;     Load ds, es, fs, gs, ss with appropriate real mode values (usually 0).
	; Set stack pointer to appropriate value:
	;     Set sp to stack value that will not interfere with real mode program.
	; Enable interrupts:
	;     Enable maskable interrupts with STI.
	;     Continue on in real mode with all bios interrupts.

[BITS 32]
segment .text

%define ALL_REGISTERS_OFFSET 32 ; popad and pushas modufication offset for esp

; POPAD and PUSHAD operations conerned ALL registers except ESP, which is normal behav +32, -32

%define BASE_LOCATION 0x7C00    ; Payload will be copied at that address
%define REBASE(x)     (BASE_LOCATION + x - begin_sub_sequence)

GLOBAL _real_mode_op
_real_mode_op:
	push ebp
	mov ebp, esp

	; preserve all caller registers
	pushad

	; copy of content at BASE_LOCATION
	mov eax, end_sub_sequence
	sub eax, begin_sub_sequence
	mov ecx, eax
	mov esi, begin_sub_sequence
	mov edi, BASE_LOCATION
	rep movsb

	; initialise temporary GDT
	mov eax, gdt_16_end
	sub eax, gdt_16
	mov word [REBASE(gdt_16_ptr)], ax

	; store linear address of GDT
	mov eax, gdt_16
	mov dword [REBASE(gdt_16_ptr + 2)], eax

	; fill the number of the interupt to launch
	mov al, [ebp + 8 + ALL_REGISTERS_OFFSET]
	mov byte [REBASE(begin_sub_sequence.int_nb_location)], al

	; put ESP on the first argument
	add esp, 8 + ALL_REGISTERS_OFFSET

	; Get all arguments registers
	popad

	sub esp, 8 + ALL_REGISTERS_OFFSET + ALL_REGISTERS_OFFSET

	; recovery of current EBP : (esp is preserved by popad operation)
	push eax
	mov eax, [esp + 12]
	mov ebp, eax
	pop eax

	; push a address to join after execution with instruction ret
	call BASE_LOCATION

end_real_mode_op:
	; store return EAX
	mov [esp + 28], eax

	; restore all registers values
	popad

	pop ebp
ret

	; -------------------------------------------------
	; *** This part is copied in BASE_LOCATION area ***
	; -------------------------------------------------
begin_sub_sequence:
	; saving of all data segments register
	mov [REBASE(_ds)], ds
	mov [REBASE(_es)], es
	mov [REBASE(_fs)], fs
	mov [REBASE(_gs)], gs
	mov [REBASE(_ss)], ss

	; saving of current CS segment
	mov [REBASE(.cs_value_location)], cs

	; store AX parameter
	mov [REBASE(_eax)], eax

	; store CR3 parameter
	mov eax, cr3
	mov [REBASE(_cr3)], eax

	; store caller idt and load BIOS idt
	sidt [REBASE(saved_idtptr)]
	lidt [REBASE(bios_idt)]

	; store caller gdt and load custom 16 bits gdt
	sgdt [REBASE(saved_gdtptr)]
	lgdt [REBASE(gdt_16_ptr)]

	; jump to CS of 16 bits selector
	jmp 0x8:REBASE(.protected_16)
.protected_16:
	; code is now in 16bits, because we are in 16 bits mode
[BITS 16]

	; set 16 bits protected mode data selector
	mov  ax, 0x10
	mov  ds, ax
	mov  es, ax
	mov  fs, ax
	mov  gs, ax
	mov  ss, ax

	; disable paging (PG) && protected bit
	mov eax, cr0
	and eax, 0x7ffffffe
	mov cr0, eax

	; fflush CR3 register
	xor eax, eax
	mov cr3, eax

	; configure CS in real mode
	jmp 0x0:REBASE(.real_16)
.real_16:

	; configure DS, ES and SS in real mode
	xor ax, ax
	mov ds, ax
	mov es, ax
	mov ss, ax

	; take saved eax
	mov eax, [REBASE(_eax)]

	; enable interupts
	sti

	; launch interupt 0xCD is the opcode of INT
	db 0xCD
.int_nb_location:
	db 0x0

	; disable interupt
	cli

	; load caller idt and caller gdt
	lidt [REBASE(saved_idtptr)]
	lgdt [REBASE(saved_gdtptr)]

	; entering in protected mode
	mov ebx, cr0
	or  bx, 1
	mov cr0, ebx     ; PE set to 1 (CR0)

	; configure CS in protected mode
	; Eq: jmp 0x8:REBASE(.protected_32) with CS value as 0x8
	; 0xEA is the opcode of long jump -> jmp ptr16:u16
	db 0xEA
	dw REBASE(.protected_32)
.cs_value_location:
	dw 0xFFFF
.protected_32:

	; code is now in 16bits
[BITS 32]
	; restore all segments registers
	mov ds, [REBASE(_ds)]
	mov es, [REBASE(_es)]
	mov fs, [REBASE(_fs)]
	mov gs, [REBASE(_gs)]
	mov ss, [REBASE(_ss)]

	; restore Paging
	mov ebx, [REBASE(_cr3)] 	; restore CR3 Page directory Phy address location
	mov cr3, ebx

	mov ebx, cr0
	or ebx, 0x80000001          ; restore PG bit (Protected bit must be enable with it)
	mov cr0, ebx

	; return to base function
	ret

bios_idt:
	dw 0x3ff ; limit
	dd 0     ; base
saved_idtptr:
	dw 0
	dd 0
saved_gdtptr:
	dw 0     ; limit
	dd 0     ; base

_cr3: dd 0
_eax: dd 0
_ds: dw 0
_es: dw 0
_fs: dw 0
_gs: dw 0
_ss: dw 0

gdt_16:
	db 0, 0, 0, 0, 0, 0, 0, 0
.gdt_16b_cs:
	dw 0xFFFF, 0x0000
	db 0x00, 0x9A, 0x0, 0x0
.gdt_16b_ds:
	dw 0xFFFF, 0x0000
	db 0x00, 0x92, 0x0, 0x0
gdt_16_end:

gdt_16_ptr:
	dw 0     ; limit
	dd 0     ; base

end_sub_sequence:
