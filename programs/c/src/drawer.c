#include <sys/types.h>
#include <stdint.h>
#include <stdio.h>
#include <fcntl.h>
#include <unistd.h>
#include <termios.h>
#include <sys/ioctl.h>
#include <string.h>
#include <stdlib.h>

int main(int ac, char ** av) {
	uint32_t color = 0xff0000;
	if (ac == 2) {
		color = atoi(av[1]);
	}
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
	for (size_t i = 0; i < width * height; i++) {
		*((uint32_t *)&buffer[i * bpp / 8]) = color;
	}
	if (buffer == NULL) {
		printf("no memory to allocate buffer");
		exit(1);
	}
	
	int fb = open("/dev/fb", O_WRONLY);
	if (fb == -1) {
		perror("open");
		exit(1);
	}
	int written = write(fb, buffer, width * height * bpp / 8);
	if (written == -1) {
		perror("write");
		exit(1);
	}
}
