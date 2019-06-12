
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <wait.h>

int main(void)
{
	pid_t pid = fork();
	if (pid == -1) {
		printf("fork failed\n");
		exit(1);
	}
	if (pid == 0) {
		printf("I am on the rails, I love train and i sleep now for 3 seconds !\n");
		sleep(3);
		exit(0);
	} else {
		int stat_loc;
		printf("I'm a father waiting it's child\n");
		sleep(1);
		pid_t child_pid = wait(&stat_loc);
		//pid_t child_pid = waitpid(pid2, &stat_loc, 0);
		if (child_pid == -1) {
			printf("wait failed\n");
			exit(1);
		}
		printf("I ended waiting, my son %i is dead, but i am happy for him\n", child_pid);
	}
	return 0;
}
