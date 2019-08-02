
#include <unistd.h>
#include <stdio.h>
#include <signal.h>
#include <errno.h>
#include <stdlib.h>

void signal_handler(int signum)
{
	(void)signum;
	printf("aaaaaaaaaarrrrrrrrrrrggggggggggggggg.......\n");
}

int main(void)
{
	signal(SIGINT, &signal_handler);

	pid_t pid = fork();

	if (pid == -1) {
		printf("fork failed\n");
		exit(1);
	}
	if (pid == 0) {
		int result;
		/* 
		 * sigset_t oldmask;
		 * sigset_t sigmask;
		 * 
		 * sigprocmask(SIG_SETMASK, NULL, &oldmask);
		 * sigmask = oldmask;
		 * sigaddset(&sigmask, SIGUSR1);
		 */
		sigset_t sigmask;

		sigemptyset(&sigmask);

		result = sigsuspend(&sigmask);

		printf("I ended sigsuspend!\n");
		if (result != -1) {
			printf("result of sigsuspend should be -1");
		}
		if (errno != EINTR) {
			printf("errno should be EINTR");
			exit(1);
		}
	} else {
		printf("I will kill my son in 1 second !\n");
		sleep(1);
		kill(pid, SIGINT);
		sleep(1);
	}
	return 0;
}
