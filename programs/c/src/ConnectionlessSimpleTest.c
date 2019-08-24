
#include <unistd.h>
#include <stdio.h>
#include <errno.h>
#include <stdlib.h>
#include <wait.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <string.h>

#define PATH "banane"

int main(void)
{
	pid_t pid = fork();
	if (pid == -1) {
		printf("fork failed\n");
		exit(1);
	}
	if (pid == 0) {
		printf("I'm the child\n");
		int fd = socket(AF_UNIX, SOCK_DGRAM, 0);
		printf("C: a: %i b: %i\n", AF_UNIX, SOCK_DGRAM);
		printf("C: new file discriptor '%i'\n", fd);
		if (fd < 0) {
			printf("fail to get fd\n");
			exit(errno);
		}
		sleep(1);
		printf("C: Transmiting message...\n");
		struct sockaddr_un dest_addr;
		dest_addr.sun_family = AF_UNIX;
		strcpy((char *)&dest_addr.sun_path, PATH);
		/*
		 * Send a message to the father
		 */
		ssize_t ret = sendto(fd, "banane banane banane", 20, 0, (const struct sockaddr *)&dest_addr, sizeof(dest_addr));
		(void)ret;
		close(fd);
		exit(0);
	} else {
		printf("I'm a father\n");
		int fd = socket(AF_UNIX, SOCK_DGRAM, 0);
		printf("a: %i b: %i\n", AF_UNIX, SOCK_DGRAM);
		printf("F: new file discriptor '%i'\n", fd);
		if (fd < 0) {
			printf("fail to get fd\n");
			exit(errno);
		}
		struct sockaddr_un addr;

		addr.sun_family = AF_UNIX;
		printf("size: %zu\n", sizeof(addr.sun_path));
		strcpy((char *)&addr.sun_path, PATH);

		/*
		 * This function create a file descriptor
		 */
		int ret = bind(fd, (const struct sockaddr *)&addr, sizeof(addr));
		if (ret < 0) {
			printf("fail to bind\n");
			exit(errno);
		}
		char buf[128];

		struct sockaddr_un input;
		size_t len;

		printf("F: Waiting for transmission\n");
		/*
		 * Wait for a child message
		 */
		ssize_t n = recvfrom(fd, buf, 128, 0, (struct sockaddr *)&input, &len);
		printf("F: Received: %s ret: %zi\n", buf, n);
		close(fd);
		/*
		 * Unlink the file
		 */
		unlink(PATH);
		int stat_loc;
		pid_t child_pid = wait(&stat_loc);
		if (child_pid == -1) {
			printf("wait failed\n");
			exit(1);
		}
		printf("I ended my way: '%u'\n", child_pid);
	}
	return 0;
}

// execve("./ConnectionlessSimpleTest", ["./ConnectionlessSimpleTest"], [/* 39 vars */]) = 0
// strace: [ Process PID=13373 runs in 32 bit mode. ]
// fork()                                  = 13374
// I'm the child
// C: a: 1 b: 2
// C: new file discriptor '3'
// write(1, "I'm a father\n", 13I'm a father
// )          = 13
// socket(AF_UNIX, SOCK_DGRAM, 0)          = 3
// write(1, "a: 1 b: 2\n", 10a: 1 b: 2
// )             = 10
// write(1, "F: new file discriptor '3'\n", 27F: new file discriptor '3'
// ) = 27
// write(1, "size: 108\n", 10size: 108
// )             = 10
// bind(3, {sa_family=AF_UNIX, sun_path="banane"}, 110) = 0
// write(1, "F: Waiting for transmission\n", 28F: Waiting for transmission
// ) = 28
// recvfrom(3, C: Transmiting message...
// "banane\0", 128, 0, 0xffabf30e, [0]) = 7
// --- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=13374, si_uid=1000, si_status=0, si_utime=0, si_stime=0} ---
// write(1, "F: Received: banane ret: 7\n", 27F: Received: banane ret: 7
// ) = 27
// close(3)                                = 0
// unlink("banane")                        = 0
// waitpid(-1, [{WIFEXITED(s) && WEXITSTATUS(s) == 0}], 0) = 13374
// write(1, "I ended my way: '13374'\n", 24I ended my way: '13374'
// ) = 24
// exit(0)                                 = ?
// +++ exited with 0 +++
