#include <stdio.h>

/* ssize_t  getline(char **line, size_t *n, FILE *stream) */
int  getline(char **line, size_t *n, FILE *stream)
{
	return getdelim(line, n, '\n', stream);
}
