#include <stddef.h>
#include <stdlib.h>
#include <errno.h>
#include <unistd.h>
#include <fcntl.h>
#include "libft.h"
#include <mlx.h>
#include "wolf.h"
#include <math.h>

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
	return &MLX_CTX;
}

int	mlx_loop (void *mlx_ptr) {
	while (42) {
		int buf;
		int len_readen = read(MLX_CTX.key_fd, &buf, 2);
		if (len_readen == 2) {
			printf("key readen: %d", buf);
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
