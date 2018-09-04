
#include "i386_type.h"
#include "vesa_graphic.h"
#include "base_system.h"
#include "libft.h"

// this loops clears the screen
// there are 25 lines each of 80 columns; each element takes 2 bytes
void		reset_text_screen(void)
{
	struct registers	reg;
	char			*vidptr;
	u32			j;
	u32			screensize;

	// video memory begins at address 0xb8000
	vidptr = (char*)0xb8000;

	// set cursor 	AH=02h 	BH = page number, DH = Line, DL = Colomn
	reg.edx = 0;
	reg.ebx = 0;
	reg.eax = 0x02;
	int8086(0x10, reg);

	j = 0;
	screensize =  80 * 25 * 2;
	while (j < screensize)
	{
		vidptr[j] = ' ';	// black character
		vidptr[j + 1] = 0x07;	// attribute-byte
		j = j + 2;
	}
}

void		text_putstr(char *str)
{
	// video memory begins at address 0xb8000
	char *vidptr = (char*)0xb8000;
	static int i = 0;

	while (*str != '\0')
	{
		vidptr[i++] = *str++;	// the character's ascii
		vidptr[i++] = 0x07;	// give char black bg and light grey fg
	}
}

void 		kmain(void)
{
	if (set_vbe(0x105) < 0)
	{
		reset_text_screen();
		text_putstr("KERNEL_FATAL_ERROR: Cannot set VBE mode");
		return ;
	}

	set_cursor_location(1, 1);

	ft_printf("width: %hu, height: %hu, bpp: %hhu ",
			g_graphic_ctx.vesa_mode_info.width,
			g_graphic_ctx.vesa_mode_info.height,
			g_graphic_ctx.vesa_mode_info.bpp);

	struct vesa_graphic_mode_list *vgml =
		&g_graphic_ctx.vesa_graphic_mode_list;

	for (u32 i = 0; i < vgml->nb_mode; i++)
		ft_printf("%#hx ", vgml->mode[i]);
	ft_printf("NB MODES = %u  ", vgml->nb_mode);

	ft_putstr(g_graphic_ctx.vesa_global_info.vesa_Signature);

	ft_printf("%#x", g_graphic_ctx.vesa_mode_info.framebuffer);

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

