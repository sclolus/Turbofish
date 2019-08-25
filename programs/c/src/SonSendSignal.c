#include <stdio.h>
#include <unistd.h>
#include <signal.h>
#include <errno.h>
#include <wait.h>

void hello_signal(int signum) {
	printf("Signal Received 5/5: %i\n", signum);
}

pid_t father_pid = 0;

int main() {
	printf("initialise SonSendSignal test\n");

	father_pid = getpid();
	printf("pid of father is '%u'\n", father_pid);

	struct sigaction sig = {0};
	sig.sa_handler = &hello_signal;
	sigaction(SIGTSTP, &sig, NULL);

	// signal(SIGTSTP, &hello_signal); signal set by defaut SA_RESTART TODO: Implement it in libc
	printf("signal initialized\n");

	int ret = fork();

	if (ret < 0) {
		printf("Fork Failed !\n");;
	} else if (ret == 0) {
		printf("I am the child, il will send a signal to my waiting father\n");
		sleep(1);
		kill(father_pid, SIGTSTP);
		sleep(6);
	} else {
		printf("I am the father and i wait my child '%u'\n", ret);
		int status;
		wait(&status);
		printf("I ended wainting... errno is: %i, status: %.8x\n", -errno, status);
	}
	return(-errno);
}
