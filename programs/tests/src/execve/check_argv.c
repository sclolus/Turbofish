#include <stdlib.h>
#include <stdio.h>
#include <string.h>

char* ARGV[] = {
	"argv1",
	"argv2",
	"argv3",
};

char* ENVP[] = {
	"env1=1",
	"env2=2",
	"env3=3",
};

int main(int argc, char *argv[], char *envp[]) {
	if (argc != 3 || strcmp(argv[0], ARGV[0]) || strcmp(argv[1], ARGV[1]) || strcmp(argv[2], ARGV[2]) || argv[3] != NULL) {
		printf("argc%d\n%s\n%s\n%s\n", argc, argv[0], argv[1], argv[2]);
		printf("bad argv\n");
			return 1;
	}
	if (strcmp(envp[0], ENVP[0]) || strcmp(envp[1], ENVP[1]) || strcmp(envp[2], ENVP[2]) || envp[3] != NULL) {
		printf("bad envp\n");
			return 1;
	}
	return 0;
}
