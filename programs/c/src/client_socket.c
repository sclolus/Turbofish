#define SOCKET_NAME "/server.socket"
#define BUFFER_SIZE 12

/*
 * File client.c
 */

#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/socket.h>
#include <sys/un.h>
#include <unistd.h>

int
main(int argc, char *argv[])
{
	struct sockaddr_un addr;
	int i;
	int ret;
	int data_socket;
	char buffer[BUFFER_SIZE];

	/* Create local socket. */

	data_socket = socket(AF_UNIX, SOCK_SEQPACKET, 0);
	if (data_socket == -1) {
		perror("socket");
		exit(EXIT_FAILURE);
	}

	/*
	 * For portability clear the whole structure, since some
	 * implementations have additional (nonstandard) fields in
	 * the structure.
	 */

	memset(&addr, 0, sizeof(struct sockaddr_un));

	/* Connect socket to socket address */

	addr.sun_family = AF_UNIX;
	strncpy((char *)addr.sun_path, SOCKET_NAME, sizeof(addr.sun_path) - 1);

	ret = connect (data_socket, (const struct sockaddr *) &addr,
				   sizeof(struct sockaddr_un));
	if (ret == -1) {
		fprintf(stderr, "The server is down.\n");
		exit(EXIT_FAILURE);
	}

	/* Send arguments. */

	for (i = 1; i < argc; ++i) {
		ret = write(data_socket, argv[i], strlen(argv[i]) + 1);
		if (ret == -1) {
			perror("write");
			break;
		}
	}

	/* Request result. */

	strcpy (buffer, "END");
	ret = write(data_socket, buffer, strlen(buffer) + 1);
	if (ret == -1) {
		perror("write");
		exit(EXIT_FAILURE);
	}

	/* Receive result. */

	ret = read(data_socket, buffer, BUFFER_SIZE);
	if (ret == -1) {
		perror("read");
		exit(EXIT_FAILURE);
	}

	/* Ensure buffer is 0-terminated. */

	buffer[BUFFER_SIZE - 1] = 0;

	printf("Result = %s\n", buffer);

	/* Close socket. */

	close(data_socket);

	exit(EXIT_SUCCESS);
}
