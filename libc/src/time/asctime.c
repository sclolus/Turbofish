#include <ltrace.h>
#include <time.h>
#include <stdio.h>

#warning missing tests

/// This is the proposed posix implementation of asctime.
/// The asctime() function shall convert the broken-down time in the structure pointed to by timeptr into a string in the form:
///
/// "Sun Sep 16 01:03:52 1973\n\0"

char *asctime(const struct tm *timeptr)
{
	TRACE
    static char wday_name[7][3] = {
        "Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"
    };
    static char mon_name[12][3] = {
        "Jan", "Feb", "Mar", "Apr", "May", "Jun",
        "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"
    };

    // 3 + 1 + 3 + 10 + 1 + 10 + 1 + 10 + 1 + 10 + 1 + 10 + 1 + 1 (null byte) = 63
    static char result[63];

    sprintf(result, "%.3s %.3s%3d %.2d:%.2d:%.2d %d\n",
        wday_name[timeptr->tm_wday],
        mon_name[timeptr->tm_mon],
        timeptr->tm_mday, timeptr->tm_hour,
        timeptr->tm_min, timeptr->tm_sec,
        1900 + timeptr->tm_year);
    return result;
}
