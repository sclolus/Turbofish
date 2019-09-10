#include <unistd.h>
#include <stdio.h>

int main() {
	while (1) {
		pid_t pid = fork();
		if (pid == 0) {
			printf("child born with pid: %d\n", getpid());
		}
	}
}
