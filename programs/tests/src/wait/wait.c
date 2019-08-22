#include <signal.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>
#include <wait.h>
#include <stdbool.h>

int main(void)
{
	printf("waitpid -1\n");
	int f = fork();
	if (f < 0) {
		perror("fork failed");
		exit(1);
	} else if (f == 0) {
		exit(0);
	} else {
		int res = waitpid(-1, NULL, 0);
		if (res == -1 || res != f) {
			dprintf(2, "waitpid -1 failed");
			exit(1);
		}
	}
	printf("waitpid -pgid\n");
	int res = setpgid(0, 0);
	if (res == -1) {
		perror("setpgid failed");
		exit(1);
	}
	gid_t pgid = getpgrp();
	f = fork();
	if (f < 0) {
		perror("fork failed");
		exit(1);
	} else if (f == 0) {
		exit(0);
	} else {
		int res = waitpid(-pgid, NULL, 0);
		if (res == -1 || res != f) {
			dprintf(2, "waitpid -pgid failed");
			exit(1);
		}
	}

	printf("waitpid pid\n");
	f = fork();
	if (f < 0) {
		perror("fork failed");
		exit(1);
	} else if (f == 0) {
		exit(0);
	} else {
		int res = waitpid(f, NULL, 0);
		if (res == -1 || res != f) {
			dprintf(2, "waitpid pid failed");
			exit(1);
		}
	}
}
