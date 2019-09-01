#include <stdio.h>

ssize_t  getline(char **restrict line, size_t *restrict n, FILE *restrict stream)
{
	return getdelim(line, n, '\n', stream);
}
