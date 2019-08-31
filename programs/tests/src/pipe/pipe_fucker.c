#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>

#define TEST_STRING "banane"

#include "pipe.h"

/*
 * Simple Pipe reader/writer test + close writer
 */
int main(void)
{
	int fd[2];

	if (pipe(fd) == -1) {
		perror("pipe error");
		exit(1);
	}
	pid_t pid = fork();
	if (pid < 0) {
		perror("fork error");
		exit(1);
	} else if (pid == 0) {
		if (close(fd[0]) < 0) {
			perror("close failed");
			exit(1);
		}
		dup2(fd[1], 1);
		write(1, TEST_STRING, 6);
		sleep(1);
		exit(0);
	} else {
		char buf[PIPE_BUF_LEN];

		int n;
		close(fd[1]);
		if ((n = read(fd[0], buf, PIPE_BUF_LEN)) < 0) {
			perror("read");
			exit(1);
		}
		printf("n value: %i\n", n);
		buf[n] = '\0';
		printf("string received: %s\n", buf);
		if (strcmp(TEST_STRING, buf) != 0) {
			printf("Differences was detected ! %s Vs %s\n", TEST_STRING, buf);
			exit(1);
		}

		if ((n = read(fd[0], buf, PIPE_BUF_LEN)) < 0) {
			perror("read");
			exit(1);
		}
		if (n != 0) {
			printf("n must be set to 0\n");
			exit(1);
		}
		buf[n] = '\0';
		printf("string received: %s\n", buf);
		printf("n value: %i\n", n);
		if ((n = read(fd[0], buf, PIPE_BUF_LEN)) < 0) {
			perror("read");
			exit(1);
		}
		if (n != 0) {
			printf("n must be set to 0\n");
			exit(1);
		}
		buf[n] = '\0';
		printf("string received: %s\n", buf);
		printf("n value: %i\n", n);
	}
	return 0;
}
