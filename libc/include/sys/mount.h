#ifndef __SYS_MOUNT_H__
#define __SYS_MOUNT_H__

int mount(const char *source, const char *target,
		  const char *filesystemtype, unsigned long mountflags,
		  const void *data);
int umount(const char *target);

#endif
