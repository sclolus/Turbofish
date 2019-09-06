#ifndef __UN_H__
# define __UN_H__

# include <stdint.h>
# include <limits.h>

/*
 * Unix socket sockaddr interface (AF_UNIX)
 */
struct sockaddr_un {
	u16 sun_family;
	u8 sun_path[PATH_MAX];
};

#endif
