#include <ltrace.h>
#include <sys/time.h>
#include <custom.h>

#warning DUMMY IMPLEMENTATION

int settimeofday(const struct timeval *tv, const struct timezone *tz) {
	DUMMY
	(void)tv;
	(void)tz;
	return 0;
}
