#ifndef __SYS_SYSMACROS_H__
# define __SYS_SYSMACROS_H__

# warning all of those macros are probably dummy.

#define major(dev) (dev >> 8)
#define minor(dev) (dev & 0xFF)
#define makedev(maj, min) ((maj << 8) | (min & 0xFF))

#define gnu_dev_minor(dev) (minor(dev))
#define gnu_dev_major(dev) (major(dev))
#define gnu_dev_makedev(maj, min) (makedev(maj, min))

#endif /* __SYS_SYSMACROS_H__ */
