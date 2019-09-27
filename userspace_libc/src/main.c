#include "libc.h"

#include <stdio.h>

int main(void)
{
	xprintf("Les %i carrotes sont cuites\n", 42);

	int i;
	int j;
	int k;
	char s[] = "bananes";

	char *fmt;
	char buf_s[512];
	char buf_p[512];	

	fmt = "%i    %i\t\t\t%i%3s";
	int a = 42;
	int b = 84;
	int c = 168;
	char s2[512];
	sprintf(buf_p, fmt, a, b, c, s);
	sscanf(buf_p, fmt, &i, &j, &k, s2);
	sprintf(buf_s, fmt, i, j, k, s2);
	printf("origin: '%s'\n", buf_p);
	printf("final:  '%s'\n", buf_s);
	return 0;
}
