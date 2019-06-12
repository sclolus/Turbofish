#include <stdio.h>
#include <unistd.h>
#include <wait.h>
#include <stdlib.h>
#include <signal.h>

int main() {
	printf("initialise Signals test\n");

	pid_t pid = getpid();
	printf("pid of process '%u'\n", pid);

	int child_pid = fork();
	if (child_pid == 0) {
		printf("i am the child i will sigstop my father\n");
		sleep(1);
		int ret = kill(pid, SIGSTOP);
		if (ret == -1) {
			printf("kill failed\n");
		}
		sleep(1);
		printf("i am the child i will sigcont my father\n");
		ret = kill(pid, SIGCONT);
		sleep(1);
		printf("i am the child i will exit\n");
		exit(0);
	} else {
		int status;
		printf("i'm the father preparing to wait\n");
		int ret = wait(&status);
		if (ret == -1) {
			exit(1);
		}
	}
	return 0;
}
