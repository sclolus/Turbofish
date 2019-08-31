#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>
#include <string.h>
#include <wait.h>

#include "tools.h"
#include "pipe.h"

/*
 * Reduce the time-life of the test by RATIO_TEST
 */
#define RATIO_TEST 8

/*
 * Nazi Pipe Reader/Writer test
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

		size_t total_len = strlen(s) / RATIO_TEST;
		size_t current = 0;
		char *ptr = (char *)s;

		srand16(0x42);

		while (current < total_len) {
			size_t trans = (size_t)rand16(PIPE_BUF_LEN * 2);
			if (trans > (total_len - current)) {
				trans = total_len - current;
			}
			int n = write(1, ptr, trans);
			if (n < 0) {
				perror("write");
				exit(1);
			}
			ptr += trans;
			current += trans;
		}
		sleep(2);
		dprintf(2, "write finished !\n");
		sleep(1);
		exit(0);
	} else {
		char buf[PIPE_BUF_LEN * 2];

		int n;
		char *ptr = (char *)s;

		if (close(fd[1]) < 0) {
			perror("close");
			exit(1);
		}

		srand16(0x42 * 2);

		while ((n = read(fd[0], buf, (size_t)rand16(PIPE_BUF_LEN * 2 - 1) + 1)) > 0) {
			buf[n] = '\0';
			printf("%s", buf);
			if (memcmp(buf, ptr, n) != 0) {
				printf("Bad Message received ! %s\n", buf);
				exit(1);
			}
			ptr += n;
		}
		printf("\n");
	}
	return 0;
}
