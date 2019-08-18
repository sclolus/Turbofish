#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <wait.h>

int main(void)
{
	pid_t father_pid = getpid();
	printf("aaaa\n");
	printf("father_pid: %d\n", father_pid);
	printf("bbb\n");
	// create a new process group
	if (setpgid(0, 0) == -1) {
		perror("setpgid failed");
		exit(1);
	}
	int f = fork();

	if (f < 0) {
		printf("Fork Failed\n");
		exit(1);
	} else if (f == 0) {
		printf("I am the child\n");
		pid_t pgid = getpgrp();
		if (pgid == -1) {
			perror("getprgp failed");
			exit(1);
		}
		pid_t pid = getpid();
		if (pid == -1) {
			perror("getpid failed");
			exit(1);
		}
		if (father_pid != pgid) {
			dprintf(2, "child is not in it's father process group: father_pid: %d pgid: %d\n", father_pid, pgid);
			exit(1);
		}
		exit(0);
	} else {
		int id;
		wait(&id);
	}
	return 0;
}
