
#include "vesa_graphic.h"
#include "libft.h"

void	_main(struct graphic_ctx *graphic_ctx)
{
	ft_printf("value: %x\n", graphic_ctx->vesa_mode_info->framebuffer);

	char *b = (char *)graphic_ctx->vesa_mode_info->framebuffer;
	int h = 0;
	while (h++ < 100000)
		*b++ = h % 256;

	draw_line(0, 0, 1023, 768);
	draw_line(1023, 0, 0, 768);

	set_cursor_position(20, 20);
	ft_printf("test X");

	char *ptr = (char *)0x00008000;
	ft_printf(ptr);

	ft_putnbr_base(-0x1267ABEF, 16);
	ft_printf("\nSeparator\n");
	ft_putnbr_base(-0x1267ABEF, 16);
	ft_printf("\nSeparator\n");
	ft_putnbr_base(-0x1267ABEF, 16);
	ft_printf("\nSeparator\n");
	ft_putnbr_base(-0x1267ABEF, 16);
	ft_printf("\nSeparator\n");

	u16 *n = (u16 *)0x00008200;

	int z = 0;
	while (*n != 0xFFFF)
	{
		ft_putnbr_base(*n++, 16);
		z++;
		if (z % 4 == 0)
			ft_printf("\n");
		else
			ft_printf(" ");
	}
	ft_printf("\n");

	n = (u16 *)0x00008128;
	ft_putnbr_base(*n, 16);
	n++;
	ft_printf("\n");
	ft_putnbr_base(*n, 16);
	ft_printf("\n");
	ft_printf("un message\n");
	ft_printf("un autre message\n");
	ft_printf("et un dernier...\n");

	u32 i = 0x11BBCCDD;
	u32 j = 0x77020304;

	ft_printf("\n");
	ft_putnbr_base(i, 16);
	ft_memcpy(&i, &j, 4);
	ft_printf("\n");
	ft_putnbr_base(i, 16);
	ft_memset(&i, 0x22, 4);
	ft_printf("\n");
	ft_putnbr_base(i, 16);
	ft_bzero(&i, 4);
	ft_printf("\n");
	ft_putnbr_base(i, 16);
	ft_printf(" sizeof ");

	ft_memset((void *)0x0000F000, 0, 200);

	ft_printf("Les carotes sont cuites, sort %i %i %i = %#x\n", 3, 2, 1, 0xFFAA);

	ft_printf("{eoc}Les {red}carotes {green}sont {yellow}cuites, {blue}sort {magenta}%i {cyan}%i {white}%i {black}= {orange}%s\n", 1, 2, 3, " une gre des zegouts");

	ft_printf("{red}test {blue}2");

	while (1);
}
