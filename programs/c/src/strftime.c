
#include <time.h>
#include <stdio.h>
#include <stdlib.h>

extern char *tzname[2];

int main(int argc, char *argv[])
{
	(void)argc;
	char outstr[200];
	time_t t;
	struct tm *tmp;

	printf("%s:%s\n", tzname[0], tzname[1]);
	t = time(NULL);
	tmp = localtime(&t);
	if (tmp == NULL) {
		perror("localtime");
		exit(EXIT_FAILURE);
	}

	if (strftime(outstr, sizeof(outstr), argv[1], tmp) == 0) {
		fprintf(stderr, "strftime returned 0");
		exit(EXIT_FAILURE);
	}

	printf("Result string is \"%s\"\n", outstr);
	exit(EXIT_SUCCESS);
}
