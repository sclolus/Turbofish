
#ifndef __VGA_TEXT_H__
# define __VGA_TEXT_H__

/*
 * This file contain a minimal set of VGA_TEXT functions
 */

#include "i386_type.h"

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


int set_text_color(enum text_color color);
int set_cursor_position(size_t x, size_t y);
void scroll_screen();
void clear_screen();

#endif
