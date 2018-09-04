
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

// for the moment, only mode in 8bpp work. 0x100 0x101 0x103 0x105 0x107
#define VBE_MODE 0x103

void 		kmain(void)
{
	if (set_vbe(VBE_MODE) < 0)
	{
		reset_text_screen();
		text_putstr("KERNEL_FATAL_ERROR: Cannot set VBE mode");
		bios_wait(5);
		bios_shutdown_computer();
		return ;
	}

	ft_printf("{white}Kernel loaded: {green}OK\n{eoc}");
	ft_printf("{white}VBE initialized: {green}OK\n{eoc}");
	ft_printf("{white}GDT loaded: {green}OK\n{eoc}");
	ft_printf("{white}Available graphic mode:\n{eoc}");

	struct vesa_graphic_mode_list *vgml =
		&g_graphic_ctx.vesa_graphic_mode_list;


	ft_printf("{orange}");
	u32 max_cap = g_graphic_ctx.vesa_mode_info.width / 8 / 8;
	u32 i = 0;
	while (i < vgml->nb_mode)
	{
		ft_printf("0x%.4hx ", vgml->mode[i]);
		i++;
		ft_printf((i % max_cap == 0) ? "\n" : " ");
	}
	if (i % max_cap != 0)
		ft_printf("\n");
	ft_printf("{eoc}");

	ft_printf("{white}Selected mode: {green}%#x\n{eoc}", VBE_MODE);
	ft_printf("-> width: {green}%hu{eoc}, height:"
			" {green}%hu{eoc}, bpp: {green}%hhu{eoc}\n",
			g_graphic_ctx.vesa_mode_info.width,
			g_graphic_ctx.vesa_mode_info.height,
			g_graphic_ctx.vesa_mode_info.bpp);
	ft_printf("-> linear frame buffer location: {green}%#x{eoc}\n",
			g_graphic_ctx.vesa_mode_info.framebuffer);
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
	return;
}

