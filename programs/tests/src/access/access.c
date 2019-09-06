#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include "tools.h"

size_t NUMBER = 0;

void test_access(mode_t mode, int amode) {
	char filename[100];

	srand16(0x42);

	pid_t pid = getpid();
	sprintf(filename, "./file_%d_%lu", pid, NUMBER++);

	printf("creating file: %s\n", filename);

	int fd = open(filename, O_CREAT | O_TRUNC, mode);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	int ret = access(filename, amode);
	if (ret == -1) {
		perror("access");
		exit(1);
	}
	ret = unlink(filename);
	if (ret == -1) {
		perror("unlink");
		exit(1);
	}
}

int main() {
	test_access(0, F_OK);

	test_access(S_IXUSR, X_OK);
	test_access(S_IWUSR, W_OK);
	test_access(S_IRUSR, R_OK);

	test_access(S_IRWXU, R_OK | W_OK | X_OK);


	test_access(S_IXUSR | S_IWUSR, X_OK | W_OK);

	test_access(S_IXUSR | S_IRUSR, X_OK | R_OK);
	
	test_access(S_IWUSR | S_IRUSR, W_OK | R_OK);
}
