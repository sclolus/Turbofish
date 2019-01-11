
#include "vesa.h"
#include "libft.h"
#include "libasm_i386.h"

/*
 * default color to blank
 */
static u32 text_color = 0x00FFFFFF;

extern u8 _print_graphical_char_begin;

static void	display_char_24(u8 c, u8 *location)
{
	u8 *bitmap;
	u8 line;

	bitmap = (u8 *)&_print_graphical_char_begin + (c << CHAR_SHL);
	for (int i = 0; i < CHAR_HEIGHT; i++) {
		line = *bitmap;
		for (int j = 0; j < CHAR_WIDTH; j++) {
			if (line & 0x80) {
				location[0] = text_color & 0xFF;
				location[1] = (text_color >> 8) & 0xFF;
				location[2] = (text_color >> 16) & 0xFF;
			}
			line <<= 1;
			location += 3;
		}
		location += vesa_ctx.edi_offset;
		bitmap++;
	}
}

static void	display_char_32(u8 c, u32 *location)
{
	u8 *bitmap;
	u8 line;

	bitmap = (u8 *)&_print_graphical_char_begin + (c << 4);
	for (int i = 0; i < CHAR_HEIGHT; i++) {
		line = *bitmap;
		for (int j = 0; j < CHAR_WIDTH; j++) {
			if (line & 0x80)
				*location = text_color;
			line <<= 1;
			location++;
		}
		location += (vesa_ctx.edi_offset >> 2);
		bitmap++;
	}
}

void		set_text_color(u32 color)
{
	text_color = color;
}

u32		get_text_color(void)
{
	return text_color;
}

/*
 * assume font is a 8 * 16 pixel bitmap
 * screen resolution must be sub multiple of 8 for width and 16 for height
 */

int		graphic_putchar(u8 c, u8 *addr)
{
	if (c >= 32) {
		if (vesa_ctx.mode.bpp == 24)
			display_char_24(c, addr);
		else
			display_char_32(c, (u32 *)addr);
	}
	return 0;
}
