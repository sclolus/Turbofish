#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include "tools.h"

size_t NUMBER = 0;

static void create_and_unlink() {
	char filename[100];

	srand16(0x42);

	pid_t pid = getpid();
	sprintf(filename, "./file_%d_%lu", pid, NUMBER++);

	printf("creating file: %s\n", filename);
	int fd = open(filename, O_RDWR | O_CREAT, 0644);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	int ret = access(filename, F_OK);
	if (ret == -1) {
		perror("access");
		exit(1);
	}
	ret = unlink(filename);
	if (ret == -1) {
		perror("unlink");
		exit(1);
	}
	ret = access(filename, F_OK);
	if (!(ret == -1)) {
		exit(1);
	}
}

int main() {
	for (int i = 0; i < 40; i++) {
		create_and_unlink();
	}
}
