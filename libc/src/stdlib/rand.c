#include <stdlib.h>
#include <fcntl.h>

int _rdrand(void);

int rand(void)
{
	/*
	// RDRAND HAVE DIFFICULTIES TO WORK ON QEMU. USE A HACK INSTEAD
	short unsigned int buf;

	int fd = open("/dev/random", O_RDONLY);
	if (fd < 0) {
		return -1;
	}
	int r = read(fd, &buf, sizeof(buf));
	if (r < 0) {
		return -1;
	}
	close(fd);
	return (buf % (RAND_MAX + 1));
	*/

	int r = _rdrand();

	if (r < 0) {
		r *= -1;
	}
	return r % (RAND_MAX + 1);
}
