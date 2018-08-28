
#include "../../kernel/lib/types.h"

#define NULL				0
#define GDTBASE				0x0                                                 // -> La base de la GDT est au secteur 0 de la mémoire.
#define GDT_MAX_DESCRIPTORS		256
#define GDT_DESCRIPTOR_SIZE		8
#define GDT_MAX_SIZE 			GDT_MAX_DESCRIPTORS*GDT_DESCRIPTOR_SIZE

struct  gdt_seg {
    u16     limit_15_0;
    u16     base_15_0;
    u8      base_23_16;
    u8      access;
    u8      limit_19_16 : 4;
    u8      other       : 4;
    u8      base_31_24;
} __attribute__ ((packed));

struct __attribute__ ((packed)) global_Descriptor_Table {
    struct gdt_seg  segment[4];
} GDT;

void setGdtSegment(u8 segment, u32 base, u32 limit, u8 access, u8 other)
{
	u32 *ptr = NULL;
	ptr += segment << 1;					 		// ptr += (0x800 + (segment << 3)) >> 2;		GDTBASE est donc divisé par 4.

	*ptr = ((base  & 0x0000FFFF) << 16) + (limit & 0x0000FFFF); 	ptr++;	// Algo des pointeurs: Le plus petit sera le 1er, le plus grand le dernier. [case 0: limit & 0x0000FFFF] [case 1: base  & 0x0000FFFF]
	*ptr = ((base & 0x00FF0000) >> 16) + (access << 8) + (limit & 0xF0000) + (other << 20) + (base & 0xFF000000);
/* 			case 0: (base & 0x00FF0000) >> 16 	soit 'base16_23'			0x000000FF
			case 2: (access << 8)	         						0x0000FF00
			case 4: (limit & 0xF0000)		soit 'limit16_19'			0x000F0000
			case 5: other									0x00F00000
			case 6: (base & 0xFF000000)		soit 'base24_32'			0xFF000000							*/
return;
}

void _setGdtSegment(u8 nb_segment, u32 base, u32 limit, u8 access, u8 other)
{
	struct gdt_seg *segment = &GDT.segment[nb_segment];

	segment->limit_15_0 = limit & 0x0000FFFF;
	segment->limit_19_16 = (limit & 0x000F0000) >> 16;
	segment->base_15_0 = base & 0x0000FFFF;
	segment->base_23_16 = (base & 0x00FF0000) >> 16;
	segment->base_31_24 = (base & 0xFF000000) >> 24;
	segment->access = access;
	segment->other = other & 0xF;

	return;
}

struct __attribute__ ((packed)) {
    u16     limit;
    u32     base;
} global_descriptor_table_ptr;

void init_GDT(int LFB)                /*** Cette fonction publique recoit la valeur du LFB en argument afin de na pas perdre la main sur l'interface graphique. ***/
{
	_setGdtSegment(0,0,0,0,0);                                         // NULL segment
	_setGdtSegment(1,0x00000000,0xFFFFFF,0b10011011,0b1101);           // CS segment
	_setGdtSegment(2,0x00000000,0xFFFFFF,0b10010011,0b1101);           // DATA segment
	_setGdtSegment(3,LFB,0xFFFFFF,0b10010011,0b1101);                  // LFB segment
//	_setGdtSegment(4,0x00000000,0x000000,0b10010111,0b1101);           /* STACK segment le segment de pile a une base et une limite qui sont à 0 ! Dans un segment de pile (expand down)
//	                                                                    ,la base n'est pas interprétée, elle est donc ici mise à 0. */

	//global_descriptor_table_ptr.base = GDTBASE;				//global_descriptor_table_ptr.base = (int)&GDT;
	//global_descriptor_table_ptr.limit = 40;			        //global_descriptor_table_ptr.limit = sizeof(GDT);

	global_descriptor_table_ptr.base = (int)&GDT;
	global_descriptor_table_ptr.limit = sizeof(GDT);

	asm("lgdtl (global_descriptor_table_ptr)    \n \
		movw $0x10, %ax                        \n \
		movw %ax, %ds                          \n \
		movw %ax, %es                          \n \
		movw %ax, %fs                          \n \
		movw %ax, %gs                          \n \
		ljmp $0x08, $next                      \n \
		next:                                  \n");

	while (1);

return;
}

/*
gdt:
    db 0, 0, 0, 0, 0, 0, 0, 0                                       ; DESCRIPTEUR NULL

gdt_cs: ; SEGMENT DE CODE STANDARD
    dw 0xFFFF, 0x0000                                               ; limite 15-0: 0xFFFF    base 15-0: 0x0000                          -> base  (32b) 0x0000:0000
    db 0x0, 0b10011011, 0b11011111, 0x0                             ; base 16-23: 0x00       limite 16-19: 0b1111    base 24-31: 0x00   -> limit (20b) 0xF:FFFF
                  ;type
gdt_ds: ; SEGMENT DE DONNES STANDARD
    dw 0xFFFF, 0x0000                                               ; limite 15-0: 0xFFFF    base 15-0: 0x0000                          -> base  (32b) 0x0000:0000
    db 0x00, 0b10010011, 0b11011111, 0x00                           ; base 16-23: 0x00       limite 16-19: 0b1111    base 24-31: 0x00   -> limit (20b) 0xF:FFFF
                  ;type
gdt_lfb: ; SEGMENT DE LA LFB, COMME UN SEGMENT DE DONNEES sauf que la BASE est mise à celle de la LFB de la carte graphique !
    dw 0xFFFF, 0x0000                                               ; limite 15-0: 0xFFFF    base 15-0: 0x0000                          -> base   (32b) 0xFC00:0000
    db 0x00, 10010011b, 11011111b, 0xFC                             ; base 16-23: 0x00       limite 16-19: 0b1111    base 24-31: 0xFC   -> limite (20b) 0xF:FFFF

gdtend:

global_descriptor_table_ptr:    ; pointeur de la GDT sur 48 bits, ou 6 octets.
    dw 0  ; limite
    dd 0  ; base
*/
