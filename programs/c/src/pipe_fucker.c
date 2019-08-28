#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <errno.h>

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
		write(1, "banane", 6);
		sleep(1);
		exit(0);
	} else {
		char buf[100];

		int n;
		close(fd[1]);
		if ((n = read(fd[0], buf, 32)) < 0) {
			perror("read");
			exit(1);
		}
		printf("n value: %i\n", n);
		buf[n] = '\0';
		printf("string received: %s\n", buf);
		if ((n = read(fd[0], buf, 32)) < 0) {
			perror("read");
			exit(1);
		}
		buf[n] = '\0';
		printf("string received: %s\n", buf);
		printf("n value: %i\n", n);
		if ((n = read(fd[0], buf, 32)) < 0) {
			perror("read");
			exit(1);
		}
		buf[n] = '\0';
		printf("string received: %s\n", buf);
		printf("n value: %i\n", n);

	}
	return 0;
}
