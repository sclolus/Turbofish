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

void cont(int signum) {
	printf("Resuming program %i: sleep 1 second\n", signum);
	sleep(1);
}

pid_t father_id = 0;

int main(void)
{
	father_id = getpid();
	signal(SIGCONT, &cont);
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
		sleep(2);
		printf("I stop my father...\n");
		kill(father_id, SIGSTOP);
		printf("Father stoped for 3 seconds. Overload it\n");
                kill(father_id, SIGUSR2);
		kill(father_id, SIGTSTP);
		sleep(3);
		kill(father_id, SIGCONT);
		sleep(1);
	} else {
		int i = 0;
		while (1) {
			printf("I am a dummy father: %i\n", i++);
			sleep(1);
		}
	}
	return 0;
}
