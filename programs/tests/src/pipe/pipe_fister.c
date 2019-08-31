#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <wait.h>

#include "pipe.h"

/*
 * SIGPIPE test (when all readers are gone): Before & After
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
		printf("Into the son\n");
		if (close(fd[0]) < 0) {
			perror("close failed");
			exit(1);
		}
		dup2(fd[1], 1);
		sleep(1);
		dprintf(2, "Attempt to Write\n");
		if (write(1, "banane", 6) >= 0) {
			dprintf(2, "SIGPIPE must be received !\n");
			exit(1);
		}
		perror("");
	} else {
		if (close(fd[1]) < 0) {
			perror("close failed");
			exit(1);
		}
		if (close(fd[0]) < 0) {
			perror("close failed");
			exit(1);
		}
		int res = waitpid(-1, NULL, 0);
		if (res == -1) {
			dprintf(2, "waitpid -1 failed");
			exit(1);
		}
	}

	int fd2[2];

	if (pipe(fd2) == -1) {
		perror("pipe error");
		exit(1);
	}
	pid = fork();
	if (pid < 0) {
		perror("fork error");
		exit(1);
	} else if (pid == 0) {
		printf("Into the son\n");
		if (close(fd2[0]) < 0) {
			perror("close failed");
			exit(1);
		}
		dup2(fd2[1], 1);
		dprintf(2, "Attempt to Write\n");
		if (write(1, s, PIPE_BUF_LEN * 2) >= 0) {
			dprintf(2, "SIGPIPE must be received !\n");
			exit(1);
		}
		perror("new message: ");
	} else {
		if (close(fd2[1]) < 0) {
			perror("close failed");
			exit(1);
		}
		printf("closing reading pipe in one second\n");
		sleep(2);
		if (close(fd2[0]) < 0) {
			perror("close failed");
			exit(1);
		}
		int res = waitpid(-1, NULL, 0);
		if (res == -1) {
			dprintf(2, "waitpid -1 failed");
			exit(1);
		}
	}
	printf("exit normally\n");
	return 0;
}
