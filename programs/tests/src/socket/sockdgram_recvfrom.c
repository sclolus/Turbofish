#include <sys/un.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <assert.h>
#include <sys/wait.h>
#include <sys/stat.h>
#include <sys/socket.h>
#include <string.h>

char CLIENT_PATH[100];
char SENDER_PATH[100];

struct sockaddr_un SENDER_ADDR;

void fill_sockaddr(struct sockaddr_un *addr) {
	memset(addr, 0, sizeof(struct sockaddr_un));
	addr->sun_family = AF_UNIX;
	strcpy((char *)addr->sun_path, CLIENT_PATH);
}

char MESSAGE[] = "hello world!";

void assert_same_file(char *path1, char *path2) {
	struct stat buf1;
	struct stat buf2;

	printf("path1: %s, path2: %s\n", path1, path2);
	int stat_res1 = stat(path1, &buf1);
	int stat_res2 = stat(path2, &buf2);
	assert(stat_res1 == 0);
	assert(stat_res2 == 0);

	assert(buf1.st_dev == buf2.st_dev );
	assert(buf1.st_ino == buf2.st_ino );
	assert(buf1.st_mode == buf2.st_mode );
	assert(buf1.st_nlink == buf2.st_nlink );
	assert(buf1.st_uid == buf2.st_uid );
	assert(buf1.st_gid == buf2.st_gid );
	assert(buf1.st_rdev == buf2.st_rdev );
	assert(buf1.st_size == buf2.st_size );
	assert(buf1.st_blksize== buf2.st_blksize);
	assert(buf1.st_blocks == buf2.st_blocks );

}

void child() {
	struct sockaddr_un dest_addr;
	fill_sockaddr(&dest_addr);

	int sock = socket(AF_UNIX, SOCK_DGRAM, 0);
	if (sock == -1) {
		perror("socket");
		exit(1);
	}
	int ret = bind(sock, (const struct sockaddr *)&SENDER_ADDR, sizeof(struct sockaddr_un));
	if (ret == -1) {
		perror("bind");
		exit(1);
	}

	sleep(2);
	printf("message send: %s\n", MESSAGE);
	ssize_t len_send = sendto(sock, MESSAGE, sizeof(MESSAGE), 0, (const struct sockaddr *)&dest_addr, sizeof(struct sockaddr_un));
	printf("len send: %ld\n", len_send);
	if (len_send == -1) {
		perror("sendto");
		exit(1);
	}
	close(sock);
}

void father(int sock) {
	char buffer[100];
	struct sockaddr_un sender_addr;
	socklen_t len;

	memset(&sender_addr, 0, sizeof(struct sockaddr_un));

    ssize_t len_received = recvfrom(sock, buffer, 100, 0, (struct sockaddr *)&sender_addr, &len);
	printf("len received: %ld from: '%s'\n", len_received, sender_addr.sun_path);
	if (len_received == -1) {
		perror("recv");
		exit(1);
	}
	printf("message received: %s\n", buffer);
	assert(strcmp(buffer, MESSAGE) == 0);
	printf("SENDER_PATH: %s\n", SENDER_PATH);
	assert_same_file((char *)sender_addr.sun_path, SENDER_PATH);
	/* assert(memcmp(sender_addr, ); */
	assert(unlink(CLIENT_PATH) == 0);
}

int main() {

	struct sockaddr_un addr;

	pid_t pid = getpid();
	sprintf(CLIENT_PATH, "./socket_file_%d", pid);
	fill_sockaddr(&addr);

	sprintf(SENDER_PATH, "./socket_file_sender_%d", pid);
	memset(&SENDER_ADDR, 0, sizeof(struct sockaddr_un));
	SENDER_ADDR.sun_family = AF_UNIX;
	strcpy((char *)SENDER_ADDR.sun_path, SENDER_PATH);

	int sock = socket(AF_UNIX, SOCK_DGRAM, 0);
	if (sock == -1) {
		perror("socket");
		exit(1);
	}
	int ret = bind(sock, (const struct sockaddr *)&addr, sizeof(struct sockaddr_un));
	if (ret == -1) {
		perror("bind");
		exit(1);
	}
	int child_pid = fork();
	if (child_pid == -1) {
		perror("fork");
		exit(1);
	} else if (child_pid == 0) {
		close(sock);
		child();
		exit(0);
	} else {
		father(sock);
		int status;
		int ret = wait(&status);
		if (ret == -1) {
			exit(1);
		}
		return 0;
	}
}
