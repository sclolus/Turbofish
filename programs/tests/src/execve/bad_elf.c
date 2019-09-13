#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <sys/stat.h>

extern char **environ;

int main() {
	char filename[100];

	pid_t pid = getpid();
	sprintf(filename, "./bad_elf_%d", pid);

	printf("creating file: %s\n", filename);
	int fd = open(filename, O_RDWR | O_CREAT, 0744);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	mode_t	mode = 0777;
	assert(0 == chmod(filename, mode));
	assert(-1 == execve(filename, (char *[]){filename, NULL}, environ));
	assert(unlink(filename) == 0);
	return EXIT_SUCCESS;
}
