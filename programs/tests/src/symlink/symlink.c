#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <assert.h>

void test_symlink(char *salt) {
	char filename[100];
	char linkpath[100];

	pid_t pid = getpid();
	sprintf(filename, "./%sfile_%d", salt, pid);

	printf("creating file: %s\n", filename);
	int fd = open(filename, O_RDWR | O_CREAT, 0644);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	sprintf(linkpath, "./file_%d_link_path", pid);
	int ret = symlink(filename, linkpath);
	if (ret == -1) {
		perror("symlink");
		exit(1);
	}

	struct stat buf;
	ret = lstat(linkpath, &buf);
	if (ret == -1) {
		perror("lstat");
		exit(1);
	}
	assert(S_ISLNK(buf.st_mode));

	char bufname[100];
	ssize_t size = readlink(linkpath, bufname, 100);
	if (size == -1) {
		perror("readlink");
	}

	printf("'%s', '%s'", bufname, filename);
	assert(strcmp(bufname, filename) == 0);

	assert(stat(linkpath, &buf) == 0);

	assert(!S_ISLNK(buf.st_mode));

	assert(unlink(filename) == 0);
	assert(unlink(linkpath) == 0);
}

int main() {
	test_symlink("little_target");
	test_symlink("bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbig_target");
}
