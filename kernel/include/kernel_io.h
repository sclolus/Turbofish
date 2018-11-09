
#ifndef __KERNEL_IO_H__
# define __KERNEL_IO_H__

# include "i386_type.h"

#define MODIFIER_QUANTITY	13

struct modifier_list {
	char *string;
	u32 color;
};

enum term_mode {
	kernel = 0,
	panic_screen,
	user
};

struct k_line {
	u8 *str;
	size_t nb_char;
};

struct k_tty {
	struct k_line *line;
	size_t nb_line;
	u8 *background_img;
	u32 default_color;
};

struct kernel_io_ctx {
	enum term_mode term_mode;
	struct k_tty *tty;
	struct k_tty *current_tty;
	size_t nb_tty;
} kernel_io_ctx;

s32		write(s32 fd, const void *buf, u32 size);
s32		write_direct(s32 fd, const u8 *buf, u32 size);
int		set_cursor_location(u32 x, u32 y);

void		init_kernel_io(void);
struct k_tty	*create_tty(u8 *background_img, u32 default_color);
int		remove_tty(u32 index);
int		select_tty(u32 index);
void		fill_tty_background(struct k_tty *tty);
void		copy_tty_content(struct k_tty *tty);
void		*add_tty_char(u8 c);
void		*new_tty_line();

#endif
