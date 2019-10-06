#ifndef __STROPTS_H__
# define __STROPTS_H__

# define TIOCGWINSZ         0x5413
# define RAW_SCANCODE_MODE  0x1
# define REFRESH_SCREEN     0x2

int ioctl(int fildes, int request, ... /* arg */);

#endif /* __STROPTS_H__ */
