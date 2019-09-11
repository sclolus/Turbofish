#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>

int main() {
	char filename[100];

	pid_t pid = getpid();
	sprintf(filename, "./file_%d", pid);

	printf("creating file: %s\n", filename);
	int fd = open(filename, O_RDWR | O_CREAT, 0644);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	int ret = unlink(filename);
	if (ret == -1) {
		perror("unlink");
		exit(1);
	}

	char s[] = "banane";
	ret = write(fd, s, sizeof(s));
	if (ret == -1) {
		perror("write");
		exit(1);
	}
	lseek(fd, 0, SEEK_SET);

	char r[sizeof(s)];
	ret = read(fd, r, sizeof(s));
	if (ret == -1) {
		perror("read");
		exit(1);
	}
	if (strcmp(r, s) != 0) {
		printf("error data writen != data read\n");
		exit(1);
	}
	close(fd);
}
