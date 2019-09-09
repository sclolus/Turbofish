#ifndef __UTIME_H__
# define __UTIME_H__

#include <sys/types.h>
/// The <utime.h> header shall declare the utimbuf structure, which
/// shall include the following members:
///
///
struct utimbuf {
	time_t actime; // Access time.
	time_t modtime; // Modification time.
};
///
/// The times shall be measured in seconds since the Epoch.
///
/// The <utime.h> header shall define the time_t type as described in
/// <sys/types.h>.
///
/// The following shall be declared as a function and may also be
/// defined as a macro. A function prototype shall be provided.
///
int utime(const char *, const struct utimbuf *);

#endif
