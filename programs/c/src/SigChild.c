#include <signal.h>
#include <unistd.h>
#include <errno.h>
#include <stdio.h>

void sigchild_handler(int signo) {
	printf("sigchild\n");
}

int main() {
	signal(SIGCHLD, sigchild_handler);
	int child_pid = fork();
	if(child_pid == 0) {
		sleep(1);

		exit(333);
	}
	else {
		int status;
		int ret = wait(&status);
		printf("%d\n", ret == EINTR);

		printf("after wait %d\n", status);
	}
	return 0;
}
