#include <stddef.h>
#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <termios.h>
#include <fcntl.h>
#include "libft.h"
#include <mlx.h>
#include "wolf.h"
#include <math.h>


void set_raw_mode(int fd) {
	struct termios termios_p;
	int ret = tcgetattr(fd, &termios_p);
	if(ret == -1) {
		perror("tcgetattr failed");
	}

	termios_p.c_lflag &= (~(ICANON | ECHO | TOSTOP));
	ret = tcsetattr(fd, TCSANOW, &termios_p);
	if( ret == -1) {
		perror("tcsetattr failed");
	}
	ioctl(fd, RAW_SCANCODE_MODE);
}

void set_cooked_mode(void) {
	struct termios termios_p;
	int ret =  tcgetattr(0, &termios_p);
	if(ret == -1) {
		perror("tcgetattr failed");
	}

	termios_p.c_lflag |= (ICANON | ECHO | TOSTOP);
	ret = tcsetattr(0, TCSANOW, &termios_p);
	if( ret == -1) {
		perror("tcsetattr failed");
	}
}

#define NBR_HOOK_MAX 10

typedef int		(*hook_t)(int keycode, void *param);

struct mlx_ctx {
	hook_t	key_release_hook;
	hook_t	key_pressed_hook;
	// the fd in which to read the keys
	int		key_fd;
};

struct mlx_ctx MLX_CTX;

unsigned int	mlx_get_color_value(void *mlx_ptr, int color) {
}

char	*mlx_get_data_addr(void *img_ptr, int *bits_per_pixel,
						   int *size_line, int *endian) {
}

int	mlx_hook(void *win_ptr, int x_event, int x_mask,
			 int (*funct)(), void *param) {
	if (x_event == KEYPRESS) {
		MLX_CTX.key_pressed_hook = funct;
	} else if (x_event == KEYRELEA) {
		MLX_CTX.key_release_hook = funct;
	}
}

void	*mlx_init() {
	MLX_CTX.key_release_hook = NULL;
	MLX_CTX.key_pressed_hook = NULL;
	int fd = open("/proc/self/fd/0", O_RDWR | O_NONBLOCK);
	if (fd == -1) {
		perror("open failed");
		exit(1);
	}
	MLX_CTX.key_fd = fd;

	set_raw_mode(fd);

	return &MLX_CTX;
}

int handle_escape_scancode(int scancode) {
	switch (scancode) {
		case 0x38:
			return 100;
		case 0x48:
			return KEY_UP;
		case 0x4b:
			return KEY_LEFT;
		case 0x4d:
			return KEY_RIGHT;
		case 0x50:
			return KEY_DOWN;
		default:
			return -1;
	}
}

int	mlx_loop (void *mlx_ptr) {
	while (42) {
		/* printf("loop"); */
		int scancode = 0;
		int len_readen = read(MLX_CTX.key_fd, &scancode, 2);
		if (len_readen > 0) {
			printf("key: %d\n", scancode);
			// Pressed
			if (scancode >= 0x1 && scancode <= 0x58) {
				if (MLX_CTX.key_pressed_hook != NULL) {
					MLX_CTX.key_pressed_hook(scancode, NULL);
				}
			}
			// Released
			if (scancode >= 0x81 && scancode <= 0xd8) {
				if (MLX_CTX.key_release_hook != NULL) {
					MLX_CTX.key_release_hook(scancode - 0x80, NULL);
				}
			}
			if (scancode >= 0xe010 && scancode <= 0xe06d) {
				int escaped_scancode = handle_escape_scancode(scancode & 0xFF);
				if (MLX_CTX.key_pressed_hook != NULL) {
					MLX_CTX.key_pressed_hook(escaped_scancode, NULL);
				}
			}
			if (scancode >= 0xe090 && scancode <= 0xe0ed) {
				int escaped_scancode = handle_escape_scancode((scancode & 0xFF) - 0x80);
				if (MLX_CTX.key_release_hook != NULL) {
					MLX_CTX.key_release_hook(escaped_scancode, NULL);
				}
			}
		}
		else if (len_readen == -1) {
			perror("read");
		}
	}
	// call the loop_hook
}

int	mlx_loop_hook (void *mlx_ptr, int (*funct_ptr)(), void *param) {
}

void	*mlx_new_window(void *mlx_ptr, int size_x, int size_y, char *title) {
}

void	*mlx_new_image(void *mlx_ptr,int width,int height) {
}

int	mlx_put_image_to_window(void *mlx_ptr, void *win_ptr, void *img_ptr,
							int x, int y) {
}

double sqrt(double x) {
}

void	*mlx_xpm_file_to_image(void *mlx_ptr, char *filename,
							   int *width, int *height) {
}
