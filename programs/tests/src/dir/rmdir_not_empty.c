#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <sys/stat.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>

size_t NUMBER = 0;

int main() {
	char filename[100];
	char dirname[50];

	pid_t pid = getpid();
	sprintf(dirname, "./dir_%d_%lu", pid, NUMBER++);

	printf("creating dir: %s\n", dirname);
	int ret = mkdir(dirname, 0644);
	if (ret == -1) {
		perror("mkdir");
		exit(1);
	}
	sprintf(filename, "./%s/file_%d_%lu", dirname, pid, NUMBER++);

	printf("creating file: %s\n", filename);
	int fd = open(filename, O_RDWR | O_CREAT, 0644);
	if (fd == -1) {
		perror("open");
		exit(1);
	}

	struct stat buf;
	ret = stat(dirname, &buf);
	if (ret == -1) {
		perror("stat");
		exit(1);
	}
	if (!S_ISDIR(buf.st_mode)) {
		dprintf(2, "it should be a directory: %s\n", dirname);
	}
	ret = rmdir(dirname);
	if (!(ret == -1)) {
		perror("rmdir");
		exit(1);
	}
	if (!(errno == ENOTEMPTY)) {
		dprintf(2, "errno should be set to ENOTEMPTY: %s\n", dirname);
		exit(1);
	}
	ret = unlink(filename);
	if (ret == -1) {
		perror("unlink");
		exit(1);
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
