
#ifndef __TURBOFISH_MLX_H__
# define __TURBOFISH_MLX_H__

typedef void mlx_ptr_t;
typedef void mlx_win_list_t;
typedef void mlx_img_list_t;
typedef void t_win_list;

void *mlx_init();

void *mlx_new_window(void *mlx_ptr, int size_x, int size_y, char *title);

int mlx_destroy_window(void *mlx_ptr, void *win_ptr);

void *mlx_new_image(void *mlx_ptr, int width, int height);

int mlx_destroy_image(void *mlx_ptr, void *img_ptr);

char *mlx_get_data_addr(void *img_ptr,
			int *bits_per_pixel,
			int *size_line,
			int *endian);

int mlx_loop_hook(void *mlx_ptr, int (*funct_ptr)(), void *param);

int mlx_loop(void *mlx_ptr);


void mlx_put_image_to_window(mlx_ptr_t *mlx_ptr,
			     mlx_win_list_t *win_ptr,
			     mlx_img_list_t *img_ptr,
			     int x,
			     int y);

int mlx_string_put(void *mlx_ptr,
		   void *win_ptr,
		   int x,
		   int y,
		   int color,
		   char *string);

int mlx_hook(t_win_list *win,
	     int x_event,
	     int x_mask,
	     int (*funct)(),void *param);

#endif
