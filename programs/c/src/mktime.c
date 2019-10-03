
#include <time.h>
#include <stdio.h>

int main(void)
{
	// Mercredi 2 octobre 2019
	struct tm t = {0};
	t.tm_mday = 2;
	t.tm_mon = 9;
	t.tm_year = 119;
	t.tm_wday = 3;
	t.tm_yday = 274;
	t.tm_isdst = 0;

	time_t res = mktime(&t);
	printf("Result of mktime: %i\n", res);

	return 0;
}
