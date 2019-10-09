
#include "turbofish_mlx.h"

#ifndef GNU

#include <stdlib.h>
#include <stdint.h>
#include <stdio.h>
#include <unistd.h>
#include <string.h>

const char DEFAULT_WINDOW_NAME[] = "Turbofish window";
const int WINDOW_WIDTH = 1024;
const int WINDOW_HEIGHT = 768;

struct window {
	int width;
	int height;
	char *title;

	u8 *pix_map;
};

struct image {
	int width;
	int height;

	u8 *pix_map;
};

struct mlx {
	struct window *window;
	struct image *image;
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
	return (void *)mlx;
}

/*
** Basic actions
*/
// Is MLX can handle just only one window ? I don't think
// I don't think that window can got a pix_map
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
	window->pix_map = (u8 *)calloc(1, window->width * window->height * 32 / 8);
	if (window->pix_map == NULL) {
		dprintf(STDERR_FILENO, "Cannot allocate memory for window pixel map\n");
		free(window->title);
		free(window);
		return NULL;
	}
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
	free(window->pix_map);
	free(window);
	mlx->window = NULL;
	return 0;
}

// Is MLX can handle just only one image ? I don't think
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
	image->pix_map = (u8 *)calloc(1, width * height * 32 / 8);
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
	*bits_per_pixel = 32;
	*size_line = *bits_per_pixel * image->width;
	*endian = 0;
	return (char *)image->pix_map;
}

int mlx_string_put(void *mlx_ptr,
		   void *win_ptr,
		   int x,
		   int y,
		   int color,
		   char *string) {
	(void)mlx_ptr;
	(void)win_ptr;
	(void)x;
	(void)y;
	(void)color;
	(void)string;
	return 0;
}

void mlx_put_image_to_window(mlx_ptr_t *mlx_ptr,
			     mlx_win_list_t *win_ptr,
			     mlx_img_list_t *img_ptr,
			     int x,
			     int y) {
	(void)mlx_ptr;
	(void)win_ptr;
	(void)img_ptr;
	(void)x;
	(void)y;
}

int mlx_hook(t_win_list *win,
	     int x_event,
	     int x_mask,
	     int (*funct)(),
	     void *param) {
	(void)win;
	(void)x_event;
	(void)x_mask;
	(void)funct;
	(void)param;
	return 0;
}

int mlx_loop_hook(void *mlx_ptr, int (*funct_ptr)(), void *param) {
	(void)mlx_ptr;
	(void)funct_ptr;
	(void)param;
	return 0;
}

int mlx_loop(void *mlx_ptr) {
	(void)mlx_ptr;
	return 0;
}

#endif
