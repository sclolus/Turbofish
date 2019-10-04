#include <ltrace.h>
#include <time.h>
#include <stdint.h>
#include <assert.h>
#include <stdlib.h>
#include <stdio.h>

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

/// URN VALUE

/// Upon successful completion, the localtime() function shall return
/// a pointer to the broken-down time structure. If an error is
/// detected, localtime() shall return a null pointer [CX] [Option
/// Start] and set errno to indicate the error.

#warning missing tests

static struct tm TM;

#define SECSPERDAY 86400 /* (3600 * 24) */
#define SECSPERYEAR SECSPERDAY * 365

static time_t	get_month_from_yday(time_t yday)
{
	TRACE
	assert(yday < 367);
	const time_t nbr_of_days[12] = {
		31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
	};

	for (uint32_t i = 0; i < 12; i++) {
		if (yday < nbr_of_days[i])
			return i;
		yday -= nbr_of_days[i];
	}
	return (time_t)13; // there no 13 month, except in fiscal laws.
}

/// We could make those two functions generic and stuff.
static time_t	get_day_of_month_from_yday(time_t yday)
{
	TRACE
	assert(yday < 367);
	const time_t nbr_of_days[12] = {
		31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,
	};

	for (uint32_t i = 0; i < 12; i++) {
		if (yday < nbr_of_days[i])
			return yday;
		yday -= nbr_of_days[i];
	}
	return (time_t)32; // there no 32nth day to a month, except in my childhood's MidSummer Night's Dreams.
}

// This algorithm calculates the day of the week based on those three parameters.
// The `year` parameter is the based from 0 BC. (That is something like 2019 or 1970).
// The `month` parameter confirming to the struct tm format ranges from 0 to 11.
static time_t	zeller_congruence(time_t day_of_month, time_t month, time_t year)
{
	TRACE
	month++;
	if (month == 1)
	{
		// This algorithm relys on January being equal to 13.
		month = 13;
		year--;
	} else if (month == 2) {
		// This algorithm relys on February being equal to 14.
		month = 14;
		year--;
	}

	int q = day_of_month;
	int m = month;
	int k = year % 100;
	int j = year / 100;

	int h = q + 13 * (m + 1) / 5 + k + k / 4 + j / 4 + 5 * j;

	// The current value of h is the day of the week, but Saturday = 0, Sunday = 1.
	h = h % 7;

	/* Converts to tm format: Sunday = 0. */
	if (h == 0) {
		h = 7;
	}

	h -= 1;
	return h;

}

struct tm *localtime(const time_t *timer)
{
	TRACE
	time_t t = *timer;

	time_t years_from_bc =  t / (3600 * 24 * 365) + 1970;
	time_t leap_days_in_seconds =
		/* Compute the number of leapdays between 1970 and `years_from_bc`
		  (exclusive).  There is a leapday every 4th years ...  */
		(((years_from_bc - 1) / 4 - 1970 / 4)
		/* ... except every 100th years_from_bc ... */
		- ((years_from_bc - 1) / 100 - 1970 / 100)
		/* ... but still every 400th years_from_bc.  */
		 + ((years_from_bc - 1) / 400 - 1970 / 400)) * SECSPERDAY;


	time_t secs  =  t % 60;
	time_t mins  = (t / 60) % 60;
	time_t hours = (t / 3600) % 24;
	time_t years = (t - leap_days_in_seconds) / (SECSPERYEAR);
	time_t yday  = ((t - (years) * SECSPERYEAR - leap_days_in_seconds) / SECSPERDAY) % 365;

	time_t month = get_month_from_yday(yday);
	time_t day_of_month = get_day_of_month_from_yday(yday);
	time_t day_of_week = zeller_congruence(day_of_month, month, years_from_bc);

	/* use zeller's congruence to get the day of the week */;
	/* time_t day_of_week = */

	/* printf("years: %u\nyday: %u\nday_of_month: %d\nmonth: %d\n", years, yday, (int)day_of_month, (int)month); */

	TM.tm_sec   = (int)secs; // Seconds [0,60].
	TM.tm_min   = (int)mins; // Minutes [0,59].
	TM.tm_hour  = (int)hours; // Hour [0,23].
	TM.tm_mday  = (int)day_of_month; // Day of month [1,31].
	TM.tm_mon   = (int)month; // Month of year [0,11].
	TM.tm_year  = (int)(years + 70); // Years since 1900.
	TM.tm_wday  = (int)day_of_week; // Day of week [0,6] (Sunday =0).
	TM.tm_yday  = (int)yday; // Day of year [0,365].
	TM.tm_isdst = (int)0; // Daylight Savings flag.
	return &TM;
}

#undef SECSPERDAY
#undef SECSPERYEAR
