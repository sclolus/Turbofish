#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include "tools.h"

size_t NB_FILE = 42;

int main() {
	char filename[100];

	srand16(0x42);

	pid_t pid = getpid();

	for (size_t i = 0; i < NB_FILE; i++) {
		sprintf(filename, "./file_%d_%lu", pid, i);

		printf("creating file: %s\n", filename);
		int fd = open(filename, O_CREAT, 0644);
		if (fd == -1) {
			perror("open");
			exit(1);
		}
	}

	for (size_t i = 0; i < NB_FILE; i++) {
		sprintf(filename, "./file_%d_%lu", pid, i);
		printf("testing file existatnce: %s\n", filename);
		int ret = access(filename, F_OK);
		if (ret == -1) {
			perror("access");
			exit(1);
		}
	}

	for (size_t i = 0; i < NB_FILE; i++) {
		sprintf(filename, "./file_%d_%lu", pid, i);
		printf("deleting file: %s\n", filename);
		int ret = unlink(filename);
		if (ret == -1) {
			perror("unlink");
			exit(1);
		}
	}

	for (size_t i = 0; i < NB_FILE; i++) {
		sprintf(filename, "./file_%d_%lu", pid, i);
		printf("testing file existatnce: %s\n", filename);
		int ret = access(filename, F_OK);
		if (ret != -1) {
			printf("file should be deleted now %s\n", filename);
			exit(1);
		}
	}
}
