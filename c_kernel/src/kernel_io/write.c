
# include "kernel_io.h"
# include "vesa.h"
# include "libft.h"
# include "libasm_i386.h"

static u32 g_cur_loc;

const struct modifier_list modifier_list[MODIFIER_QUANTITY] = {
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

/*
 * Common extract modifier from string
 */
static u32	extract_modifier(const u8 *buf)
{
	int l;

	l = 0;
	while (l < MODIFIER_QUANTITY) {
		size_t len = strlen(modifier_list[l].string);
		if (ft_memcmp(modifier_list[l].string, buf, len) == 0) {
			if (l != 0)
				set_text_color(modifier_list[l].color);
			else
				set_text_color(kernel_io_ctx.current_tty->
						default_color);
			return len - 1;
		}
		l++;
	}
	return 0;
}

/*
 * Copy a line chunk from DOUBLE FRAMEBUFFER to LINEAR FRAMEBUFFER
 */
static void	update_line(u32 location)
{
	_sse2_memcpy(
			(void *)location + vesa_ctx.mode.framebuffer,
			(void *)location + DB_FRAMEBUFFER_ADDR,
			vesa_ctx.mode.pitch * CHAR_HEIGHT);
}

/*
 * Test if a scroll is necessary and process it if it must be done
 */
static void	test_scroll(void)
{
	if (g_cur_loc < (u32)(vesa_ctx.mode.pitch * vesa_ctx.mode.height))
		return ;

	fill_tty_background(kernel_io_ctx.current_tty);
	set_cursor_location(0, 0);
	copy_tty_content(kernel_io_ctx.current_tty);
	refresh_screen();

	g_cur_loc -= vesa_ctx.mode.pitch - CHAR_HEIGHT;
}

/*
 * Indirect write fill the tty buffer, Direct not.
 */
static void	write_char(u8 c, int direct)
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

/*
 * Common write method
 * May be utilized like the WRITE SYSCALL in UNIX system, the PRINTK method
 * use this function.
 * All operation are recorded into tty structure
 */
s32	write(s32 fd, const void *buf, u32 count)
{
	u8 *_buf;

	(void)fd;
	_buf = (u8 *)buf;
	switch (kernel_io_ctx.term_mode) {
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
	case panic_screen:
		write_direct(fd, (u8 *)buf, count);
		break;
	case user:
		break;
	default:
		break;
	}
	return 0;
}

/*
 * Direct write into the double frame buffer
 * The tty structure is not filled, that function may be utilized to refresh
 * a tty content, or for an end screen like panic()
 */
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

/*
 * Manually set the cursor location
 * This function is not compatible with tty writing, it's only used for one
 * screen write like panic()
 */
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
