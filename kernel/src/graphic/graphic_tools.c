
#include "vesa_graphic.h"

static void f_win32(u32 *dst, u32 size, u8 red, u8 green, u8 blue)
{
	u32 color;

	color = ((red << 16) & 0xFF0000) | ((green << 8) & 0xFF00) | blue;
	while (size--)
		*dst++ = color;
}

static void f_win24(u8 *dst, u32 size, u8 red, u8 green, u8 blue)
{
	while (size--) {
		*dst++ = blue;
		*dst++ = green;
		*dst++ = red;
	}
}

void	fill_window(u8 red, u8 green, u8 blue)
{
	u32 *dst;
	u32 size;

	dst = (uint32_t *)vesa_ctx.mode.framebuffer;
	size = vesa_ctx.mode.width * vesa_ctx.mode.height;

	if (vesa_ctx.mode.bpp == 32)
		f_win32((u32 *)dst, size, red, green, blue);
	else
		f_win24((u8 *)dst, size, red, green, blue);
}
