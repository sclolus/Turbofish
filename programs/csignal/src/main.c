#include <stdio.h>
#include <unistd.h>
#include <signal.h>

void hello_signal(int signum) {
	printf("Signal Received 5/5: %i\n", signum);
}

int main() {
	printf("initialise Signals test\n");

	printf("hello_signal: %x\n", hello_signal);
	pid_t pid = getpid();
	printf("pid of process '%u'\n", pid);

	// 10 is the number of SIGUSR
	int ret = (int)signal(10, (int)hello_signal);
	printf("signal function return: %i\n", ret);

	printf("KILLING");
	ret = kill(pid, 10);
	if (ret == -1) {
		printf("kill failed\n");
	}
	printf("after kill\n");

	while (42) {}

	return 0;
}
