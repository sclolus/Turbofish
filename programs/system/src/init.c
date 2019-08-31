#include <sys/wait.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>

int open_tty_device(const char *tty_device)
{
	int fd = open(tty_device, 0);
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
		exit(1);
	} else if (pid == 0) {
		int fd = open_tty_device(tty_device);
		if (argc != 2) {
			dprintf(2, "bad argument number %i: should be 2\n", argc);
			exit(1);
		}
		if (setpgid(0, 0) < 0) {
			perror("setpgid failed");
			exit(1);
		}
		if (tcsetpgrp(fd, getpgid(0)) < 0) {
			perror("tcsetpgrp failed");
			exit(1);
		}
		printf("argc: %i -> self: %s to_execve: %s to_tty: %s\n", argc, argv[0], argv[1], tty_device);
		int ret = execve(argv[1], argv + 1, envp);
		if (ret < 0) {
			perror("execve failed");
			exit(1);
		}

	}
	return pid;
}

int main(int argc, char *argv[], char *envp[])
{
	/* #[allow(unused)] */
	pid_t _p1 = init_forker("/dev/tty1", argc, argv, envp);
	pid_t _p2 = init_forker("/dev/tty2", argc, argv, envp);
	pid_t _p3 = init_forker("/dev/tty3", argc, argv, envp);
	pid_t _p4 = init_forker("/dev/tty4", argc, argv, envp);

	(void)_p1;
	(void)_p2;
	(void)_p3;
	(void)_p4;

	int status;

	while (1) {
		pid_t ret = wait(&status);
		if (ret < 0) {
			/* #[allow(unused)] */
			int _fd = open_tty_device("/dev/tty1");
			(void)_fd;
			perror("init wait failed");
			exit(1);
		}
	}
	return 0;
}
