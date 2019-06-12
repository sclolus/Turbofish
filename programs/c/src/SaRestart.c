#include <stdio.h>
#include <wait.h>
#include <unistd.h>
#include <signal.h>

void hello_signal(int signum) {
	printf("Signal Received 5/5: %i\n", signum);
	printf("end signal\n");
}

static sigset_t   signal_mask;  /* signals to block         */


int main () {
	pid_t pid = getpid();
	printf("pid of process '%u'\n", pid);
	// 10 is the number of SIGUSR
	struct sigaction sa;

	sa.sa_handler = hello_signal;
	sa.sa_flags = 0;
	if (sigaction(10, &sa, NULL) == -1) {
		printf("sigaction failed\n");
	}

	int child_pid = fork();
	if (child_pid > 0) {
		int status;
		if (wait(&status) == -1) {
			printf("wait failed\n");
		}
		printf("after wait");
	}
	else {
		sleep(2);
		kill(pid, 10);
		sleep(2);
	}
	return 0;
}
