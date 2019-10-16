
#ifndef __TURBOFISH_MLX_H__
# define __TURBOFISH_MLX_H__

typedef void mlx_ptr_t;
typedef void mlx_win_list_t;
typedef void mlx_img_list_t;
typedef void t_win_list;

#define CHAR_WIDTH 8
#define CHAR_HEIGHT 16

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

// There is no window for the moment
// # define X11_DESTROY_NOTIFY    17
// # define X11_BUTTON_4          4
// # define X11_MOTION_NOTIFY     6

# define KEYB_1            2
# define KEYB_2            3
# define KEYB_3            4
# define KEYB_4            5
# define KEYB_5            6
# define KEYB_6            7
# define KEYB_7            8
# define KEYB_8            9
# define KEYB_9            10
# define KEYB_C            46
# define KEYB_P            25
# define KEYB_R            19
# define KEYB_PLUS         78
# define KEYB_MINUS        74
# define KEYB_MMO_W        17
# define KEYB_MMO_S        31
# define KEYB_MMO_A        30
# define KEYB_MMO_D        32

#  define KEYB_M           50
#  define KEYB_HELP        35
#  define KEYB_ESCAPE      1

// This define seams to be just internaly
#  define KEYB_ARROW_UP    126
#  define KEYB_ARROW_DOWN  125
#  define KEYB_ARROW_LEFT  123
#  define KEYB_ARROW_RIGHT 124

#endif
