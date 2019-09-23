#include <sys/un.h>
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <stdlib.h>
#include <signal.h>
#include <assert.h>
#include <sys/wait.h>
#include <sys/socket.h>

/************************************************************/
/* This is a stream socket server sample program for UNIX   */
/* domain sockets. This program listens for a connection    */
/* from a client program, accepts it, reads data from the   */
/* client, then sends data back to connected UNIX socket.   */
/************************************************************/

char SOCK_PATH[100];
char CLIENT_PATH[100];

#define DATA_CLIENT_TO_SERVER "Hello from client"
#define DATA_SERVER_TO_CLIENT "Hello from server"

void usr1(int signum) {
	(void)signum;
}

int server(pid_t child_pid){

    int server_sock, client_sock, rc;
	socklen_t len;
    int bytes_rec = 0;
    struct sockaddr_un server_sockaddr;
    struct sockaddr_un client_sockaddr;
    char buf[256];
    int backlog = 10;
    memset(&server_sockaddr, 0, sizeof(struct sockaddr_un));
    memset(&client_sockaddr, 0, sizeof(struct sockaddr_un));
    memset(buf, 0, 256);

    /**************************************/
    /* Create a UNIX domain stream socket */
    /**************************************/
    server_sock = socket(AF_UNIX, SOCK_STREAM, 0);
    if (server_sock == -1){
        perror("SOCKET ERROR");
        exit(1);
    }

    /***************************************/
    /* Set up the UNIX sockaddr structure  */
    /* by using AF_UNIX for the family and */
    /* giving it a filepath to bind to.    */
    /*                                     */
    /* Unlink the file so the bind will    */
    /* succeed, then bind to that file.    */
    /***************************************/
    server_sockaddr.sun_family = AF_UNIX;
    strcpy((char *)server_sockaddr.sun_path, SOCK_PATH);
    len = sizeof(server_sockaddr);

    unlink(SOCK_PATH);
    printf("binding server\n");
    rc = bind(server_sock, (struct sockaddr *) &server_sockaddr, len);
    if (rc == -1){

        perror("BIND ERROR");
        close(server_sock);
        exit(1);
    }

    /*********************************/
    /* Listen for any client sockets */
    /*********************************/
    printf("socket listening...\n");
    rc = listen(server_sock, backlog);
    if (rc == -1){
        perror("LISTEN ERROR");
        close(server_sock);
        exit(1);
    }

    printf("sending signal \n");
	/* 
	 * if (kill(getpid(), SIGKILL) == -1) {
	 * 	perror("kill");
	 * }
	 */
	if (kill(child_pid, SIGUSR1) == -1) {
		perror("kill");
	}

    printf("accepting connections\n");
    /*********************************/
    /* Accept an incoming connection */
    /*********************************/
    client_sock = accept(server_sock, (struct sockaddr *) &client_sockaddr, &len);
    if (client_sock == -1){
        perror("ACCEPT ERROR");
        close(server_sock);
        close(client_sock);
        exit(1);
    }

	printf("Client socket filepath: %s\n", client_sockaddr.sun_path);

    /************************************/
    /* Read and print the data          */
    /* incoming on the connected socket */
    /************************************/
    printf("waiting to read...\n");
    bytes_rec = recv(client_sock, buf, sizeof(buf), 0);
    if (bytes_rec == -1){
        perror("RECV ERROR");
        close(server_sock);
        close(client_sock);
        exit(1);
    }
    else {
        printf("DATA RECEIVED FROM CLIENT = %s\n", buf);
    }

	assert(bytes_rec == sizeof(DATA_CLIENT_TO_SERVER) - 1);
	assert(strcmp(buf, DATA_CLIENT_TO_SERVER) == 0);

    /******************************************/
    /* Send data back to the connected socket */
    /******************************************/
    memset(buf, 0, 256);
    strcpy(buf, DATA_SERVER_TO_CLIENT);
    printf("Sending data...\n");
    rc = send(client_sock, buf, strlen(buf), 0);
    if (rc == -1) {
        perror("SEND ERROR");
        close(server_sock);
        close(client_sock);
        exit(1);
    }
    else {
        printf("Data sent!\n");
    }

    /******************************/
    /* Close the sockets and exit */
    /******************************/
    close(server_sock);
    close(client_sock);

    return 0;
}

int client(void){

    int client_sock, rc, len;
    struct sockaddr_un server_sockaddr;
    struct sockaddr_un client_sockaddr;
    char buf[256];
    memset(&server_sockaddr, 0, sizeof(struct sockaddr_un));
    memset(&client_sockaddr, 0, sizeof(struct sockaddr_un));

    /**************************************/
    /* Create a UNIX domain stream socket */
    /**************************************/
    client_sock = socket(AF_UNIX, SOCK_STREAM, 0);
    if (client_sock == -1) {
        perror("SOCKET ERROR");
        exit(1);
    }

    /***************************************/
    /* Set up the UNIX sockaddr structure  */
    /* by using AF_UNIX for the family and */
    /* giving it a filepath to bind to.    */
    /*                                     */
    /* Unlink the file so the bind will    */
    /* succeed, then bind to that file.    */
    /***************************************/
    client_sockaddr.sun_family = AF_UNIX;
    strcpy((char *)client_sockaddr.sun_path, CLIENT_PATH);
    len = sizeof(client_sockaddr);

    unlink(CLIENT_PATH);
	printf("binding client\n");
    rc = bind(client_sock, (struct sockaddr *) &client_sockaddr, len);
    if (rc == -1){
        perror("BIND ERROR");
        close(client_sock);
        exit(1);
    }
	assert(unlink(CLIENT_PATH) == 0);

    /***************************************/
    /* Set up the UNIX sockaddr structure  */
    /* for the server socket and connect   */
    /* to it.                              */
    /***************************************/
    server_sockaddr.sun_family = AF_UNIX;
    strcpy((char *)server_sockaddr.sun_path, SOCK_PATH);
	
	signal(SIGUSR1, &usr1);
	pause();

	printf("connecting client\n");
    rc = connect(client_sock, (struct sockaddr *) &server_sockaddr, len);
    if(rc == -1){
        perror("CONNECT ERROR");
        close(client_sock);
        exit(1);
    }

    /************************************/
    /* Copy the data to the buffer and  */
    /* send it to the server socket.    */
    /************************************/
    strcpy(buf, DATA_CLIENT_TO_SERVER);
    printf("Sending data...\n");
    rc = send(client_sock, buf, strlen(buf), 0);
    printf("end Sending data...\n");
    if (rc == -1) {
        perror("SEND ERROR");
        close(client_sock);
        exit(1);
    }
    else {
        printf("Data sent!\n");
    }

    /**************************************/
    /* Read the data sent from the server */
    /* and print it.                      */
    /**************************************/
    printf("Waiting to recieve data...\n");
    memset(buf, 0, sizeof(buf));
    rc = recv(client_sock, buf, sizeof(buf), 0);
    if (rc == -1) {
        perror("RECV ERROR");
        close(client_sock);
        exit(1);
    }
    else {
        printf("DATA RECEIVED FROM SERVER = %s\n", buf);
    }

	assert(rc == sizeof(DATA_SERVER_TO_CLIENT) - 1);
	assert(strcmp(buf, DATA_SERVER_TO_CLIENT) == 0);

    /******************************/
    /* Close the socket and exit. */
    /******************************/
    close(client_sock);

    return 0;
}

int main() {

	pid_t pid = getpid();
	sprintf(SOCK_PATH, "tpf_unix_sock.server_%d", pid);
	sprintf(CLIENT_PATH, "tpf_unix_sock.client_%d", pid);

	int child_pid = fork();
	if (child_pid == -1) {
		perror("fork");
		exit(1);
	} else if (child_pid == 0) {
		client();
		exit(0);
	} else {
		printf("%d\n", child_pid);
		sleep(1);
		server(child_pid);
		int status;
		int ret = wait(&status);
		assert(unlink(SOCK_PATH) == 0);
		if (ret == -1) {
			exit(1);
		}
		return 0;
	}
}
