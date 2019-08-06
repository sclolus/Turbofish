
#include <stdio.h>
#include <stdlib.h>

extern char **environ;

int main(int ac, char **av, char **envp) {
	(void)ac;
	(void)av;
	printf("environ variable: %zu\n", (size_t)environ);
	printf("envp variable: %zu\n", (size_t)envp);
	if (envp != environ) {
		exit(1);
	}

	printf("environ variable: %zu\n", (size_t)environ);
	for (int i = 0; environ[i] != NULL; i++) {
		printf("%s\n", environ[i]);
	}
	return 0;
}
