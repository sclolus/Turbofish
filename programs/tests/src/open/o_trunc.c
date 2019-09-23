#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

int main() {
	char filename[100];

	pid_t pid = getpid();
	sprintf(filename, "./test_otrunc_file_%d", pid);

	printf("creating file: %s\n", filename);
	int fd = open(filename, O_RDWR | O_CREAT, 0644);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	char banane[] = "banane";
	assert(write(fd, banane, sizeof(banane)) == sizeof(banane));
	char buf[100];

	lseek(fd, 0, SEEK_SET);
	int count = read(fd, buf, sizeof(banane));
	printf("%d bytes readen, buf: %s", count, buf);
	if (count == -1) {
		perror("read");
	}
	assert(count == sizeof(banane));
	assert(strcmp(buf, banane) == 0);
	close(fd);

	fd = open(filename, O_RDWR | O_TRUNC , 0644);
	memset(buf, 0, sizeof(banane));

	count = read(fd, buf, sizeof(banane));
	assert(count == 0);
	close(fd);
	assert(unlink(filename) == 0);
}
