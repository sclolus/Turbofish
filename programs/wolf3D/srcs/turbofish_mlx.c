
#include "turbofish_mlx.h"

#include <stdlib.h>

#ifndef GNU

/*
**  needed before everything else.
**  return (void *)0 if failed
*/
void *mlx_init() {
	return NULL;
}

/*
** Basic actions
*/
void *mlx_new_window(void *mlx_ptr, int size_x, int size_y, char *title) {
	(void)mlx_ptr;
	(void)size_x;
	(void)size_y;
	(void)title;
	return NULL;
}

int mlx_destroy_image(void *mlx_ptr, void *img_ptr) {
	(void)mlx_ptr;
	(void)img_ptr;
	return 0;
}

int mlx_destroy_window(void *mlx_ptr, void *win_ptr) {
	(void)mlx_ptr;
	(void)win_ptr;
	return 0;
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

void *mlx_new_image(void *mlx_ptr, int width, int height) {
	(void)mlx_ptr;
	(void)width;
	(void)height;
	return NULL;
}

/*
**  return void NULL if failed
*/
char *mlx_get_data_addr(void *img_ptr,
			int *bits_per_pixel,
			int *size_line,
			int *endian) {
	(void)img_ptr;
	(void)bits_per_pixel;
	(void)size_line;
	(void)endian;
	return NULL;
}

#endif
