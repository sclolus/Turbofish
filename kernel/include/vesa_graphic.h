#ifndef _VESA_GRAPHIC_
#define _VESA_GRAPHIC_

#include "i386_type.h"

extern void set_cursor_position(u32 x, u32 y);
extern void set_text_color(u8 color);
extern void putchar(char c);
extern void asm_printk(const char *s);

extern void draw_line(int x1, int y1, int x2, int y2);

#endif
