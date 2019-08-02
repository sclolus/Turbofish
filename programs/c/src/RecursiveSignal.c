#include <stdio.h>
#include <unistd.h>
#include <signal.h>

void hello_signal(int signum) {
	printf("Signal Received 5/5: %i\n", signum);
	pid_t pid = getpid();
	int ret = kill(pid, 10);
	if (ret == -1) {
		printf("kill failed\n");
	}
	ret = sleep(1);
	printf("end signal\n");
}

int main() {
	printf("initialise Signals test\n");

	printf("hello_signal: %p\n", hello_signal);
	pid_t pid = getpid();
	printf("pid of process '%u'\n", pid);

	// 10 is the number of SIGUSR
	struct sigaction sa;

	sa.sa_handler = hello_signal;
	sa.sa_flags = SA_RESTART;
	/* sa.sa_flags = SA_NODEFER; /\* Restart functions if */
                                 /* interrupted by handler *\/ */
    if (sigaction(10, &sa, NULL) == -1) {
		printf("sigaction failed\n");
	}
        /* Handle error */;

	printf("KILLING\n");
	int ret = kill(pid, 10);
	if (ret == -1) {
		printf("kill failed\n");
	}
	printf("after kill\n");
	ret = sleep(10);
	if (ret != 0) {
		printf("Ny sleep was interrupted !!! I remain %d seconds ...", ret);
	}
	while (42) {}

	return 0;
}
