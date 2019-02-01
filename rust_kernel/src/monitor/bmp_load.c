
#include "bmp.h"

/*
 * TODO this Function has no provide scale OR allocating
 */

u8 *GRAPHIC_BUFFER_LOCATION = (u8 *)0x2000000;

/*
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
*/

static void	fill_image_24(
			u8 *data,
			const u8 *pixelbuffer,
			u32 width,
			u32 height)
{
	size_t			i;
	s32			p;
	u32			c;
	const u8		*ptr;

	p = height - 1;
	ptr = pixelbuffer + (p * width * 3);
	c = 0;
	i = 0;
	while (p >= 0) {
		data[i] = ptr[c++];
		if (c == (width * 3)) {
			p--;
			ptr = pixelbuffer + (p * width * 3);
			c = 0;
		}
		i++;
	}
}

static void	fill_image_32(
			u8 *data,
			const u8 *pixelbuffer,
			u32 width,
			u32 height)
{
	size_t			i;
	s32			p;
	u32			c;
	const u8		*ptr;

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

u8		*_get_image_buffer(
			const u8 *file,
			u32 width,
			u32 height, u32 bpp)
{
	const struct bitmap	*bmp;
	const u8		*start;
	u8			*img_buf;

	bmp = (const struct bitmap *)file;

/*
	paste_fileheader((struct bitmap *)img);
*/

/*
	u32 image_width = img->bitmapinfoheader.width;
	u32 image_height = img->bitmapinfoheader.height;
	img_buf = (u8 *)kmalloc(vesa_ctx.mode.pitch * vesa_ctx.mode.height);

	if (img_buf == NULL)
		return NULL;
*/
	img_buf = GRAPHIC_BUFFER_LOCATION;

	start = (u8 *)bmp + bmp->fileheader.fileoffset_to_pixelarray;
	if (bpp == 24)
		fill_image_24(img_buf, start, width, height);
	else
		fill_image_32(img_buf, start, width, height);
	return img_buf;
}
