#include <stdio.h>

#warning getline (stdio.h) is currently prefixed by _ft

/*
 * ssize_t  getline(char **line, size_t *n, FILE *stream)
 */
int  ft_getline(char **line, size_t *n, FILE *stream)
{
	return getdelim(line, n, '\n', stream);
}
