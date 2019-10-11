#include <stddef.h>
#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <sys/ioctl.h>
#include <termios.h>
#include <fcntl.h>
#include <assert.h>
#include "libft.h"
/* #include <mlx.h> */
#include "wolf.h"
#include <math.h>


/*
 * #define NBR_HOOK_MAX 10
 *
 * struct window {
 * 	int 	size_x;
 * 	int 	size_y;
 * };
 */

/*
 * struct mlx_ctx {
 * 	loop_hook_t	loop_hook;
 * 	// the fd in which to read the keys
 * 	int		key_fd;
 * 	int		fb;
 * 	struct window win;
 * };
 */

/*
 * struct mlx_ctx MLX_CTX;
 * 
 * void	*mlx_init() {
 * 	MLX_CTX.key_release_hook = NULL;
 * 	MLX_CTX.key_pressed_hook = NULL;
 * 	MLX_CTX.loop_hook = NULL;
 * 	return &MLX_CTX;
 * }
 */

/*
 * int	mlx_loop (void *mlx_ptr) {
 * 	while (42) {
 * 		/\* printf("loop"); *\/
 * 		int scancode = 0;
 * 		int len_readen = read(MLX_CTX.key_fd, &scancode, 2);
 * 		if (len_readen > 0) {
 * 			printf("key: %d\n", scancode);
 * 			// Pressed
 * 			if (scancode >= 0x1 && scancode <= 0x58) {
 * 				if (MLX_CTX.key_pressed_hook != NULL) {
 * 					MLX_CTX.key_pressed_hook(scancode, NULL);
 * 				}
 * 			}
 * 			// Released
 * 			if (scancode >= 0x81 && scancode <= 0xd8) {
 * 				if (MLX_CTX.key_release_hook != NULL) {
 * 					MLX_CTX.key_release_hook(scancode - 0x80, NULL);
 * 				}
 * 			}
 * 			if (scancode >= 0xe010 && scancode <= 0xe06d) {
 * 				int escaped_scancode = handle_escape_scancode(scancode & 0xFF);
 * 				if (MLX_CTX.key_pressed_hook != NULL) {
 * 					MLX_CTX.key_pressed_hook(escaped_scancode, NULL);
 * 				}
 * 			}
 * 			if (scancode >= 0xe090 && scancode <= 0xe0ed) {
 * 				int escaped_scancode = handle_escape_scancode((scancode & 0xFF) - 0x80);
 * 				if (MLX_CTX.key_release_hook != NULL) {
 * 					MLX_CTX.key_release_hook(escaped_scancode, NULL);
 * 				}
 * 			}
 * 		}
 * 		else if (len_readen == -1) {
 * 			perror("read");
 * 		}
 * 		if (MLX_CTX.loop_hook != NULL) {
 * 			MLX_CTX.loop_hook(NULL);
 * 		}
 * 	}
 * 	// call the loop_hook
 * }
 */

/*
 * void	*mlx_new_window(void *mlx_ptr, int size_x, int size_y, char *title) {
 * 	(void)title;
 * 
 * 	int fb = open("/dev/fb", O_WRONLY);
 * 	if (fb == -1) {
 * 		perror("open");
 * 		exit(1);
 * 	}
 * 	MLX_CTX.fb = fb;
 * 	
 * 	struct window *win = malloc(sizeof(struct window));
 * 	win->size_x = size_x;
 * 	win->size_y = size_y;
 * 
 * 	return win;
 * }
 */

/*
 * void	*mlx_new_image(void *mlx_ptr,int width,int height) {
 * }
 * 
 * int	mlx_put_image_to_window(void *mlx_ptr, void *win_ptr, void *img_ptr,
 * 							int x, int y) {
 * 	assert(x == 0);
 * 	assert(y == 0);
 * 	struct window *win = (struct window *)win_ptr;
 * 	int written = write(MLX_CTX.fb, img_ptr, win->size_x * win->size_y * 3);
 * 	if (written == -1) {
 * 		perror("write");
 * 		exit(1);
 * 	}
 * 	ioctl(MLX_CTX.fb, REFRESH_SCREEN);
 * }
 */

/*
 * void	*mlx_xpm_file_to_image(void *mlx_ptr, char *filename,
 * 							   int *width, int *height) {
 * }
 * 
 * unsigned int	mlx_get_color_value(void *mlx_ptr, int color) {
 * }
 * 
 * char	*mlx_get_data_addr(void *img_ptr, int *bits_per_pixel,
 * 						   int *size_line, int *endian) {
 * }
 */


#ifndef GNU

#include "turbofish_mlx.h"

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>
#include <string.h>
#include <stdbool.h>
#include <stropts.h>

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
	ioctl(fd, RAW_SCANCODE_MODE, 1);
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

typedef int		(*hook_t)(int keycode, void *param);
typedef int		(*loop_hook_t)(void *);


const char DEFAULT_WINDOW_NAME[] = "Turbofish window";
const int WINDOW_WIDTH = 1024;
const int WINDOW_HEIGHT = 768;
const int BPP = 32;

struct window {
	int width;
	int height;
	char *title;
	struct local_buffer local_buffer;
	hook_t	key_release_hook;
	hook_t	key_pressed_hook;
};

struct image {
	int width;
	int height;

	u8 *pix_map;
};

struct mlx {
	struct window *window;
	struct image *image;
	int (*callback)();
	int		key_fd;
	void *env;
};

/*
**  needed before everything else.
**  return (void *)0 if failed
*/
void *mlx_init(void)
{
	struct mlx *mlx = (struct mlx *)calloc(1, sizeof(struct mlx));
	if (mlx == NULL) {
		dprintf(STDERR_FILENO, "Cannot allocate memory for main mlx structure\n");
	}
	if (mlx != NULL) {
		int fd = open("/proc/self/fd/0", O_RDWR | O_NONBLOCK);
		if (fd == -1) {
			perror("open failed");
			exit(1);
		}
		mlx->key_fd = fd;

		set_raw_mode(fd);
	}

	return (void *)mlx;
}

/*
** Basic actions
*/
void *mlx_new_window(void *mlx_ptr, int size_x, int size_y, char *title)
{
	struct mlx *mlx = (struct mlx *)mlx_ptr;

	if (mlx == NULL) {
		dprintf(STDERR_FILENO, "Sending NUll ptr is not a good idea\n");
		return NULL;
	}
	if (size_x != WINDOW_WIDTH || size_y != WINDOW_HEIGHT) {
		dprintf(STDERR_FILENO, "Only supported window of 1024*768px\n");
		return NULL;
	}
	struct window *window = (struct window *)calloc(1, sizeof(struct window));
	if (window == NULL) {
		dprintf(STDERR_FILENO, "Cannot allocate enough memory for window\n");
		return NULL;
	}
	window->width = size_x;
	window->height = size_y;
	if (title != NULL) {
		window->title = strdup(title);
	} else {
		window->title = strdup(DEFAULT_WINDOW_NAME);
	}
	if (window->title == NULL) {
		dprintf(STDERR_FILENO, "Cannot allocate memory for title\n");
		free(window);
		return NULL;
	}
	ioctl(0, GET_FRAME_BUFFER_PTR, (void *)&window->local_buffer);
	printf("local_buffer: %p of len %zu\n", window->local_buffer.buf, window->local_buffer.len);
	mlx->window = window;
	return (void *)window;
}

int mlx_destroy_window(void *mlx_ptr, void *win_ptr)
{
	struct window *window = (struct window *)win_ptr;
	struct mlx *mlx = (struct mlx *)mlx_ptr;

	if (window == NULL || mlx == NULL) {
		dprintf(STDERR_FILENO, "Sending NUll(s) ptr(s) is not a good idea\n");
		return -1;
	}
	free(window->title);
	free(window);
	mlx->window = NULL;
	return 0;
}

void *mlx_new_image(void *mlx_ptr, int width, int height)
{
	struct mlx *mlx = (struct mlx *)mlx_ptr;

	if (mlx == NULL) {
		dprintf(STDERR_FILENO, "Sending NUll ptr is not a good idea\n");
		return NULL;
	}

	struct image *image = (struct image *)calloc(1, sizeof(struct image));
	if (image == NULL) {
		dprintf(STDERR_FILENO, "Cannot allocate memory for basic image\n");
		return NULL;
	}
	image->pix_map = (u8 *)calloc(1, width * height * BPP / 8);
	if (image->pix_map == NULL) {
		dprintf(STDERR_FILENO, "Cannot allocate memory for image pixel map\n");
		free(image);
		return NULL;
	}
	mlx->image = image;
	return (void *)image;
}

int mlx_destroy_image(void *mlx_ptr, void *img_ptr)
{
	struct mlx *mlx = (struct mlx *)mlx_ptr;
	struct image *image = (struct image *)img_ptr;

	if (mlx == NULL || image == NULL) {
		dprintf(STDERR_FILENO, "Sending NUll(s) ptr(s) is not a good idea\n");
		return -1;
	}
	free(image->pix_map);
	free(image);
	mlx->image = NULL;
	return 0;
}

/*
**  return void NULL if failed
*/
char *mlx_get_data_addr(void *img_ptr,
			int *bits_per_pixel,
			int *size_line,
			int *endian)
{
	struct image *image = (struct image *)img_ptr;

	if (image == NULL || bits_per_pixel == NULL || size_line == NULL || endian == NULL) {
		dprintf(STDERR_FILENO, "Sending NUll(s) ptr(s) is not a good idea\n");
		return NULL;
	}
	*bits_per_pixel = BPP;
	*size_line = *bits_per_pixel / 8 * image->width;
	*endian = 0;
	return (char *)image->pix_map;
}

int mlx_loop_hook(void *mlx_ptr, int (*funct_ptr)(), void *param)
{
	struct mlx *mlx = (struct mlx *)mlx_ptr;

	if (mlx == NULL || funct_ptr == NULL || param == NULL) {
		dprintf(STDERR_FILENO, "Sending NUll ptr is not a good idea\n");
		return -1;
	}
	mlx->callback = funct_ptr;
	mlx->env = param;
	return 0;
}

int mlx_loop(void *mlx_ptr)
{
	struct mlx *mlx = (struct mlx *)mlx_ptr;

	if (mlx == NULL || mlx->callback == NULL || mlx->env == NULL) {
		dprintf(STDERR_FILENO, "Sending NUll ptr is not a good idea\n");
		return -1;
	}
	while (true) {
		/* printf("loop"); */
		int scancode = 0;
		int len_readen = read(mlx->key_fd, &scancode, 2);
		if (len_readen > 0) {
			printf("key: %d\n", scancode);
			// Pressed
			if (scancode >= 0x1 && scancode <= 0x58) {
				if (mlx->window->key_pressed_hook != NULL) {
					mlx->window->key_pressed_hook(scancode, NULL);
				}
			}
			// Released
			if (scancode >= 0x81 && scancode <= 0xd8) {
				if (mlx->window->key_release_hook != NULL) {
					mlx->window->key_release_hook(scancode - 0x80, NULL);
				}
			}
			if (scancode >= 0xe010 && scancode <= 0xe06d) {
				int escaped_scancode = handle_escape_scancode(scancode & 0xFF);
				if (mlx->window->key_pressed_hook != NULL) {
					mlx->window->key_pressed_hook(escaped_scancode, NULL);
				}
			}
			if (scancode >= 0xe090 && scancode <= 0xe0ed) {
				int escaped_scancode = handle_escape_scancode((scancode & 0xFF) - 0x80);
				if (mlx->window->key_release_hook != NULL) {
					mlx->window->key_release_hook(escaped_scancode, NULL);
				}
			}
		}
		else if (len_readen == -1) {
			perror("read");
		}
		int _ret = mlx->callback(mlx->env);
		(void)_ret;
	}
	// _unreachable!()
	return 0;
}

void mlx_put_image_to_window(mlx_ptr_t *mlx_ptr,
			     mlx_win_list_t *win_ptr,
			     mlx_img_list_t *img_ptr,
			     int x,
			     int y)
{
	struct mlx *mlx = (struct mlx *)mlx_ptr;
	struct window *window = (struct window *)win_ptr;
	struct image *image = (struct image *)img_ptr;

	if (mlx == NULL || window == NULL || image == NULL) {
		dprintf(STDERR_FILENO, "Sending NUll(s) ptr(s) is not a good idea\n");
		exit(1);
	}
	int j = 0;
	for (int i = 0; i < WINDOW_WIDTH * WINDOW_HEIGHT; i++) {
		u32 content = ((u32 *)(image->pix_map))[i];
		window->local_buffer.buf[j++] = content & 0xff;
		window->local_buffer.buf[j++] = (content & 0xff00) >> 8;
		window->local_buffer.buf[j++] = (content & 0xff0000) >> 16;
	}
	ioctl(0, REFRESH_SCREEN, &window->local_buffer);
	(void)x;
	(void)y;
}

int mlx_string_put(void *mlx_ptr,
		   void *win_ptr,
		   int x,
		   int y,
		   int color,
		   char *string)
{
	printf("%s\n", string);
	(void)mlx_ptr;
	(void)win_ptr;
	(void)x;
	(void)y;
	(void)color;
	(void)string;
	return 0;
}

int	mlx_hook(t_win_list *win_ptr, int x_event, int x_mask,
			int (*funct)(), void *param) {

	if (win_ptr != NULL) {
		struct window *win = win_ptr;
		if (x_event == KEYPRESS) {
			win->key_pressed_hook = funct;
		} else if (x_event == KEYRELEA) {
			win->key_release_hook = funct;
		}
	}
	return 0;
}


/* 
 * void	*mlx_xpm_file_to_image(void *mlx_ptr, char *filename,
 * 							int *width, int *height) {
 * }
 */

#endif
