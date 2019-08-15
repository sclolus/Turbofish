
#include <string.h>
#include <stdio.h>

int increment(char *s, int i)
{
	size_t size = 42;

	i += 3;
	return (int)size + strlen(s) + i;
}

int transfert_char(char *s)
{
	int a = 5;

	s += 1;
	a += 1;
	return increment(s, a);
}

int main(void)
{
	char *s = NULL;

	printf("This program will be do a segfault on strlen\n");
	return transfert_char(s);
}
