/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   bmp_load.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: stoupin <stoupin@student.42.fr>            +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/05/18 22:13:29 by bmickael          #+#    #+#             */
/*   Updated: 2018/02/01 16:50:35 by stoupin          ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include "internal_bmp.h"

#include "libft.h"

static void	paste_fileheader(struct bitmap *s)
{
	printk("%s %c%c\n", "signature",
			s->fileheader.signature[0],
			s->fileheader.signature[1]);
	printk("%s: %i\n", "filesize", s->fileheader.filesize);
	printk("%s: %i\n", "offset", s->fileheader.fileoffset_to_pixelarray);
	printk("%s: %i\n", "header_size", s->bitmapinfoheader.dibheadersize);
	printk("%s: %i\n", "width", s->bitmapinfoheader.width);
	printk("%s: %i\n", "height", s->bitmapinfoheader.height);
	printk("%s: %i\n", "planes", s->bitmapinfoheader.planes);
	printk("%s: %i\n", "bpp", s->bitmapinfoheader.bitsperpixel);
	printk("%s: %i\n", "compression", s->bitmapinfoheader.compression);
	printk("%s: %i\n", "imagesize", s->bitmapinfoheader.imagesize);
	printk("%s: %i\n", "xpermeter", s->bitmapinfoheader.ypixelpermeter);
	printk("%s: %i\n", "ypermeter", s->bitmapinfoheader.xpixelpermeter);
	printk("%s: %i\n", "numcolorpal",
			s->bitmapinfoheader.numcolorspallette);
}

static void	fill_image(
		u8 *data,
		u8 *pixelbuffer,
		int width,
		int height)
{
	size_t	i;
	int	p;
	int	c;
	u8	*ptr;

	p = height - 1;
	ptr = pixelbuffer + (p * width * 3);
	c = 0;
	i = 0;
	while (p >= 0) {
		if ((i % 4) == 3)
			i++;
		data[i] = ptr[c++];
		if (c == (width * 3)) {
			p--;
			ptr = pixelbuffer + (p * width * 3);
			c = 0;
		}
		i++;
	}
}

int		bmp_load(u8 *img, int *width, int *height, int **data)
{
	paste_fileheader((struct bitmap *)img);

	/*
	*width = s->bitmapinfoheader.width;
	*height = s->bitmapinfoheader.height;
	if (!(*data = (int *)ft_memalloc(sizeof(int) * (*width) * (*height))))
		return -1;
	fill_image((u8 *)*data, (u8 *)
			((char*)s + s->fileheader.fileoffset_to_pixelarray),
				*width, *height);
	*/

	(void)width;
	(void)height;
	(void)data;
	return 0;
}
