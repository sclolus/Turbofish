#include <stdio.h>
#include <wait.h>
#include <unistd.h>
#include <stdlib.h>

int main(void)
{
	pid_t pid = fork();
	if (pid < 0) {
		printf("fork Failure\n");
		exit(1);
	} else if (pid == 0) {
		sleep(1);
		raise(SIGSTOP);
		sleep(2);
		printf("end child of life\n");
	} else {
		int status;

		sleep(2);
		kill(pid, SIGCONT);
		pid_t ret = waitpid(pid, &status, WUNTRACED | WCONTINUED);
		if (ret < 0) {
			printf("waitpid failure\n");
			exit(1);
		}
		printf("raw son status: %hhx   WIFSTOPPED result: %i WIFCONTINUED result: %i\n", status, WIFSTOPPED(status), WIFCONTINUED(status));
		ret = waitpid(pid, &status, WUNTRACED | WCONTINUED);
		if (ret < 0) {
			printf("waitpid failure\n");
			exit(1);
		}
		printf("raw son status: %hhx   WIFSTOPPED result: %i\n", status, WIFSTOPPED(status));

		sleep(1);
		kill(pid, SIGCONT);
		ret = waitpid(pid, &status, WUNTRACED | WCONTINUED);
		if (ret < 0) {
			printf("waitpid failure\n");
			exit(1);
		}
		printf("raw son status: %hhx   WIFCONTINUED result: %i\n", status, WIFCONTINUED(status));
	}
	return 0;
}
