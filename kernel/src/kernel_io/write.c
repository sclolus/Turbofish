
# include "kernel_io.h"
# include "vesa_graphic.h"
# include "libft.h"

#define MODIFIER_QUANTITY	13

struct modifier_list {
	char *string;
	int color;
};

static const struct modifier_list g_modifier_list[MODIFIER_QUANTITY] = {
	{ "\x1B[38m", 0x0},		// black
	{ "\x1B[0m",  0xFFFFFF },	// end of color
	{ "\x1B[31m", 0xFF0000 },	// red
	{ "\x1B[32m", 0x00FF00 },	// green
	{ "\x1B[33m", 0xFFFF00 },	// yellow
	{ "\x1B[34m", 0x0000FF },	// blue
	{ "\x1B[35m", 0xFF00FF },	// magenta
	{ "\x1B[36m", 0x00FFFF },	// cyan
	{ "\x1B[37m", 0xFFFFFF },	// white
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
			set_text_color(g_modifier_list[l].color);
			return len - 1;
		}
		l++;
	}
	return 0;
}

s32	write(s32 fd, const void *buf, u32 count)
{
	u8 *_buf;

	(void)fd;
	_buf = (u8 *)buf;
	switch (g_kernel_io_ctx.term_mode) {
	case boot:
		for (u32 i = 0; i < count; i++) {
			if (_buf[i] == '\x1B')
				i += extract_modifier(_buf + i);
			else
				graphic_putchar(_buf[i]);
		}
		//refresh_screen();
		break;
	case kernel:
		break;
	case user:
		break;
	default:
		break;
	}
	return 0;
}
