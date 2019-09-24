#include <stropts.h>
#include <errno.h>

#warning DUMMY IMPLEMENTATION of ioctl

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
	errno = ENOSYS;
	return - 1;
}
