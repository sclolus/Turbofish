
#include "vesa_graphic.h"
#include "libft.h"

void putnbr_base(int n, int base);

# include <stdarg.h>

typedef struct					s_status
{
	va_list						ap;
	const char 					*s;
	int							fd;
	int							buff_len;
	int							total_size;
	char						*str;
}								t_status;

void _start(void)
{
	char *b = (char *)0xFD000000;
	int h = 0;
	while (h++ < 100000)
		*b++ = h % 256;

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

	u32 i = 0x11BBCCDD;
	u32 j = 0x77020304;

	asm_printk("\n");
	putnbr_base(i, 16);
	ft_memcpy(&i, &j, 4);
	asm_printk("\n");
	putnbr_base(i, 16);
	ft_memset(&i, 0x22, 4);
	asm_printk("\n");
	putnbr_base(i, 16);
	ft_bzero(&i, 4);
	asm_printk("\n");
	putnbr_base(i, 16);
	asm_printk(" sizeof ");
	putnbr_base(sizeof(t_status), 10);

	ft_memset((void *)0x0000F000, 0, 200);

	ft_printf("Les carotes sont cuites, sort %i %i %i = %#x\n", 3, 2, 1, 0xFFAA);

	ft_printf("Les carotes sont cuites, sort %i %i %i = %s\n", 1, 2, 3, " une gre des zegouts");

	while (1);
}
