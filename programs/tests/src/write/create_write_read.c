#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include "tools.h"

size_t NB_TESTS = 10;
size_t FILE_SIZE_MAX = 10000;


// return a vector of random char of size size
char *random_vec(size_t size) {
	char *vec = malloc(size);
	if (vec == NULL) {
		perror("malloc");
		exit(1);
	}
	for (size_t i = 0; i < size; i++) {
		vec[i] = (unsigned int)rand16(UCHAR_MAX);
	}
	return vec;
}

void	read_write_of_size(size_t size) {
	//create a disk of size of the file + a little space for metadata
	char filename[100];

	pid_t pid = getpid();
	sprintf(filename, "./simple_file_%d_%d", pid, (int)rand16(USHRT_MAX));

	int fd = open(filename, O_RDWR | O_CREAT | O_TRUNC, 0644);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	/* WRITE with the ext2 */
	char * v = random_vec(size);
	int count = write(fd, v, size);
	if (count == -1) {
		perror("write");
		exit(1);
	}
	if ((size_t)count != size) {
		dprintf(2, "all data has not been written\n");
		exit(1);
	}
	printf("%d bytes written\n", count);

	/* READ with the ext2 */
	char * buf = malloc(size);
	if (buf == NULL) {
		perror("malloc");
		exit(1);
	}
	memset(buf, 42, size);

	int ret = lseek(fd, 0, SEEK_SET);
	if (ret == -1) {
		perror("lseek");
		exit(1);
	}
	if (ret != 0) {
		dprintf(2, "lseek is bullshit\n");
		exit(1);
	}
	count = read(fd, buf, size);

	if (count == -1) {
		perror("read");
		exit(1);
	}
	if ((size_t)count != size) {
		dprintf(2, "all data has not been readen size: %u, count: %d\n", size, count);
		exit(1);
	}
	printf("%d bytes readen\n", count);

	for (size_t i = 0; i < size; i++) {
		if (buf[i] != v[i]) {
			dprintf(2, "data is not the same as index i: %d, readen: %x, writen: %x\n", i, buf[i], v[i]);
			exit(1);
		}
	}
}

int main() {
	srand16(0x42);

	for (size_t i = 0; i < NB_TESTS; i++) {
		printf("test nbr %lu\n", i);
		size_t size = ((size_t)rand16(USHRT_MAX)) % FILE_SIZE_MAX;
        read_write_of_size(size);
    }
}
