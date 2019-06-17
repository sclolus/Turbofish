#include <stdio.h>
#include <wait.h>
#include <unistd.h>
#include <stdlib.h>
#include <signal.h>

void hello_signal(int signum) {
	printf("Signal Received 5/5: %i\n", signum);
	printf("end signal\n");
}

static sigset_t signal_mask; /* signals to block */

int main (void) {
	pid_t pid = getpid();
	printf("F: pid of process '%u'\n", pid);
	struct sigaction sa;

	sa.sa_handler = hello_signal;
	sa.sa_flags = 0;
	if (sigaction(SIGUSR2, &sa, NULL) == -1) {
		printf("F: sigaction failed\n");
	}

	int f = fork();
	if (f < 0) {
		printf("F: Fork Failed\n");
		exit(1);
	}
	else if (f > 0) {
		printf("F: I am a father of child: %i\n", f);
		int status;
		int ret;
		if ((ret = wait(&status)) < 0) {
			printf("F: wait failed: ret: %i\n", ret);
		} else {
			printf("F: wait success: child_pid: %i exit_status: %i\n", ret, status);
		}
	}
	else {
		printf("C: I am the child, I will Interrupt my father in 2 s\n");
		sleep(2);
		printf("C: Interrupted !\n");
		kill(pid, SIGUSR2);
		sleep(2);
		printf("C: I return()\n");
	}
	return 0;
}
