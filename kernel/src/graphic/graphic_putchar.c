
#include "vesa_graphic.h"
#include "libft.h"

extern void display_char(u8 c, u32 edi);

static u32 g_cur_loc = 0;

static void	test_scroll(void)
{
	if (g_cur_loc == (g_graphic_ctx.vesa_mode_info.width
			* g_graphic_ctx.vesa_mode_info.height)) {
		u32 p = (u32)g_graphic_ctx.vesa_mode_info.framebuffer +
				g_graphic_ctx.vesa_mode_info.width * 16;

		memcpy(
			(ptr_32 *)g_graphic_ctx.vesa_mode_info.framebuffer,
			(ptr_32 *)p,
			g_graphic_ctx.vesa_mode_info.width *
			(g_graphic_ctx.vesa_mode_info.height - 16));

		p = (u32)g_graphic_ctx.vesa_mode_info.framebuffer +
				g_graphic_ctx.vesa_mode_info.width *
				(g_graphic_ctx.vesa_mode_info.height - 16);

		bzero((ptr_32 *)p, g_graphic_ctx.vesa_mode_info.width * 16);

		g_cur_loc -= g_graphic_ctx.vesa_mode_info.width * 16;
	}
}

// assume font is a 8 * 16 pixel bitmap
// screen resolution must be sub multiple of 8 for width and 16 for height
void		graphic_putchar(u8 c)
{
	if (c >= 32) {
		test_scroll();
		display_char(c, g_cur_loc);
		g_cur_loc += 8;
		if (g_cur_loc % g_graphic_ctx.vesa_mode_info.width == 0)
			g_cur_loc += 15 * g_graphic_ctx.vesa_mode_info.width;
	} else if (c == '\n') {
		g_cur_loc -= g_cur_loc % g_graphic_ctx.vesa_mode_info.width;
		g_cur_loc += 16 * g_graphic_ctx.vesa_mode_info.width;
		test_scroll();
	}
}

int		set_cursor_location(u32 x, u32 y)
{
	if (x >= g_graphic_ctx.vesa_mode_info.width >> 3)
		return -1;
	if (y >= g_graphic_ctx.vesa_mode_info.height >> 4)
		return -1;
	g_cur_loc = (x * 8) + (y * g_graphic_ctx.vesa_mode_info.width * 16);
	return 0;
}
