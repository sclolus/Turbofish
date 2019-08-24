
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
		int fd = socket(AF_UNIX, SOCK_STREAM, 0);
		printf("C: a: %i b: %i\n", AF_UNIX, SOCK_STREAM);
		printf("C: new file discriptor '%i'\n", fd);
		if (fd < 0) {
			printf("fail to get fd\n");
			exit(errno);
		}
		sleep(1);
		struct sockaddr_un dest_addr;
		dest_addr.sun_family = AF_UNIX;
		strcpy((char *)&dest_addr.sun_path, PATH);
		printf("C: Try to connect...\n");
		/*
		 * Connect to the server
		 */
		int ret = connect(fd, (const struct sockaddr *)&dest_addr, sizeof(dest_addr));
		if (ret < 0) {
			printf("C: fail to connect\n");
			exit(errno);
		}
		/*
		 * Sending a message to the server
		 */
		char buf[128];
		strcpy(buf, "Hello World");
		ret = send(fd, buf, 128, 0);
		if (ret == -1) {
			printf("C: Cannot send a message\n");
			exit(errno);
		}
		/*
		 * Receaving a message from the server
		 */
		ret = recv(fd, buf, 128, 0);
		if (ret == -1) {
			printf("C: Cannot recv a message\n");
			exit(errno);
		}
		printf("C: Message recu: %s\n", buf);
		/*
		 * Closing socket
		 */
		close(fd);
		exit(0);
	} else {
		printf("I'm a father\n");
		int fd = socket(AF_UNIX, SOCK_STREAM, 0);
		printf("F: a: %i b: %i\n", AF_UNIX, SOCK_STREAM);
		printf("F: new file discriptor '%i'\n", fd);
		if (fd < 0) {
			printf("F: fail to get fd\n");
			exit(errno);
		}
		struct sockaddr_un addr;

		addr.sun_family = AF_UNIX;
		printf("F: size: %zu\n", sizeof(addr.sun_path));
		strcpy((char *)&addr.sun_path, PATH);

		/*
		 * This function create a file descriptor
		 */
		int ret = bind(fd, (const struct sockaddr *)&addr, sizeof(addr));
		if (ret < 0) {
			printf("F: fail to bind\n");
			exit(errno);
		}

		/*
		 * Listen for new connections
		 */
		if (listen(fd, 16) == -1) {
			printf("F: fail to listen\n");
			exit(errno);
		}

		struct sockaddr_un peer_addr;
		size_t peer_addr_size = sizeof(struct sockaddr_un);
		/*
		 * Wait for connexion
		 */
		int peer_fd = accept(fd, (struct sockaddr *)&peer_addr, &peer_addr_size);
		if (peer_fd == -1) {
			printf("F: Cannot accept a connextion\n");
			exit(errno);
		}
		printf("F: accepting connexion: fd: %i\n", peer_fd);

		char buf[128];
		/*
		 * Wait to receave a message
		 */
		printf("F: Waiting for message\n");
		ret = recv(peer_fd, buf, 128, 0);
		if (ret == -1) {
			printf("F: Cannot receave a message\n");
			exit(errno);
		}
		printf("F: message receave: %s\n", buf);
		if (strlen(buf) == 0) {
			printf("F: maybe the peer close the connexion\n");
		}
		strcpy(buf, "Les carrotes sont cuites");
		/*
		 * Send a response
		 */
		printf("F: sending a response...\n");
		ret = send(peer_fd, buf, 128, 0);
		if (ret == -1) {
			printf("F: Cannot send a message\n");
			exit(errno);
		}
		/*
		 * Try to shutdown the connexion
		 */
		printf("F: Preparing shutdown\n");
		ret = shutdown(peer_fd, 0);
		if (ret == -1) {
			printf("F: Cannot shutdown peer communication\n");
			exit(errno);
		}
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

// execve("./ConnectionOrientedSimpleTest", ["./ConnectionOrientedSimpleTest"], [/* 39 vars */]) = 0
// strace: [ Process PID=15068 runs in 32 bit mode. ]
// fork()                                  = 15069
// I'm the child
// C: a: 1 b: 1
// C: new file discriptor '3'
// write(1, "I'm a father\n", 13I'm a father
// )          = 13
// socket(AF_UNIX, SOCK_STREAM, 0)         = 3
// write(1, "F: a: 1 b: 1\n", 13F: a: 1 b: 1
// )          = 13
// write(1, "F: new file discriptor '3'\n", 27F: new file discriptor '3'
// ) = 27
// write(1, "F: size: 108\n", 13F: size: 108
// )          = 13
// bind(3, {sa_family=AF_UNIX, sun_path="banane"}, 110) = 0
// listen(3, 16)                           = 0
// accept(3, C: Try to connect...
// {sa_family=AF_UNIX}, [110->2]) = 4
// write(1, "F: accepting connexion: fd: 4\n", 30F: accepting connexion: fd: 4
// ) = 30
// write(1, "F: Waiting for message\n", 23F: Waiting for message
// ) = 23
// recv(4, "Hello World\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0"..., 128, 0) = 128
// write(1, "F: message receave: Hello World\n", 32F: message receave: Hello World
// ) = 32
// write(1, "F: sending a response...\n", 25F: sending a response...
// ) = 25
// send(4, "Les carrotes sont cuites\0\0\0\0\0\0\0\0"..., 128, 0) = 128
// C: Message recu: Les carrotes sont cuites
// write(1, "F: Preparing shutdown\n", 22F: Preparing shutdown
// ) = 22
// --- SIGCHLD {si_signo=SIGCHLD, si_code=CLD_EXITED, si_pid=15069, si_uid=1000, si_status=0, si_utime=0, si_stime=0} ---
// shutdown(4, SHUT_RD)                    = 0
// close(3)                                = 0
// unlink("banane")                        = 0
// waitpid(-1, [{WIFEXITED(s) && WEXITSTATUS(s) == 0}], 0) = 15069
// write(1, "I ended my way: '15069'\n", 24I ended my way: '15069'
// ) = 24
// exit(0)                                 = ?
// +++ exited with 0 +++
