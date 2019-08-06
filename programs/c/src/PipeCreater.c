#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <sys/wait.h>

int main(void)
{
	int fd1[2];

	pipe(fd1);
	printf("fd1[0]: %i\n", fd1[0]);
	printf("fd1[1]: %i\n", fd1[1]);

	close(fd1[0]);
	int fd2[2];

	pipe(fd2);
	printf("fd2[0]: %i\n", fd2[0]);
	printf("fd2[1]: %i\n", fd2[1]);

	close(fd1[1]);
	int fd3[2];

	pipe(fd3);
	printf("fd3[0]: %i\n", fd3[0]);
	printf("fd3[1]: %i\n", fd3[1]);

	printf("Test fd numbers\n");
	if (fd2[0] != 3 || fd2[1] != 5 || fd3[0] != 4 || fd3[1] != 6) {
		exit(-1);
	}

	pid_t pid = fork();
	if (pid < 0) {
		printf("Fork failed !\n");
		exit(-1);
	} else if (pid == 0) {
		printf("I am the child !\n");
		int fd4[2];

		pipe(fd4);
		printf("fd4[0]: %i\n", fd4[0]);
		printf("fd4[1]: %i\n", fd4[1]);
		printf("child exit\n");
	} else {
		printf("I am the father !\n");

		int status;
		wait(&status);

		sleep(1);
		printf("Father exit\n");
	}
	return 0;
}
