
#include <stdio.h>
#include <stdlib.h>
#include <fcntl.h>
#include <unistd.h>

int main(int argc, char **argv)
{
	if (argc != 3) {
		dprintf(STDERR_FILENO, "%s: usage %s filename file size\n", argv[0], argv[0]);
		exit(-1);
	}
	unsigned int filesize = (unsigned int)strtoll(argv[2], NULL, 10);
	if ((filesize & 0x3) != 0) {
		dprintf(STDERR_FILENO, "size must be a multiple of 4\n");
		exit(-1);
	}
	int fd = open(argv[1], O_WRONLY | O_TRUNC | O_CREAT, S_IRWXU | S_IRGRP | S_IROTH);
	if (fd < 0) {
		dprintf(STDERR_FILENO, "Cannot create or open %s\n", argv[1]);
		exit(-1);
	}
	for (unsigned int i = 0; i < filesize; i += 4) {
		write(fd, &i, 4);
	}
	close(fd);
	return 0;
}
