#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>

int COUNTER = 0;

void incr_counter(void)
{
	COUNTER++;
}

void exit_counter(void)
{
	// this should exit 31
	_exit(COUNTER);
}

void child()
{
	// add exit counter
	atexit(exit_counter);
	// add 31 times incr counter
	for (int i = 0; i < 31; i++) {
		if (atexit(incr_counter) != 0) {
			_exit(1);
		}
	}
	exit(0);
}

int main()
{
	int pid = fork();
	if (pid == -1) {
			exit(1);
	}
	else if (pid == 0) {
		child();
	}
	else {
		int status;
		int ret = wait(&status);
		if (ret == -1) {
			exit(1);
		}
		if (WIFEXITED(status) && WEXITSTATUS(status) == 31) {
			exit(0);
		}
		exit(1);
	}
}
