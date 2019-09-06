#include <stdio.h>

char    *fgets(char *restrict s, int n, FILE *restrict stream)
{
	if (n == 0) { // I guess this is not an error.
		return s;
	}

	if (feof(stream)) {
		return NULL;
	}

	int	c;
	int	len = 0;

	while (len < n - 1 && EOF != (c = fgetc(stream))) {
		char	read_char = (char)c;

		s[len] = read_char;

		len++;
		if (read_char == '\n')
			break;
	}

	s[len] = '\0';

	if (ferror(stream)) {
		return NULL;
	}
	return s;
}


#ifdef UNIT_TESTS
# include <criterion/criterion.h>

#endif
