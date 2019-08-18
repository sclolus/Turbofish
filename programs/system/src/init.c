#include <sys/wait.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>

const char program[] = "/bin/shell";

int main(void)
{
	pid_t pid = fork();
	if (pid < 0) {
		printf("%s: Fork failed\n", __func__);
		exit(1);
	} else if (pid == 0) {
		int ret = execve(program, NULL, NULL);
		if (ret < 0) {
			printf("%s: Execve failed\n", __func__);
			exit(1);
		}
	} else {
		int status;

		while (1) {
			pid_t ret = wait(&status);
			if (ret < 0) {
				printf("%s: Wait failed\n", __func__);
				exit(1);
			}
			printf("Deleting zombie: pid = %i status = %i\n", ret, status);
		}
	}
	return 0;
}
