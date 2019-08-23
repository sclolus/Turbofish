#include <stdio.h>
#include <wait.h>
#include <unistd.h>
#include <stdlib.h>

int main(void)
{
	pid_t pid = fork();
	if (pid < 0) {
		printf("fork Failure\n");
	} else if (pid == 0) {
		raise(SIGSTOP);
		printf("end child of life\n");
		while(1) {
			sleep(0.01);
		}
	} else {
		int status;
		pid_t ret = waitpid(pid, &status, WUNTRACED);
		if (ret < 0) {
			perror("waitpid failed:");
			exit(1);
		}
		if (!WIFSTOPPED(status)) {
			dprintf(2, "WIFSTOPPED should be true");
			exit(1);
		}
		kill(pid, SIGCONT);
		printf("raw son status: %hhx   WIFSTOPPED result: %i WIFCONTINUED result: %i\n", status, WIFSTOPPED(status), WIFCONTINUED(status));
		ret = waitpid(pid, &status, WUNTRACED | WCONTINUED);
		if (ret < 0) {
			perror("waitpid failed:");
			exit(1);
		}
		if (!WIFCONTINUED(status)) {
			dprintf(2, "WIFCONTINUED should be true");
			exit(1);
		}
		printf("raw son status: %hhx   WIFSTOPPED result: %i\n", status, WIFSTOPPED(status));

		sleep(1);
		kill(pid, SIGKILL);
		if (ret < 0) {
			perror("kill failed:");
			exit(1);
		}
		ret = waitpid(pid, &status, WUNTRACED | WCONTINUED);
		if (ret < 0) {
			perror("waitpid failed:");
			exit(1);
		}
		if (!WIFSIGNALED(status)) {
			dprintf(2, "WIFCONTINUED should be true");
			exit(1);
		}
		if (WTERMSIG(status) != SIGKILL) {
			dprintf(2, "WTERMSIG should be SIGKILL");
			exit(1);
		}
		if (WIFEXITED(status)) {
			dprintf(2, "WIFEXITED should be false");
			exit(1);
		}
		printf("raw son status: %hhx   WIFCONTINUED result: %i\n", status, WIFCONTINUED(status));
	}
	return 0;
}
