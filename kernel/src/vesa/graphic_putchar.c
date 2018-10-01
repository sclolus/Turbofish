
#include "vesa.h"
#include "libft.h"
#include "libasm_i386.h"

extern void display_char_24(u8 c, u32 edi);
extern void display_char_32(u8 c, u32 edi);

static u32 g_cur_loc = 0;

#define CHAR_HEIGHT	16

static void	test_scroll(void)
{
	if (g_cur_loc == (vesa_ctx.mode.pitch * vesa_ctx.mode.height)) {
		u32 p = (u32)DB_FRAMEBUFFER_ADDR +
				vesa_ctx.mode.pitch * CHAR_HEIGHT;

		/*
		memcpy(
			(ptr_32 *)vesa_ctx.mode.framebuffer,
			(ptr_32 *)p,
			vesa_ctx.mode.pitch *
			(vesa_ctx.mode.height - CHAR_HEIGHT);
		*/

		sse2_memcpy(
			(void *)DB_FRAMEBUFFER_ADDR,
			(void *)p,
			vesa_ctx.mode.pitch *
			(vesa_ctx.mode.height - CHAR_HEIGHT));

		p = (u32)DB_FRAMEBUFFER_ADDR + vesa_ctx.mode.pitch *
				(vesa_ctx.mode.height - CHAR_HEIGHT);

		bzero((ptr_32 *)p, vesa_ctx.mode.pitch * CHAR_HEIGHT);

		g_cur_loc -= vesa_ctx.mode.pitch * CHAR_HEIGHT;

		refresh_screen();
	}
}

#define CHAR_WIDTH 8
#define CHAR_HEIGHT 16
#define CHAR_SHL 4

extern u32 g_edi_offset;
extern u8 _print_graphical_char_begin;
extern u32 text_color;

void		_display_char_24(u8 c, u8 *location)
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
		location += g_edi_offset;
		bitmap++;
	}
}

void		_display_char_32(u8 c, u32 *location)
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
		location += g_edi_offset;
		bitmap++;
	}
}

// assume font is a 8 * 16 pixel bitmap
// screen resolution must be sub multiple of 8 for width and 16 for height
void		graphic_putchar(u8 c)
{
	if (c >= 32) {
		test_scroll();
		if (vesa_ctx.mode.bpp == 24)
			_display_char_24(
					c,
					(u8 *)(DB_FRAMEBUFFER_ADDR + g_cur_loc));
		else
			display_char_32(c, DB_FRAMEBUFFER_ADDR + g_cur_loc);
		g_cur_loc += vesa_ctx.mode.bpp;
		if (g_cur_loc % vesa_ctx.mode.pitch == 0)
			g_cur_loc += (CHAR_HEIGHT - 1) * vesa_ctx.mode.pitch;
	} else if (c == '\n') {
		g_cur_loc -= g_cur_loc % (vesa_ctx.mode.pitch);
		g_cur_loc += CHAR_HEIGHT * vesa_ctx.mode.pitch;
		test_scroll();
	}
}

int		set_cursor_location(u32 x, u32 y)
{
	if (x >= vesa_ctx.mode.width >> 3)
		return -1;
	if (y >= vesa_ctx.mode.height >> 4)
		return -1;
	g_cur_loc = (x * vesa_ctx.mode.bpp)
			+ (y * vesa_ctx.mode.pitch * CHAR_HEIGHT);
	return 0;
}
