#include <sys/stat.h>
#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include <sys/wait.h>

char MESSAGE[] = "hello world!";

void child(char *filename) {
	int fd = open(filename, O_WRONLY);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	int ret = write(fd, MESSAGE, sizeof(MESSAGE));
	if (ret != (int)sizeof(MESSAGE)) {
		dprintf(2, "bytes writen %d\n", ret);
		exit(1);
	}
}

void father(char *filename) {
	char buf[100];

	int fd = open(filename, O_RDONLY);
	if (fd == -1) {
		perror("open");
		exit(1);
	}

	int ret = read(fd, buf, sizeof(buf));
	if (ret != (int)sizeof(MESSAGE)) {
		dprintf(2, "bytes readen %d\n", ret);
		exit(1);
	}

	assert(strcmp(buf, MESSAGE) == 0);
}

int main() {
	char filename[100];

	pid_t pid = getpid();
	sprintf(filename, "./my_fifo_%d", pid);

	printf("creating fifo: %s\n", filename);
	unlink(filename);
	assert(mkfifo(filename, 0644) == 0);


	int child_pid = fork();
	if (child_pid == -1) {
		perror("fork");
		exit(1);
	} else if (child_pid == 0) {
		child(filename);
		exit(0);
	} else {
		father(filename);
		int status;
		int ret = wait(&status);
		if (ret == -1) {
			exit(1);
		}
		assert(0 == unlink(filename));
		return EXIT_SUCCESS;
	}
}
