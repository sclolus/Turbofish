
#include <unistd.h>
#include <stdio.h>
#include <signal.h>
#include <errno.h>
#include <stdlib.h>

void signal_handler(int signum)
{
	(void)signum;
	printf("%s: I was killed by my father...\n", __FILE__);
}

int main(void)
{
	signal(SIGSEGV, &signal_handler);

	pid_t pid = fork();

	if (pid == -1) {
		printf("fork failed\n");
		exit(1);
	}
	if (pid == 0) {
		int result = pause();

		printf("I ended sigpause !\n");
		if (result != -1) {
			printf("result of pause should be -1");
			exit(1);
		}
		if (errno != EINTR) {
			printf("errno should be EINTR");
			exit(1);
		}
	} else {
		printf("%s: I will kill my son in 1 second !\n", __FILE__);
		sleep(1);
		kill(pid, SIGSEGV);
		sleep(1);
	}
	return 0;
}
