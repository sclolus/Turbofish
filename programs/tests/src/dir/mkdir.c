#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>

size_t NUMBER = 0;

int main() {
	char dirname[100];

	pid_t pid = getpid();
	sprintf(dirname, "./dir_%d_%lu", pid, NUMBER++);

	printf("creating dir: %s\n", dirname);
	int fd = mkdir(dirname, 0644);
	if (fd == -1) {
		perror("mkdir");
		exit(1);
	}
	struct stat buf;
	int ret = stat(dirname, &buf);
	if (ret == -1) {
		perror("stat");
		exit(1);
	}
	if (!S_ISDIR(buf.st_mode)) {
		dprintf(2, "it should be a directory: %s\n", dirname);
	}
	ret = rmdir(dirname);
	if (ret == -1) {
		perror("rmdir");
		exit(1);
	}
	ret = stat(dirname, &buf);
	if (!(ret == -1)) {
		perror("directory shall no longer exist");
		exit(1);
	}
}
