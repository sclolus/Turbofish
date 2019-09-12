#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <assert.h>

int main() {
	char filename[100];
	char newname[100];

	pid_t pid = getpid();
	sprintf(filename, "./file_%d", pid);
	sprintf(newname, "./renamed_file_%d", pid);

	printf("creating file: %s\n", filename);
	int fd = open(filename, O_RDWR | O_CREAT, 0644);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	assert(-1 != open(newname, O_RDWR | O_CREAT, 0644));

	if (rename(filename, newname) == -1) {
		perror("rename");
		exit(1);
	}
	struct stat buf1;

	assert(stat(filename, &buf1) == -1);
	assert(stat(newname, &buf1) == 0);
	assert(unlink(newname) == 0);
}
