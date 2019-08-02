#include <signal.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <wait.h>

void usr2(int signum) {
	printf("B: usr2 signal handler %i\n", signum);
	raise(SIGTTOU);
	printf("E: usr2 signal handler %i\n", signum);
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

pid_t father_id = 0;

int main(void)
{
	father_id = getpid();
	signal(SIGUSR2, &usr2);
	signal(SIGTSTP, &tstp);
	signal(SIGTTIN, &ttin);
	signal(SIGTTOU, &ttou);
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
	}
	return 0;
}
