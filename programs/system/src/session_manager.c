#include <sys/wait.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include <stdbool.h>

int open_tty_device(const char *tty_device)
{
	int fd = open(tty_device, O_RDWR);
	if (fd < 0) {
		exit(1);
	}
	dup(fd);
	dup(fd);
	return fd;
}

pid_t init_forker(const char *tty_device, int argc, char *argv[], char *envp[])
{
	pid_t pid = fork();
	if (pid < 0) {
		/* #[allow(unused)] */
		int _fd = open_tty_device(tty_device);
		(void)_fd;
		perror("fork failed");
		return -1;
	} else if (pid == 0) {
		int fd = open_tty_device(tty_device);

		if (argc < 2) {
			dprintf(2, "bad argument number %i: should be at least 2\n", argc);
			return -1;
		}
		if (setpgid(0, 0) < 0) {
			perror("setpgid failed");
			return -1;
		}
		if (tcsetpgrp(fd, getpgid(0)) < 0) {
			perror("tcsetpgrp failed");
			return -1;
		}
		printf("argc: %i -> self: %s to_execve: %s to_tty: %s\n", argc, argv[0], argv[1], tty_device);
		char *shell = getenv("SHELL");
		int ret;
		if (shell != NULL) {
		 ret = execve(shell, argv + 1, envp);
		} else {
			ret = execve(argv[1], argv + 1, envp);
		}
		if (ret < 0) {
			perror("execve failed");
			dprintf(STDERR_FILENO, "seeking for '%s'\n", argv[1]);
			return -1;
		}
	}
	return pid;
}

#define MAX_TTY 4
#define BUF_LEN 42

int main(int argc, char **argv, char **envp)
{
	char buf[BUF_LEN];
	pid_t p[MAX_TTY];

	// Create all the process
	for (int i = 0; i < MAX_TTY; i++) {
		snprintf(buf, BUF_LEN, "/dev/tty%i", i + 1);
		p[i] = init_forker(buf, argc, argv, envp);
		if ((p[i]) < 0) {
			dprintf(STDERR_FILENO, "CRITICAL ERROR DETECTED !\n");
			while (1) {}
		}
	}

	int status;

	// In case of child exit, resurect him
	while (true) {
		pid_t ret = wait(&status);
		if (ret < 0) {
			int _fd = open_tty_device("/dev/tty1");

			(void)_fd;
			perror("session manager wait failed");
			while (1) {}
		}
		for (int i = 0; i < MAX_TTY; i++) {
			if (p[i] == ret) {
				snprintf(buf, BUF_LEN, "/dev/tty%i", i + 1);
				p[i] = init_forker(buf, argc, argv, envp);
				break;
			}
		}

	}
	return 0;
}
