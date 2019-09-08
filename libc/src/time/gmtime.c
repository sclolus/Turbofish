#include <ltrace.h>
#include <time.h>
#include <custom.h>

static struct tm TM;

#warning DUMMY IMPLEMENTATION

struct tm *gmtime(const time_t *timep)
{
	TRACE
	DUMMY
	(void)timep;
	TM.tm_sec   = 0; // Seconds [0,60].
	TM.tm_min   = 0; // Minutes [0,59].
	TM.tm_hour  = 0; // Hour [0,23].
	TM.tm_mday  = 1; // Day of month [1,31].
	TM.tm_mon   = 0; // Month of year [0,11].
	TM.tm_year  = 0; // Years since 1900.
	TM.tm_wday  = 0; // Day of week [0,6] (Sunday =0).
	TM.tm_yday  = 0; // Day of year [0,365].
	TM.tm_isdst = 0; // Daylight Savings flag.
	return &TM;
}
