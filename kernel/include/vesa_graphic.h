
#ifndef __VESA_GRAPHIC_H__
# define __VESA_GRAPHIC_H__

#include "i386_type.h"

s32		set_vbe(u16 selected_mode);
int		set_cursor_location(u32 x, u32 y);
void		graphic_putchar(u8 c);

void		fill_window(u8 red, u8 green, u8 blue);

int		bmp_load(u8 *file_offset, int *width, int *height, int **data);

extern void	set_text_color(u32 pix_color);
extern void	draw_line(u32 x1, u32 y1, u32 x2, u32 y2);

struct __attribute__ ((packed)) vesa_global_info {
	char	vesa_Signature[4];
	u16	vesa_version;
	char	oem_name[4];
	u32	capability_flag;
	u16	list_supported_mode_offset;
	u16	list_supported_mode_segment;
	u16	memory_ammount;
	u8	vbe_2_field[236];
};

/*
 * definition took from OSDEV https://forum.osdev.org/viewtopic.php?f=2&t=30186
 */
struct __attribute__ ((packed)) vesa_mode_info {
	u16 attributes;		// deprecated, only bit 7 should be of interest
				// to you, and it indicates the mode supports a
				// linear frame buffer.
	u8 window_a;		// deprecated
	u8 window_b;		// deprecated
	u16 granularity;	// deprecated
				// used while calculating bank numbers
	u16 window_size;
	u16 segment_a;
	u16 segment_b;
	u32 win_func_ptr;	// deprecated; used to switch banks from
				// protected mode without returning to real mode
	u16 pitch;		// number of bytes per horizontal line
	u16 width;		// width in pixels
	u16 height;		// height in pixels
	u8 w_char;		// unused...
	u8 y_char;		// ...
	u8 planes;
	u8 bpp;			// bits per pixel in this mode
	u8 banks;		// deprecated total number of banks in this mode
	u8 memory_model;
	u8 bank_size;		// deprecated; size of a bank, almost always
				// 64 KB but may be 16 KB...
	u8 image_pages;
	u8 reserved0;

	u8 red_mask;
	u8 red_position;
	u8 green_mask;
	u8 green_position;
	u8 blue_mask;
	u8 blue_position;
	u8 reserved_mask;
	u8 reserved_position;
	u8 direct_color_attributes;

	ptr_32 *framebuffer;	// physical address of the linear frame buffer
				// write here to draw to the screen
	u32 off_screen_mem_off;
	u16 off_screen_mem_size;// size of memory in the frame buffer but not
				// being displayed on the screen
	u8 reserved1[206];
};

#define MAX_NB_VESA_MODE	128

struct vesa_graphic_mode_list {
	u16		mode[MAX_NB_VESA_MODE];
	u32		nb_mode;
};

struct graphic_ctx {
	struct vesa_global_info		global_info;
	struct vesa_mode_info		mode;
	struct vesa_graphic_mode_list	mode_list;
} vesa_ctx;

#endif
