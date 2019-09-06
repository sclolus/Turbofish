#ifndef __UN_H__
# define __UN_H__


# define UNIX_PATHNAME_MAXSIZE 108
# include <stdint.h>

/*
 * Unix socket sockaddr interface (AF_UNIX)
 */
struct sockaddr_un {
	u16 sun_family;
	u8 sun_path[UNIX_PATHNAME_MAXSIZE];
};

#endif
