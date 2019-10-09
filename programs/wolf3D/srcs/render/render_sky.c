/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   render_sky.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/06 11:10:22 by stoupin           #+#    #+#             */
/*   Updated: 2017/07/06 11:10:23 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include "render.h"

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
													get_pix(src, c_src, 0, 1);
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
