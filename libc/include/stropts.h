#ifndef __STROPTS_H__
# define __STROPTS_H__

# define TIOCGWINSZ           0x5413

# define RAW_SCANCODE_MODE    0x1
# define GET_FRAME_BUFFER_PTR 0x3

#include <string.h>

# define REFRESH_SCREEN       0x2

// This local buffer is used to share graphics betweeen user space and TTYs
struct local_buffer {
	unsigned char *buf;
	size_t len;
	size_t bpp;
};

int ioctl(int fildes, int request, ... /* arg */);

#endif /* __STROPTS_H__ */
