#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>

size_t NUMBER = 0;

int main() {
	char filename[100];

	pid_t pid = getpid();
	sprintf(filename, "./file_%d_%lu", pid, NUMBER++);

	printf("creating file: %s\n", filename);
	int fd = open(filename, O_RDWR | O_CREAT, 0644);
	if (fd == -1) {
		perror("open");
		exit(1);
	}

	printf("creating dir: %s\n", filename);
	int ret = mkdir(filename, 0644);
	if (!(ret == -1)) {
		dprintf(2, "perror should have failed cause a file exists: %s\n", filename);
		exit(1);
	}
	ret = unlink(filename);
	if (ret == -1) {
		perror("unlink");
		exit(1);
	}
}
