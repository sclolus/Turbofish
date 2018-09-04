
#include "i386_type.h"
#include "int8086.h"
#include "vesa_graphic.h"
#include "base_system.h"
#include "libft.h"

#define VESA_GLOBAL_INFO_PTR 0xA000
#define VESA_MODE_INFO_PTR 0xB000

# define HEX_T(x)	"0123456789ABCDEF"[x]

void		clear_screen(void)
{
/* this loops clears the screen
	* there are 25 lines each of 80 columns; each element takes 2 bytes */

	u32 j = 0;
	u32 screensize = 80 * 25 * 2;
	/* video memory begins at address 0xb8000 */
	char *vidptr = (char*)0xb8000;
	while (j < screensize) {
		/* blank character */
		vidptr[j] = ' ';
		/* attribute-byte */
		vidptr[j+1] = 0x07;
		j = j + 2;
	}
}

void		term_putchar(char c)
{
	static int i = 0;

	/* video memory begins at address 0xb8000 */
	char *vidptr = (char*)0xb8000;

	/* the character's ascii */
	vidptr[i++] = c;
	/* attribute-byte: give character black bg and light grey fg */
	vidptr[i++] = 0x07;
}

void		term_putstr(char *str)
{
	while (*str != '\0')
	{
		term_putchar(*str);
		str++;
	}
}

void 		kmain(void)
{
	struct registers reg;

	reg.eax = 0x4F00;
	reg.edi = VESA_GLOBAL_INFO_PTR;
	int8086(0x10, reg);

	struct vesa_global_info *vgi = (struct vesa_global_info *)VESA_GLOBAL_INFO_PTR;

	reg.eax = 0x4F01;
	reg.ecx = 0x4105;				// Ajoute au bit 14 de CX, la valeur 1 pour "être sur de tenir compte de "Linéar Frame Buffer"
	reg.edi = VESA_MODE_INFO_PTR;
	int8086(0x10, reg);

	struct vesa_mode_info *vmi = (struct vesa_mode_info *)VESA_MODE_INFO_PTR;

/*
// sequence d'extinction du PC
	reg.eax = 0x530E;
	reg.ebx = 0x102;
	int8086(0x15, reg);

	reg.eax = 0x5300;
	reg.ebx = 0x0;
	int8086(0x15, reg);

	reg.eax = 0x5301;
	reg.ebx = 0x0;
	int8086(0x15, reg);

	reg.eax = 0x530E;
	reg.ebx = 0x102;
	int8086(0x15, reg);

	reg.eax = 0x5307;
	reg.ebx = 0x1;
	reg.ecx = 0x3;
	int8086(0x15, reg);
*/

	clear_screen();

	term_putstr(" 0x");

	init_GDT(vmi->framebuffer);

	reg.eax = 0x4F02;
	reg.ebx = 0x105;
	int8086(0x10, reg);

	ft_putstr(vgi->vesa_Signature);
	ft_printf("%#x", vmi->framebuffer);

	putchar('-');
	putchar('-');
	putchar('-');
	set_text_color(1);
	putchar('H');
	set_text_color(2);
	putchar('E');
	set_text_color(3);
	putchar('L');
	set_text_color(4);
	putchar('L');
	set_text_color(5);
	putchar('O');
	putchar(' ');
	set_text_color(6);
	putchar('W');
	set_text_color(7);
	putchar('O');
	set_text_color(8);
	putchar('R');
	set_text_color(9);
	putchar('L');
	set_text_color(10);
	putchar('D');

	draw_line(0, 0, 1023, 768);
	draw_line(1023, 0, 0, 768);

	ft_putnbr_base(-0x1267ABEF, 16);
	ft_printf("\nSeparator\n");
	ft_putnbr_base(-0x1267ABEF, 16);
	ft_printf("\nSeparator\n");
	ft_putnbr_base(-0x1267ABEF, 16);
	ft_printf("\nSeparator\n");
	ft_putnbr_base(-0x1267ABEF, 16);
	ft_printf("\nSeparator\n");

	ft_printf("Les carotes sont cuites");
	ft_printf("Les carotes sont cuites, sort %i %i %i = %#x\n", 3, 2, 1, 0xFFAA);
	ft_printf("{eoc}Les {red}carotes {green}sont {yellow}cuites, {blue}sort {magenta}%i {cyan}%i {white}%i {black}= {orange}%s\n", 1, 2, 3, " une gre des zegouts");
	ft_printf("{red}test {blue}2");
	return;
}

