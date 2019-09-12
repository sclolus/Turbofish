#ifndef __LTRACE_H__
# define __LTRACE_H__

#include <stdio.h>

#ifdef LTRACE 
# define TRACE dprintf(2, "%s called\n", __FUNCTION__);
#else
# define TRACE
#endif

#endif
