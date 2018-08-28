
#include "../../kernel/lib/types.h"

#define NULL 0
#define IDTBASE 0x0800
#define IDTSIZE 256
#define INTGATE 0x8E00

void _asm_irq_default_base();
void _asm_irq_default_master();
void _asm_irq_default_slave();

void _asm_irq_clock();
void irq_keyboard();

/***
struct idt_seg {									// descripteur de segment IDT
    u16 	offset0_15;
    u16 	select;
    u16 	type;
    u16 	offset16_31;
} __attribute__ ((packed));

struct __attribute__ ((packed)) idt_Descriptor_Table {		// Structure d'une table IDT de (IDTSIZE) segments.
    struct idt_seg  segment[IDTSIZE];
} IDT;s
***/

void init_idt_seg(u8 segment, u32 offset, u16 select , u16 type) 	// Cette fonction écrit DIRECTEMENT EN MEMOIRE les différents segments de L'IDT à partir de 0x0000:0000 à 0x0000:0800
{
	u32 *ptr = NULL;				// Pointeur NULL, pointe sur la location mémoire 0x0000:0000, IDTBASE étant à 0.
	ptr += (IDTBASE >> 2) + (segment << 1);					// Le pointeur étant de 32 bits, pour 8 octets, on doit multiplier segment par 2.
									// pointeur initialisé en 0, puis 8, puis 16. indique un offset en octet sur la mémoire à partir de la base. 0000:0000 , 0000:0008, 0000:0010, 0000:0018 ...

	*ptr =  (select << 16) | (offset & 0x0000FFFF); // remplissage des 4 premiers octets.
	ptr++;							// une incrémentation de 1 d'un pointeur de int 32 bits incrémente en réalité le pointeur de 32 bits soit de 4 octets !
	*ptr = (offset & 0xFFFF0000) + type;		// remplassage des 4 octets suivant.
return;
}

struct __attribute__ ((packed)) {
	u16     limit;
	u32     base;
} Idt_Descriptor_Table_Ptr;

void init_IDT()
{
	u16 i;
	for (i=0; i<IDTSIZE; i++) 	init_idt_seg(i, (u32)_asm_irq_default_base, 0x8, INTGATE);		// IRQ par défault.

	init_idt_seg(32, (u32)_asm_irq_clock	, 0x8, INTGATE);						// IRQ 0 Clock
	init_idt_seg(33, (u32)irq_keyboard	, 0x8, INTGATE);						// IRQ 1 Keyboard
	init_idt_seg(34, (u32)_asm_irq_default_master	, 0x8, INTGATE);						// IRQ 2
	init_idt_seg(35, (u32)_asm_irq_default_master	, 0x8, INTGATE);						// IRQ 3
	init_idt_seg(36, (u32)_asm_irq_default_master	, 0x8, INTGATE);						// IRQ 4
	init_idt_seg(37, (u32)_asm_irq_default_master	, 0x8, INTGATE);						// IRQ 5
	init_idt_seg(38, (u32)_asm_irq_default_master	, 0x8, INTGATE);						// IRQ 6
	init_idt_seg(39, (u32)_asm_irq_default_master	, 0x8, INTGATE);						// IRQ 7

	init_idt_seg(112, (u32)_asm_irq_default_slave	, 0x8, INTGATE);						// IRQ 8
	init_idt_seg(113, (u32)_asm_irq_default_slave	, 0x8, INTGATE);						// IRQ 9
	init_idt_seg(114, (u32)_asm_irq_default_slave	, 0x8, INTGATE);						// IRQ 10
	init_idt_seg(115, (u32)_asm_irq_default_slave	, 0x8, INTGATE);						// IRQ 11
	init_idt_seg(116, (u32)_asm_irq_default_slave	, 0x8, INTGATE);						// IRQ 12
	init_idt_seg(117, (u32)_asm_irq_default_slave	, 0x8, INTGATE);						// IRQ 13
	init_idt_seg(118, (u32)_asm_irq_default_slave	, 0x8, INTGATE);						// IRQ 14
	init_idt_seg(119, (u32)_asm_irq_default_slave	, 0x8, INTGATE);						// IRQ 15


	Idt_Descriptor_Table_Ptr.base 	= IDTBASE;
	Idt_Descriptor_Table_Ptr.limit 	= IDTSIZE << 3;							// Taille de cette dernière.

	asm(" lidt (Idt_Descriptor_Table_Ptr) ");
return;
}

