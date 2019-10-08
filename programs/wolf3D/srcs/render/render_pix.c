/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   render_pix.c                                       :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <marvin@42.fr>                     +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/06 11:20:50 by stoupin           #+#    #+#             */
/*   Updated: 2017/07/06 11:20:52 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "render.h"

static inline t_pix	interp_pix(t_pix b, t_pix f, float ratio)
{
	t_pix	new_pix;

	new_pix.c.a = 0;
	new_pix.c.b = (unsigned char)(ratio * (f.c.b - b.c.b) + b.c.b);
	new_pix.c.g = (unsigned char)(ratio * (f.c.g - b.c.g) + b.c.g);
	new_pix.c.r = (unsigned char)(ratio * (f.c.r - b.c.r) + b.c.r);
	return (new_pix);
}

static inline t_pix	get_pix_complex(t_bmp *src, t_coord_f c_src)
{
	t_pix		corners[4];
	t_coord_i	c_src_i;
	t_coord_f	delta;
	int			i;

	c_src_i = (t_coord_i){(int)c_src.x, (int)c_src.y};
	i = src->dim.x * c_src_i.y + c_src_i.x;
	corners[0] = src->pix[i];
	corners[1] = src->pix[i + 1];
	i += src->dim.x;
	corners[2] = src->pix[i];
	corners[3] = src->pix[i + 1];
	delta = (t_coord_f){c_src.x - c_src_i.x, c_src.y - c_src_i.y};
	return (interp_pix(
			interp_pix(corners[0], corners[1], delta.x),
			interp_pix(corners[2], corners[3], delta.x),
			delta.y));
}

static inline t_pix	get_pix_simple(t_bmp *src, t_coord_f c_src)
{
	t_coord_i	c_src_i;
	int			i;

	c_src_i = (t_coord_i){(int)c_src.x, (int)c_src.y};
	i = src->dim.x * c_src_i.y + c_src_i.x;
	return (src->pix[i]);
}

t_pix				get_pix(t_bmp *src, t_coord_f c_src, float dist,
													int bilinear_interpolation)
{
	t_pix	(*get_pix)(t_bmp *, t_coord_f);
	t_pix	pix;

	get_pix = bilinear_interpolation && dist < 3 ?
				&get_pix_complex : &get_pix_simple;
	pix = get_pix(src, c_src);
	if (pix.i == 0xff00ff)
		pix.c.a = 0xff;
	else if (dist > SHADOW_LIMIT)
	{
		dist = SHADOW_LIMIT / dist;
		pix.c.b *= dist;
		pix.c.g *= dist;
		pix.c.r *= dist;
	}
	return (pix);
}
