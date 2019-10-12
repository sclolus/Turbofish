/* ************************************************************************** */
/*                                                                            */
/*                                                        :::      ::::::::   */
/*   internal_bmp.h                                     :+:      :+:    :+:   */
/*                                                    +:+ +:+         +:+     */
/*   By: bmickael <marvin@42.fr>                    +#+  +:+       +#+        */
/*                                                +#+#+#+#+#+   +#+           */
/*   Created: 2017/07/05 08:59:11 by bmickael          #+#    #+#             */
/*   Updated: 2017/07/05 08:59:13 by bmickael         ###   ########.fr       */
/*                                                                            */
/* ************************************************************************** */

#ifndef INTERNAL_BMP_H
# define INTERNAL_BMP_H

# include <stdint.h>

# define BPP 				24
# define PLANES				1
# define COMPRESSION		0
# define XPIXELPERMETER		0x130B
# define YPIXELPERMETER		0x130B

# pragma pack(push, 1)

typedef struct				s_fileheader
{
	uint8_t					signature[2];
	uint32_t				filesize;
	uint32_t				reserved;
	uint32_t				fileoffset_to_pixelarray;
}							t_fileheader;

typedef struct				s_bitmapinfoheader
{
	uint32_t				dibheadersize;
	uint32_t				width;
	uint32_t				height;
	uint16_t				planes;
	uint16_t				bitsperpixel;
	uint32_t				compression;
	uint32_t				imagesize;
	uint32_t				ypixelpermeter;
	uint32_t				xpixelpermeter;
	uint32_t				numcolorspallette;
	uint32_t				mostimpcolor;
}							t_bitmapinfoheader;

typedef struct				s_bitmap
{
	t_fileheader			fileheader;
	t_bitmapinfoheader		bitmapinfoheader;
}							t_bitmap;

# pragma pack(pop)

#endif
