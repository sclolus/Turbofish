#include <sys/un.h>
#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <assert.h>
#include <sys/wait.h>
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

void child() {
	struct sockaddr_un dest_addr;
	fill_sockaddr(&dest_addr);

	pid_t pid = getpid();
	sprintf(SENDER_PATH, "./socket_file_sender_%d", pid);
	memset(&SENDER_ADDR, 0, sizeof(struct sockaddr_un));
	SENDER_ADDR.sun_family = AF_UNIX;
	strcpy((char *)SENDER_ADDR.sun_path, SENDER_PATH);

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

	sleep(1);
    ssize_t len_received = recvfrom(sock, buffer, 100, 0, (struct sockaddr *)&sender_addr, &len);
	printf("len received: %ld from: '%s'\n", len_received, sender_addr.sun_path);
	if (len_received == -1) {
		perror("recv");
		exit(1);
	}
	printf("message received: %s\n", buffer);
	assert(strcmp(buffer, MESSAGE) == 0);
	/* assert(memcmp(sender_addr, ); */
	assert(unlink(CLIENT_PATH) == 0);
}

int main() {

	struct sockaddr_un addr;

	pid_t pid = getpid();
	sprintf(CLIENT_PATH, "./socket_file_%d", pid);
	fill_sockaddr(&addr);
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
