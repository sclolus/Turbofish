#include <sys/types.h>
#include <stdint.h>
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <termios.h>
#include <sys/ioctl.h>
#include <string.h>
#include <stdlib.h>
//! Uselfull tools to read BMP files

/// Basic header of a BMP image
struct BmpImage {
	uint8_t signature[2];
	uint32_t filesize;
	uint32_t reserved;
	uint32_t fileoffset_to_pixelarray;

	uint32_t dib_header_size;
	uint32_t width;
	uint32_t height;
	uint16_t planes;
	uint16_t bits_per_pixel;
	uint32_t compression;
	uint32_t image_size;
	uint32_t y_pixel_parameter;
	uint32_t x_pixel_parameter;
	uint32_t num_colors_pallette;
	uint32_t most_important_color;
};

// Last pixel line of bitmap format is the first line of standard screen format
void	fill_image(
	uint8_t *output,
	uint8_t *image,
	size_t width,
	size_t height,
	size_t bpp,
	struct BmpImage header
	) {
	/* 
	 * let ptr_input = unsafe { slice::from_raw_parts(image, header.filesize as usize) };
	 * let ptr_output =
	 * 	unsafe { slice::from_raw_parts_mut(output, ) };
	 */

	// offset to last input line
	size_t input_index = (header.height - 1) * header.width * 3;

	for (size_t i = 0; i < width * height * bpp / 8 ; i++) {
		if (bpp == 32 && (i % 4) == 3) {
			continue;
		}
		output[i] = image[input_index];
		input_index += 1;
		// check if on end of pixel line
		if ((input_index % (header.width * 3)) == 0 
			&& input_index != header.width * 3)
		{
			input_index -= header.width * 3 * 2;
		}
	}
}

/// This function implemente no scale change, only work with 1024 * 768 * (24b || 32b bitmap)
int		draw_image(
	const struct BmpImage *image,
	uint8_t *buffer,
	size_t width,
	size_t height,
	size_t bpp
	) {
	if (bpp != 32 && bpp != 24) {
		return -1;
	} else {
		struct BmpImage header = *image;
		if (header.bits_per_pixel != 24 && header.width != 1024 && header.height != 768) {
			return -1;
		} else {
			uint8_t *ptr = (uint8_t*)image;
			fill_image(
				buffer,
				ptr + header.fileoffset_to_pixelarray,
				width,
				height,
				bpp,
				header
				);
		}
	}
	return 0;
}

#define BUFFER_SIZE  (4096 * 10)

uint8_t *read_to_end(int fd) {
	uint8_t *tmp_buffer = malloc(BUFFER_SIZE);
	uint8_t *buffer = malloc(BUFFER_SIZE);
	size_t	size = 0;
	int len_readen;

	printf("size: %lu\n", size);
	while ((len_readen = read(fd, tmp_buffer, BUFFER_SIZE)) > 0) {
		printf("size: %lu\n", size);
		printf("len readen: %d\n", len_readen);
		buffer = realloc(buffer, size + len_readen);
		if (buffer == NULL) {
			printf("no memory to allocate buffer");
			exit(1);
		}
		memcpy(buffer + size, tmp_buffer, len_readen);
		size += len_readen;
	}
	if (len_readen == -1) {
		perror("read");
		exit(1);
	}
	return buffer;
}

int main(int ac, char ** av) {
	if (ac != 2) {
		printf("usage: bmploader image_file");
		return 1;
	}
	int fd = open(av[1], O_RDONLY);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	uint8_t* data = read_to_end(fd);
	struct winsize win;
	memset(&win, 0, sizeof(struct winsize));
	int ret = ioctl(0, TIOCGWINSZ, &win);
	if (ret == -1) {
		perror("ioctl");
		exit(1);
	}
	size_t width = win.ws_xpixel;
	size_t height =  win.ws_ypixel;
	size_t bpp = win.bpp;
	uint8_t *buffer = malloc(width * height * bpp / 8);
	if (buffer == NULL) {
		printf("no memory to allocate buffer");
		exit(1);
	}
	
	draw_image((struct BmpImage *)data, buffer, width,height, bpp);

	int fb = open("/dev/fb", O_WRONLY);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	int written = write(fb, buffer, width * height * bpp / 8);
	if (written == -1) {
		perror("write");
		exit(1);
	}
}
