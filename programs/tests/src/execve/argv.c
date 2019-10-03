#include <stdio.h>
#include <wait.h>
#include <unistd.h>
#include <stdlib.h>

char* ARGV[] = {
	"argv1",
	"argv2",
	"argv3",
	NULL,
};

char* ENVP[] = {
	"env1=1",
	"env2=2",
	"env3=3",
	NULL,
};

int main()
{
	pid_t pid = fork();
	if (pid < 0) {
		perror("fork failed");
		exit(1);
	} else if (pid == 0) {
		int res = execve("/bin/DeepTests/execve/check_argv", ARGV, ENVP);
		if (res == -1) {
			perror("execve failed");
			exit(1);
		}
	} else {
		int status;
		int res = wait(&status);
		if (res == -1) {
			perror("wait check_argv failed");
			exit(1);
		}
		if (WIFEXITED(status) && WEXITSTATUS(status) == 0) {
			exit(0);
		}
		exit(1);
	}
	return 0;
}
