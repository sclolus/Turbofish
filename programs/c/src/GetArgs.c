#include <stdio.h>

int main(int argc, char *argv[], char *envp[])
{
	printf("argc: %i\n", argc);
	printf("listing argv:\n");
	int i = 0;
	while (argv[i] != 0x0) {
		printf("argv %i: %s\n", i, argv[i]);
		i++;
	}
	printf("listing envp:\n");
	i = 0;
	while (envp[i] != 0x0) {
		printf("envp %i: %s\n", i, envp[i]);
		i++;
	}
	return 0;
}
