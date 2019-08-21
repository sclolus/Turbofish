#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>

int main(void)
{
	int fd[2];

	int ret = pipe(fd);
	if (ret < 0) {
		printf("Pipe failure\n");
		exit(1);
	}
	pid_t pid = fork();
	if (pid < 0) {
		printf("Fork failure\n");
		exit(1);
	} else if (pid == 0) {
		write(fd[1], "banane", 6);
	} else {
		char buf[100];
		buf[6] = '\0';
		int ret = read(fd[0], buf, 6);
		if (ret < 0) {
			printf("Read failure\n");
			exit(1);
		}
		printf("Father has read: %s\n", buf);
	}
	return 0;
}
