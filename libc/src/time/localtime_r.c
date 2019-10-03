#include <ltrace.h>
#include <time.h>
#include <string.h>

/*
 * The localtime() function converts the calendar time timep to broken-down
 * time representation
 */
struct tm *localtime_r(const time_t *timer, struct tm *result)
{
	TRACE
	struct tm *static_time = localtime(timer);

	memcpy(result, static_time, sizeof(struct tm));
	/*
	 * On success, localtime_r() return the address
	 * of the structure pointed to by result.
	 */
	return result;
}
