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
	char file_in_newname[255];

	pid_t pid = getpid();
	sprintf(filename, "./dir_test_rename_dir_not_empty_%d", pid);
	sprintf(newname, "./renamed_dir_%d", pid);

	printf("creating dir: %s\n", filename);

	assert(mkdir(filename, 0744) == 0);

	assert(mkdir(newname, 0744) == 0);

	sprintf(file_in_newname, "%s/file", newname);
	assert(-1 != open(file_in_newname, O_RDWR | O_CREAT, 0644));

	assert(rename(filename, newname) == -1);

	assert(unlink(file_in_newname) == 0);
	assert(rmdir(newname) == 0);
	assert(rmdir(filename) == 0);
}
