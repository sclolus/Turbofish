#include <stdlib.h>
#include <stdio.h>

int main(int argc, char **argv) {
	if (argc != 2) {
		printf("usage: getenv var\n");
		exit(1);
	}
	char *value = getenv(argv[1]);
	if (value != NULL) {
		printf("env var %s is %s\n", argv[1], value);
	} else {
		printf("var %s is null\n", argv[1]);
	}
}
