
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

# define KEYRELEASEMASK           0xFF
# define KEYPRESSMASK             0xFF

# define TURBOFISH_KEY_RELEASE    3
# define TURBOFISH_KEY_PRESS      2

// # define X11_DESTROY_NOTIFY    17
// # define X11_BUTTON_4          4
// # define X11_MOTION_NOTIFY     6

# define KEYB_1            18
# define KEYB_2            19
# define KEYB_3            20
# define KEYB_4            21
# define KEYB_5            23
# define KEYB_6            22
# define KEYB_7            26
# define KEYB_8            28
# define KEYB_9            26
# define KEYB_C            8
# define KEYB_P            35
# define KEYB_R            15
# define KEYB_PLUS         69
# define KEYB_MINUS        78
# define KEYB_MMO_W        13
# define KEYB_MMO_S        1
# define KEYB_MMO_A        0
# define KEYB_MMO_D        2

#  define KEYB_M           109
#  define KEYB_HELP        44
#  define KEYB_ESCAPE      27
#  define KEYB_ARROW_UP    82
#  define KEYB_ARROW_DOWN  84
#  define KEYB_ARROW_LEFT  81
#  define KEYB_ARROW_RIGHT 83

#endif
