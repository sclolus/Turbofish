#include <signal.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

int main() {
	sigset_t mask, oldmask;

	sigemptyset(&mask);
	sigaddset(&mask, SIGINT);
	int res = sigprocmask(SIG_SETMASK, &mask, &oldmask);
	if (res == -1) {
		perror("sigprocmask failed");
		return -1;
	}
	sigaddset(&oldmask, SIGINT);
	// this kill should not be call because we blocked sigint
	kill(getpid(), SIGINT);
	exit(0);
}
