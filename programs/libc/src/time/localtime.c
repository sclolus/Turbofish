#include <time.h>

/// For localtime(): [CX] [Option Start] The functionality described
/// on this reference page is aligned with the ISO C standard. Any
/// conflict between the requirements described here and the ISO C
/// standard is unintentional. This volume of POSIX.1-2017 defers to
/// the ISO C standard. [Option End]

/// The localtime() function shall convert the time in seconds since
/// the Epoch pointed to by timer into a broken-down time, expressed
/// as a local time. The function corrects for the timezone and any
/// seasonal time adjustments. [CX] [Option Start] Local timezone
/// information is used as though localtime() calls tzset().

/// The relationship between a time in seconds since the Epoch used as
/// an argument to localtime() and the tm structure (defined in the
/// <time.h> header) is that the result shall be as specified in the
/// expression given in the definition of seconds since the Epoch (see
/// XBD Seconds Since the Epoch) corrected for timezone and any
/// seasonal time adjustments, where the names in the structure and in
/// the expression correspond.

/// The same relationship shall apply for localtime_r().

/// The localtime() function need not be thread-safe.

/// The asctime(), ctime(), gmtime(), and localtime() functions shall
/// return values in one of two static objects: a broken-down time
/// structure and an array of type char. Execution of any of the
/// functions may overwrite the information returned in either of
/// these objects by any of the other functions.

/// The localtime_r() function shall convert the time in seconds since
/// the Epoch pointed to by timer into a broken-down time stored in
/// the structure to which result points. The localtime_r() function
/// shall also return a pointer to that same structure.

/// Unlike localtime(), the localtime_r() function is not required to
/// set tzname. If localtime_r() sets tzname, it shall also set
/// daylight and timezone. If localtime_r() does not set tzname, it
/// shall not set daylight and shall not set timezone. [Option End]

///URN VALUE

/// Upon successful completion, the localtime() function shall return
/// a pointer to the broken-down time structure. If an error is
/// detected, localtime() shall return a null pointer [CX] [Option
/// Start] and set errno to indicate the error.

#warning DUMMY IMPLEMENTATION

static struct tm TM;

struct tm *localtime(const time_t *timer)
{
	(void)timer;
	TM.tm_sec   = 0;// Seconds [0,60].
	TM.tm_min   = 0;// Minutes [0,59].
	TM.tm_hour  = 0;// Hour [0,23].
	TM.tm_mday  = 1;// Day of month [1,31].
	TM.tm_mon   = 0;// Month of year [0,11].
	TM.tm_year  = 0;// Years since 1900.
	TM.tm_wday  = 0;// Day of week [0,6] (Sunday =0).
	TM.tm_yday  = 0;// Day of year [0,365].
	TM.tm_isdst = 0;// Daylight Savings flag.
	return &TM;
}
