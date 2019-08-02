
#include <unistd.h>
#include <stdio.h>
#include <signal.h>
#include <stdlib.h>

void death(int signum)
{
	(void)signum;
	printf("aaaaaaaaaaaaarrrrrrrrrrrggggggggggggggg.......\n");
	exit(0);
}

int main(void)
{
	signal(SIGINT, &death);

	pid_t pid = fork();

	if (pid == -1) {
		printf("fork failed\n");
		exit(1);
	}
	if (pid == 0) {
		printf("I am on the rails, I love train, i sleep, and i wake up in 3 seconds !\n");
		sleep(1);
		printf("I am on the rails, I love train, i sleep, and i wake up in 2 seconds !\n");
		sleep(1);
		printf("I am on the rails, I love train, i sleep, and i wake up in 1 seconds !\n");
		sleep(1);
		printf("I am on the rails, I love train and i am not dead ! hahaha !\n");
		exit(0);
	} else {
		printf("I will kill my son in 1 second !\n");
		sleep(1);
		kill(pid, SIGINT);
		printf("I ended waiting, my son %i is dead, i am happy and sadic. niark !\n", pid);
	}
	return 0;
}
