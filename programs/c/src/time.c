#include <time.h>
#include <unistd.h>
#include <stdlib.h>
#include <stdio.h>

int main(int argc, char **argv)
{
	(void)argc;
	(void)argv;

	time_t	t;

	while (42) {
		if ((time_t)-1 == time(&t)) {
			dprintf(STDERR_FILENO, "time() failed\n");
			return EXIT_FAILURE;
		}
		/* résultat : le 18/2/1983 à 21:39:05	 */
		/* printf("Expected: %s\n", "18/2/1983 à 21:39:05"); */
		/* t = 414452345; */
		printf("Current unix time: %u\n", t);
		printf("Current Unix Date: %s\n", ctime(&t));
		sleep(1);
	}
}
