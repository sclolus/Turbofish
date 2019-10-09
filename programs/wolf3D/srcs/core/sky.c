/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   skybox2.c                                          :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/06/06 11:51:00 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/06 11:51:02 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "core/wolf3d.h"
#include "bmp/bmp.h"

static inline t_pix	interp_pix(t_pix b, t_pix f, float ratio)
{
	t_pix	new_pix;

	new_pix.c.a = 0;
	new_pix.c.b = (unsigned char)(ratio * (f.c.b - b.c.b) + b.c.b);
	new_pix.c.g = (unsigned char)(ratio * (f.c.g - b.c.g) + b.c.g);
	new_pix.c.r = (unsigned char)(ratio * (f.c.r - b.c.r) + b.c.r);
	return (new_pix);
}

static inline t_pix	get_pix(t_bmp *src, t_coord_f c_src)
{
	t_pix		corners[4];
	t_coord_i	c_src_i;
	t_coord_f	delta;
	int			i;

	c_src_i = (t_coord_i){(int)c_src.x, (int)c_src.y};
	delta = (t_coord_f){c_src.x - c_src_i.x, c_src.y - c_src_i.y};
	i = src->dim.x * c_src_i.y + c_src_i.x;
	corners[0] = src->pix[i];
	corners[1] = src->pix[i + 1];
	i += src->dim.x;
	corners[2] = src->pix[i];
	corners[3] = src->pix[i + 1];
	return (interp_pix(
			interp_pix(corners[0], corners[1], delta.x),
			interp_pix(corners[2], corners[3], delta.x),
			delta.y));
}

static void			copy_img(t_bmp *dst, t_bmp *src, int new_dim_x)
{
	t_coord_i c_dst;
	t_coord_f c_src;

	c_dst.y = -1;
	while (++c_dst.y < dst->dim.y)
	{
		c_dst.x = -1;
		while (++c_dst.x < dst->dim.x)
		{
			c_src = (t_coord_f){c_dst.x * (float)src->dim.x / dst->dim.x,
								c_dst.y * (float)src->dim.y / dst->dim.y};
			if ((int)c_src.x >= ((src->dim.x - 1))
				|| ((int)c_src.y >= (src->dim.y - 1)))
				dst->pix[new_dim_x * c_dst.y + c_dst.x] =
				src->pix[(int)(src->dim.x * (int)c_src.y + (int)c_src.x)];
			else
				dst->pix[new_dim_x * c_dst.y + c_dst.x] =
														get_pix(src, c_src);
		}
	}
}

static void			paste_bout(t_pix *data)
{
	int y;
	int x;

	y = 0;
	while (y < HEIGHT)
	{
		x = RATIO * WIDTH;
		while (x < ((RATIO + 1) * WIDTH))
		{
			data[y * (WIDTH * (RATIO + 1)) + x] =
					data[y * (WIDTH * (RATIO + 1)) + (x - (RATIO * WIDTH))];
			x++;
		}
		y++;
	}
}

void				init_sky(t_env *e, char *file_name)
{
	t_bmp	*sky_bmp;

	if (!(e->sky = malloc(sizeof(t_sky))))
		exit(EXIT_FAILURE);
	if (!(e->sky->data = malloc(sizeof(t_bmp))))
		exit(EXIT_FAILURE);
	sky_bmp = load_bitmap((char*[]){file_name}, 1);
	sky_bmp = &sky_bmp[0];
	e->sky->data->dim.x = WIDTH * RATIO;
	e->sky->data->dim.y = HEIGHT;
	if (!(e->sky->data->pix = (t_pix *)calloc(1, WIDTH * (RATIO + 1) * HEIGHT * sizeof(t_pix))))
		exit(EXIT_FAILURE);
	copy_img(e->sky->data, sky_bmp, WIDTH * (RATIO + 1));
	paste_bout(e->sky->data->pix);
	free(sky_bmp->pix);
	free(sky_bmp);
}
