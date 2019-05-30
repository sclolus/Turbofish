
#include "../libc/include/unistd.h"
#include "../libc/include/stdio.h"
#include "../libc/include/errno.h"

int main(void)
{
	pid_t pid = fork();
	if (pid == -1) {
		printf("fork failed\n");
		exit(1);
	}
	if (pid == 0) {
		printf("I love train\n");
		exit(0);
	} else {
		int stat_loc;
		printf("I'm a father waiting it's child\n");
		pid_t child_pid = wait(&stat_loc);
		if (child_pid == -1) {
			printf("wait failed\n");
			exit(1);
		}
		printf("I ended waiting\n");
	}
	return 0;
}
