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
	sprintf(filename, "./dir_%d", pid);
	sprintf(newname, "./renamed_dir_%d", pid);

	printf("creating dir: %s\n", filename);

	assert(mkdir(filename, 0644) == 0);

	assert(mkdir(newname, 0644) == 0);

	if (rename(filename, newname) == -1) {
		perror("rename");
	exit(1);
	}
	struct stat buf1;

	assert(stat(filename, &buf1) == -1);
	assert(stat(newname, &buf1) == 0);
	assert(rmdir(newname) == 0);
}
