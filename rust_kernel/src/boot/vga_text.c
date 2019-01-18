
#include "i386_type.h"
#include "libft.h"

enum text_color {
	red,
	green,
	yellow,
	cyan,
	brown,
	magenta,
	blue,
	white,
	undefined,
};

struct vga_mode {
	u8 *memory_location;
	size_t width;
	size_t height;
	size_t x;
	size_t y;
	u8 color;
};

static struct vga_mode vga =
	{memory_location: (u8 *)0xb8000, width: 80, height: 25, x: 0, y: 0, color: 3};

void putchar(char c) {
	u8 *ptr = vga.memory_location;
	size_t pos = vga.x + vga.y * vga.width;

	ptr[pos * 2] = c;
	ptr[pos * 2 + 1] = vga.color;
}

void scroll_screen() {
	u8 *ptr = vga.memory_location;

	memmove((void *)ptr, (void *)(ptr + vga.width * 2), vga.width * (vga.height - 1) * 2);
	memset((void *)(ptr + (vga.width * (vga.height - 1) * 2)), 0, vga.width * 2);
	vga.y -= 1;
}

void clear_screen() {
	memset(vga.memory_location, 0, vga.width * vga.height * 2);
	vga.x = 0;
	vga.y = 0;
}

int set_cursor_position(size_t x, size_t y) {
	if (x >= vga.width || y >= vga.height) {
		return -1;
	} else {
		vga.x = x;
		vga.y = y;
		return 0;
	}
}

int set_text_color(enum text_color color) {
	switch (color) {
	case blue:
		vga.color = 11;
		break;
	case green:
		vga.color = 10;
		break;
	case yellow:
		vga.color = 14;
		break;
	case cyan:
		vga.color = 3;
		break;
	case red:
		vga.color = 4;
		break;
	case magenta:
		vga.color = 13;
		break;
	case white:
		vga.color = 7;
		break;
	default:
		return -1;
		break;
	}
	return 0;
}

#define MODIFIER_QUANTITY 13

struct modifier_list {
	char *s;
	enum text_color color;
};

const struct modifier_list modifier_list[MODIFIER_QUANTITY] = {
	{ "\x1B[0m",  white },    // end of color
	{ "\x1B[31m", red },      // red
	{ "\x1B[32m", green },    // green
	{ "\x1B[33m", yellow },   // yellow
	{ "\x1B[34m", blue },     // blue
	{ "\x1B[35m", magenta },  // magenta
	{ "\x1B[36m", cyan },	  // cyan
	{ "\x1B[37m", white },	  // white
	{ "\x1B[38m", undefined}, // black
	{ "\x1B[39m", undefined}, // orange
	{ "\x1B[40m", undefined}, // grey
	{ "\x1B[41m", undefined}, // deep blue
	{ "\x1B[42m", undefined}, // light green
};

static u32	extract_modifier(const char *buf)
{
	int l;

	l = 0;
	while (l < MODIFIER_QUANTITY) {
		size_t len = strlen(modifier_list[l].s);
		if (memcmp(modifier_list[l].s, buf, len) == 0) {
			set_text_color(modifier_list[l].color);
			return len - 1;
		}
		l++;
	}
	return 0;
}

int write(int fd, char *buf, size_t len) {
	(void)fd;
	for (size_t i = 0; i < len; i++) {
		if (buf[i] == '\x1B') {
			i += extract_modifier(buf + i);
		} else if (buf[i] == '\n') {
			vga.x = 0;
			vga.y = vga.y + 1;
			if (vga.y == vga.height) {
				scroll_screen();
			}
		} else {
			putchar(buf[i]);
			vga.x = vga.x + 1;
			if (vga.x == vga.width) {
				vga.x = 0;
				vga.y = vga.y + 1;
				if (vga.y == vga.height) {
					scroll_screen();
				}
			}
		}
	}
	return 0;
}
