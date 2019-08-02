
#include <stdio.h>
#include <stdlib.h>

extern char **environ;

int main(int ac, char **av, char **envp) {
	printf("environ variable: %llu\n", (size_t)environ);
	printf("envp variable: %llu\n", (size_t)envp);
	if (envp != environ) {
		exit(1);
	}

	printf("environ variable: %llu\n", (size_t)environ);
	for (int i = 0; environ[i] != NULL; i++) {
		printf("%s\n", environ[i]);
	}
	return 0;
}
