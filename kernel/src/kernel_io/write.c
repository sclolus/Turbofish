
# include "kernel_io.h"
# include "vesa_graphic.h"
# include "libft.h"

#define MODIFIER_QUANTITY	13

struct modifier_list {
	char *string;
	int color;
};

static const struct modifier_list g_modifier_list[MODIFIER_QUANTITY] = {
	{ "\x1B[0m", 7 },	// end of color
	{ "\x1B[31m", 4 },	// red
	{ "\x1B[32m", 2 },	// green
	{ "\x1B[33m", 14 },	// yellow
	{ "\x1B[34m", 1 },	// blue
	{ "\x1B[35m", 5 },	// magenta
	{ "\x1B[36m", 3 },	// cyan
	{ "\x1B[37m", 7 },	// white
	{ "\x1B[38m", 0},	// black
	{ "\x1B[39m", 6},	// orange
	{ "\x1B[40m", 8},	// grey
	{ "\x1B[41m", 9},	// deep blue
	{ "\x1B[42m", 10}	// light green
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
