#include <signal.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <wait.h>

void cont(int signum) {
	printf("Resuming program %i: sleep 1 second\n", signum);
	sleep(1);
}

pid_t father_id = 0;

int main(void)
{
	father_id = getpid();
	signal(SIGCONT, &cont);
	int f = fork();
	if (f < 0) {
		printf("Fork Failed\n");
		exit(1);
	} else if (f == 0) {
		printf("I am the child\n");
		sleep(1);
		printf("I stop my father...\n");
		kill(father_id, SIGSTOP);
		printf("Father stoped for 3 seconds\n");
		sleep(3);
		kill(father_id, SIGCONT);
		sleep(1);
	} else {
		int i = 0;
		while (1) {
			printf("I am a dummy father: %i\n", i++);
		}
	}
	return 0;
}
