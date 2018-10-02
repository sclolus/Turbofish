
# include "kernel_io.h"
# include "vesa.h"
# include "libft.h"
# include "libasm_i386.h"

u32 g_cur_loc;

const struct modifier_list g_modifier_list[MODIFIER_QUANTITY] = {
	{ "\x1B[0m",  0xFFFFFF },	// end of color
	{ "\x1B[31m", 0xFF0000 },	// red
	{ "\x1B[32m", 0x00FF00 },	// green
	{ "\x1B[33m", 0xFFFF00 },	// yellow
	{ "\x1B[34m", 0x0000FF },	// blue
	{ "\x1B[35m", 0xFF00FF },	// magenta
	{ "\x1B[36m", 0x00FFFF },	// cyan
	{ "\x1B[37m", 0xFFFFFF },	// white
	{ "\x1B[38m", 0x0},		// black
	{ "\x1B[39m", 0xFFA500},	// orange
	{ "\x1B[40m", 0x808080},	// grey
	{ "\x1B[41m", 0x00BFFF},	// deep blue
	{ "\x1B[42m", 0x7FFF00}		// light green
};

u32	extract_modifier(const u8 *buf)
{
	int l;

	l = 0;
	while (l < MODIFIER_QUANTITY) {
		size_t len = strlen(g_modifier_list[l].string);
		if (memcmp(g_modifier_list[l].string, buf, len) == 0) {
			if (l != 0)
				set_text_color(g_modifier_list[l].color);
			else
				set_text_color(g_kernel_io_ctx.current_tty->
						default_color);
			return len - 1;
		}
		l++;
	}
	return 0;
}

static void	update_line(u32 location)
{
	sse2_memcpy(
			(void *)location + vesa_ctx.mode.framebuffer,
			(void *)location + DB_FRAMEBUFFER_ADDR,
			vesa_ctx.mode.pitch * CHAR_HEIGHT);
}

static void	test_scroll(void)
{
	if (g_cur_loc < (vesa_ctx.mode.pitch * vesa_ctx.mode.height))
		return ;

	fill_tty_background(g_kernel_io_ctx.current_tty);
	set_cursor_location(0, 0);
	copy_tty_content(g_kernel_io_ctx.current_tty);
	refresh_screen();



	g_cur_loc -= vesa_ctx.mode.pitch - CHAR_HEIGHT;
}

void	write_char(u8 c, int direct)
{
	if (direct == 0)
		add_tty_char(c);

	if (c == '\n') {

		g_cur_loc -= g_cur_loc % (vesa_ctx.mode.pitch);
		if (direct == 0) {
			update_line(g_cur_loc);
			new_tty_line();
		}

		g_cur_loc += CHAR_HEIGHT * vesa_ctx.mode.pitch;
		if (direct == 0)
			test_scroll();
		return ;
	}
	if (direct == 0)
		test_scroll();

	graphic_putchar(c, (u8 *)(DB_FRAMEBUFFER_ADDR + g_cur_loc));

	g_cur_loc += vesa_ctx.mode.bpp;
	if (g_cur_loc % vesa_ctx.mode.pitch == 0) {

		g_cur_loc -= vesa_ctx.mode.pitch;
		if (direct == 0) {
			update_line(g_cur_loc);
			new_tty_line();
		}
		g_cur_loc += CHAR_HEIGHT * vesa_ctx.mode.pitch;
	}
}

s32	write(s32 fd, const void *buf, u32 count)
{
	u8 *_buf;

	(void)fd;
	_buf = (u8 *)buf;
	switch (g_kernel_io_ctx.term_mode) {
	case boot:
		/*
		 * wanted fall
		 */
	case kernel:
		for (u32 i = 0; i < count; i++) {
			if (_buf[i] == '\x1B') {

				u8 o = extract_modifier(_buf + i);
				for (int j = 0; j < (o + 1); j++)
					add_tty_char(_buf[i + j]);
				i += o;
			} else {
				write_char(_buf[i], 0);
			}
		}
		break;
	case user:
		break;
	default:
		break;
	}
	return 0;
}

s32	write_direct(s32 fd, const u8 *buf, u32 count)
{
	for (u32 i = 0; i < count; i++) {
		if (buf[i] == '\x1B') {
			i += extract_modifier(buf + i);
		} else
			write_char(buf[i], 1);
	}
	(void)fd;
	return 0;
}

int	set_cursor_location(u32 x, u32 y)
{
	if (x >= vesa_ctx.mode.width >> 3)
		return -1;
	if (y >= vesa_ctx.mode.height >> 4)
		return -1;
	g_cur_loc = (x * vesa_ctx.mode.bpp)
			+ (y * vesa_ctx.mode.pitch * CHAR_HEIGHT);
	return 0;
}
