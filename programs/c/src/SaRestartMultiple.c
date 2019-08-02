#include <stdio.h>
#include <wait.h>
#include <unistd.h>
#include <stdlib.h>
#include <signal.h>

void hello_signal(int signum) {
	printf("Signal Received 5/5: %i\n", signum);
	printf("end signal\n");
}

void tstp(int signum) {
	printf("B: tstp signal handler %i\n", signum);
	raise(SIGTTIN);
	printf("E: tstp signal handler %i\n", signum);
}

void ttin(int signum) {
	printf("ttin signal handler %i\n", signum);
}

void ttou(int signum) {
	printf("ttou signal handler %i\n", signum);
}

int main (void) {
	pid_t pid = getpid();
	printf("F: pid of process '%u'\n", pid);
	struct sigaction sa;

	sa.sa_handler = hello_signal;
	sa.sa_flags = SA_RESTART;
	if (sigaction(SIGUSR2, &sa, NULL) == -1) {
		printf("F: sigaction failed\n");
	}
	sa.sa_handler = &tstp;
	if (sigaction(SIGTSTP, &sa, NULL) == -1) {
		printf("F: sigaction failed\n");
	}
	sa.sa_handler = &ttin;
	if (sigaction(SIGTTIN, &sa, NULL) == -1) {
		printf("F: sigaction failed\n");
	}
	sa.sa_handler = &ttou;
	if (sigaction(SIGTTOU, &sa, NULL) == -1) {
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
			printf("F: wait failed\n");
		} else {
			printf("F: wait success: child_pid: %i exit_status: %i\n", ret, status);
		}
	}
	else {
		printf("C: I am the child, I will Interrupt my father in 2 s\n");
		sleep(2);
		printf("C: Multiple Interrupted !\n");
		kill(pid, SIGUSR2);
		kill(pid, SIGTSTP);
		kill(pid, SIGTTOU);
		sleep(2);
		printf("C: I return()\n");
	}
	return 0;
}
