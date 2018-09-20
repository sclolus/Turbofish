
#ifndef INTERNAL_BMP_H
# define INTERNAL_BMP_H

#include "i386_type.h"

# pragma pack(push, 1)

struct fileheader
{
	u8	signature[2];
	u32	filesize;
	u32	reserved;
	u32	fileoffset_to_pixelarray;
};

struct bitmapinfoheader
{
	u32	dibheadersize;
	u32	width;
	u32	height;
	u16	planes;
	u16	bitsperpixel;
	u32	compression;
	u32	imagesize;
	u32	ypixelpermeter;
	u32	xpixelpermeter;
	u32	numcolorspallette;
	u32	mostimpcolor;
};

struct bitmap
{
	struct fileheader	fileheader;
	struct bitmapinfoheader	bitmapinfoheader;
};

# pragma pack(pop)

#endif
