/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   bmp_save.c                                         :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/05/18 22:13:29 by bmickael          #+#    #+#             */
/*   Updated: 2017/06/03 15:57:53 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#include <stdlib.h>
#include <stdio.h>
#include <time.h>
#include "libft.h"
#include "bmp/internal_bmp.h"

static void		paste_fileheader(t_bitmap *pbitmap, int width, int height)
{
	ft_memcpy((char *)pbitmap->fileheader.signature, "BM", 2);
	pbitmap->fileheader.filesize = (width * height) + sizeof(t_bitmap);
	pbitmap->fileheader.fileoffset_to_pixelarray = sizeof(t_bitmap);
	pbitmap->bitmapinfoheader.dibheadersize = sizeof(t_bitmapinfoheader);
	pbitmap->bitmapinfoheader.width = width;
	pbitmap->bitmapinfoheader.height = height;
	pbitmap->bitmapinfoheader.planes = PLANES;
	pbitmap->bitmapinfoheader.bitsperpixel = BPP;
	pbitmap->bitmapinfoheader.compression = COMPRESSION;
	pbitmap->bitmapinfoheader.imagesize = (width * height * BPP / 8);
	pbitmap->bitmapinfoheader.ypixelpermeter = YPIXELPERMETER;
	pbitmap->bitmapinfoheader.xpixelpermeter = XPIXELPERMETER;
	pbitmap->bitmapinfoheader.numcolorspallette = 0;
}

static void		fill_pixelbuffer(uint8_t *pixelbuffer, char *data, int width,
																	int height)
{
	size_t	i;
	uint8_t *ptr;
	int		c;
	int		p;

	p = height - 1;
	ptr = pixelbuffer + (p * width * 3);
	c = 0;
	i = 0;
	while (p >= 0)
	{
		if ((i % 4) == 3)
			i++;
		ptr[c++] = data[i];
		if (c == (width * 3))
		{
			p--;
			ptr = pixelbuffer + (p * width * 3);
			c = 0;
		}
		i++;
	}
}

int				bmp_save(char *filename, int width, int height, int *data)
{
	char		buff[512];
	t_bitmap	*pbitmap;
	uint8_t		*pixelbuffer;
	FILE		*file;

	ft_bzero(buff, 512);
	ft_sprintf(buff, "screenshoots/%lu%s", (unsigned long)time(NULL), ".bmp");
	if (!(file = fopen(buff, "wb+")))
		return (1);
	if (!(pbitmap = (t_bitmap *)ft_memalloc(sizeof(t_bitmap))))
		return (1);
	if (!(pixelbuffer = (uint8_t *)ft_memalloc(height * width * BPP / 8)))
		return (1);
	paste_fileheader(pbitmap, width, height);
	fwrite(pbitmap, 1, sizeof(t_bitmap), file);
	fill_pixelbuffer(pixelbuffer, (char *)data, width, height);
	fwrite(pixelbuffer, 1, pbitmap->bitmapinfoheader.imagesize, file);
	fclose(file);
	free(pbitmap);
	free(pixelbuffer);
	(void)filename;
	return (0);
}
