#include <stdio.h>
#include <unistd.h>
#include <wait.h>
#include <stdlib.h>
#include <signal.h>

void hello_signal(int signum) {
	printf("Signal Received 5/5: %i\n", signum);
}

int main() {
	printf("initialise Signals test\n");

	struct sigaction sa;

	/* sa.sa_handler = hello_signal; */
	/* sa.sa_flags = SA_RESTART; */
	sa.sa_handler = SIG_DFL; 
	sa.sa_flags = 0; 
					
    if (sigaction(SIGSTOP, &sa, NULL) == -1) {
		printf("sigaction failed\n");
	}

    if (sigaction(SIGCONT, &sa, NULL) == -1) {
		printf("sigaction failed\n");
	}
	sa.sa_handler = hello_signal;
	sa.sa_flags = 0;
    if (sigaction(SIGUSR1, &sa, NULL) == -1) {
		printf("sigaction failed\n");
	}
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
		ret = kill(pid, SIGUSR1);
		ret = kill(pid, SIGCONT);
		sleep(10);
		printf("i am the child i will exit\n");
		exit(0);
	} else {
		int status;
		printf("i'm the father preparing to wait\n");
		int ret = wait(&status);
		printf("after wait\n");
		if (ret == -1) {
			exit(1);
		}
		while (42);
	}
	return 0;
}
