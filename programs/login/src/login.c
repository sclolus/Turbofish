#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdint.h>
#include <errno.h>
#include <sys/wait.h>
#include <string.h>

# define BIN_NAME "login"

# define err_errno(format, ...) do {						\
		dprintf(2, BIN_NAME ": " format ": %s\n" __VA_OPT__(,) __VA_ARGS__, strerror(errno)); \
		exit(EXIT_FAILURE);					\
	} while (0);

int worker(char **envp)
{
	char	*login = NULL;
	size_t	curr_size = 0;

	printf("\nLogin: ");
	if (-1 == ft_getline(&login, &curr_size, stdin)) {
		err_errno("I/O error");
	}

	if (curr_size) {
		login[curr_size - 1] = '\0';
		printf("'%s'\n", login);
	}

	char	*bin = "/bin/su";
	execve(bin, (char*[]){bin, "-l", "-s", "/bin/dash", login, NULL}, envp);
	perror("Failed to execute su");
	exit(EXIT_FAILURE);
}

int main(int argc, char **argv, char **envp)
{
	(void)argc;
	(void)argv;

	pid_t	pid;

	while (42) {
		if (-1 == (pid = fork())) {
			err_errno("Failed to fork");
		} else if (pid == 0) {
			worker(envp);
			exit(EXIT_FAILURE);
		} else {
			int status;

			if (-1 == wait(&status)) {
				err_errno("Failed to wait worker");
			}
		}
	}
}
