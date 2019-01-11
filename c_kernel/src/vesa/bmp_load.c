
#include "internal_bmp.h"

#include "vesa.h"
#include "dynamic_allocator.h"

#include "libft.h"

#ifdef DEBUG_IMAGE
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
#endif

static void	fill_image_24(
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

u8		*bmp_load(u8 *file_offset, int *width, int *height, int **data)
{
	struct bitmap	*img;
	u8		*start;
	u8		*img_buf;

	img = (struct bitmap *)file_offset;

#ifdef DEBUG_IMAGE
	paste_fileheader((struct bitmap *)img);
#endif

	*width = img->bitmapinfoheader.width;
	*height = img->bitmapinfoheader.height;
	img_buf = (u8 *)kmalloc(vesa_ctx.mode.pitch * vesa_ctx.mode.height);
	if (img_buf == NULL)
		return NULL;

	start = (u8 *)img + img->fileheader.fileoffset_to_pixelarray;
	if (vesa_ctx.mode.bpp == 24)
		fill_image_24(
				img_buf,
				start,
				*width,
				*height);
	else
		fill_image_32(
				img_buf,
				start,
				*width,
				*height);
	(void)data;
	return img_buf;
}
