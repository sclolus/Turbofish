#include <stdio.h>
#include <unistd.h>
#include <signal.h>

void hello_signal(int signum) {
	printf("Signal Received 5/5: %i\n", signum);
}

int main() {
	printf("initialise Signals test\n");

	printf("hello_signal: %p\n", hello_signal);
	pid_t pid = getpid();
	printf("pid of process '%u'\n", pid);

	// 10 is the number of SIGUSR
	int ret = (int)signal(10, hello_signal);
	printf("signal function return: %i\n", ret);

	printf("KILLING\n");
	ret = kill(pid, 10);
	if (ret == -1) {
		printf("kill failed\n");
	}
	printf("after kill\n");
	ret = sleep(2);
	if (ret != 0) {
		printf("Ny sleep was interrupted !!! I remain %d seconds ...", ret);
	}

	printf("after sleep\n");
	return 0;
}
