
#include "vesa_graphic.h"

void putnbr_base(int n, int base);

void _start(void)
{
	char *j = (char *)0xFD000000;
	int h = 0;
	while (h++ < 100000)
		*j++ = h % 256;

	draw_line(0, 0, 1023, 768);
	draw_line(1023, 0, 0, 768);

	set_cursor_position(20, 20);
	asm_printk("test X");

	char *ptr = (char *)0x00008000;
	asm_printk(ptr);

	putnbr_base(-0x1267ABEF, 16);
	asm_printk("\nSeparator\n");
	putnbr_base(-0x1267ABEF, 16);
	asm_printk("\nSeparator\n");
	putnbr_base(-0x1267ABEF, 16);
	asm_printk("\nSeparator\n");
	putnbr_base(-0x1267ABEF, 16);
	asm_printk("\nSeparator\n");

	u16 *n = (u16 *)0x00008200;

	int z = 0;
	while (*n != 0xFFFF)
	{
		putnbr_base(*n++, 16);
		z++;
		if (z % 4 == 0)
			asm_printk("\n");
		else
			asm_printk(" ");
	}
	asm_printk("\n");

	n = (u16 *)0x00008128;
	putnbr_base(*n, 16);
	n++;
	asm_printk("\n");
	putnbr_base(*n, 16);
	asm_printk("\n");
	asm_printk("un message\n");
	asm_printk("un autre message\n");
	asm_printk("et un dernier...\n");
	while (1);
}
