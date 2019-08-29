#include <sys/wait.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>

int main(int argc, char *argv[], char *envp[])
{
	pid_t pid = fork();
	if (pid < 0) {
		// printf("%s: Fork failed\n", __func__);
		exit(1);
	} else if (pid == 0) {
		int fd = open("/dev/tty1", 0);
		dup(fd);
		dup(fd);
		if (argc != 2) {
			printf("Bad argument number %i: should be 2\n", argc);
			while (1) {}
		}
		setpgid(0, 0);
		tcsetpgrp(fd, getpgid(0));
		printf("argc: %i -> self: %s to_execve: %s\n", argc, argv[0], argv[1]);
		int ret = execve(argv[1], argv + 1, envp);
		if (ret < 0) {
			printf("%s: Execve failed\n", __func__);
			exit(1);
		}
	}

	pid = fork();
	if (pid < 0) {
		// printf("%s: Fork failed\n", __func__);
		exit(1);
	} else if (pid == 0) {
		int fd = open("/dev/tty2", 0);
		dup(fd);
		dup(fd);
		if (argc != 2) {
			printf("Bad argument number %i: should be 2\n", argc);
			while (1) {}
		}
		setpgid(0, 0);
		tcsetpgrp(fd, getpgid(0));
		printf("argc: %i -> self: %s to_execve: %s\n", argc, argv[0], argv[1]);
		int ret = execve(argv[1], argv + 1, envp);
		if (ret < 0) {
			printf("%s: Execve failed\n", __func__);
			exit(1);
		}
	} else {
		int status;

		while (1) {
			pid_t ret = wait(&status);
			if (ret < 0) {
				printf("%s: Wait failed\n", __func__);
				exit(1);
			}
			printf("Deleting zombie: pid = %i status = %i\n", ret, status);
		}
	}
	return 0;
}
