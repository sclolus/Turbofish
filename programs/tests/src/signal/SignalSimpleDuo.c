#include <signal.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <wait.h>
#include <stdbool.h>

bool SIGUSR2_HANDLER_CALLED = false;
bool SIGSTP_HANDLER_CALLED = false;

void usr2(int signum) {
	printf("usr2 signal handler %i\n", signum);
	SIGUSR2_HANDLER_CALLED = true;
}

void tstp(int signum) {
	printf("tstp signal handler %i\n", signum);
	SIGSTP_HANDLER_CALLED = true;
}

pid_t father_id = 0;

int main(void)
{
	father_id = getpid();
	signal(SIGUSR2, &usr2);
	signal(SIGTSTP, &tstp);
	int f = fork();
	if (f < 0) {
		printf("Fork Failed\n");
		exit(1);
	} else if (f == 0) {
		printf("I am the child\n");
		kill(father_id, SIGUSR2);
		kill(father_id, SIGTSTP);
		sleep(1);
	} else {
		printf("I am the father, i wait my child\n");
		int id;
		int ret = wait(&id);
		printf("wait status %i\n", ret);
		if (!SIGSTP_HANDLER_CALLED || !SIGUSR2_HANDLER_CALLED) {
			exit(1);
		} else {
			exit(0);
		}
	}
	return 0;
}
