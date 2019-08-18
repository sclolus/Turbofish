#include <signal.h>
#include <stdlib.h>
#include <stdio.h>
#include <unistd.h>

int main() {
	sigset_t mask, oldmask;

	printf("prout\n");
	sigemptyset(&mask);
	sigaddset(&mask, SIGINT);
	int res = sigprocmask(SIG_SETMASK, &mask, &oldmask);
	printf("prout\n");
	if (res == -1) {
		perror("sigprocmask failed");
		return -1;
	}

	sigaddset(&oldmask, SIGINT);
	sigset_t newmask;

	res = sigprocmask(0, NULL, &newmask);
	if (res == -1) {
		perror("sigprocmask failed");
		return -1;
	}
	if (oldmask != mask) {
		dprintf(2, "sigprocmask SIG_SETMASK doesn't work");
		exit(1);
	}
	printf("prout\n");

	// this kill should not be call because we blocked sigint
	kill(getpid(), SIGINT);
	exit(0);
}
