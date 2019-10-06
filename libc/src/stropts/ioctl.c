#include <ltrace.h>
#include <stropts.h>
#include <errno.h>
#include <stdio.h>
#include <custom.h>
#include <user_syscall.h>

/* DESCRIPTION */
/* The ioctl() function shall perform a variety of control functions
 * on STREAMS devices. For non-STREAMS devices, the functions
 * performed by this call are unspecified. The request argument and an
 * optional third argument (with varying type) shall be passed to and
 * interpreted  by the appropriate part of the STREAM associated with fildes. */

/* The fildes argument is an open file descriptor that refers to a device. */

/* The request argument selects the control function to be performed
 * and shall depend on the STREAMS device being addressed. */

/* The arg argument represents additional information that is needed
 * by this specific STREAMS device to perform the requested
 * function. The type of arg depends upon the particular control
 * request, but it shall be either an integer or a pointer to a
 * device-specific data structure.  */

/* The ioctl() commands applicable to STREAMS, their arguments, and
 * error conditions that apply to each individual command are
 * described below.  */
int ioctl(int fildes, int request, ... /* arg */)
{
	TRACE
	va_list ap;
	void *arg = 0;

	TRACE
	DUMMY
	va_start(ap, request);
	switch (request) {
		case TIOCGWINSZ:
			arg = va_arg(ap, struct winsize*);
			break;
		case RAW_SCANCODE_MODE:
			arg = va_arg(ap, int);
			break;
	}
	int ret = _user_syscall(IOCTL, 3, fildes, request, arg);
	va_end(ap);
	set_errno_and_return(ret);
}
