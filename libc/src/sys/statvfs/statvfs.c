#include <sys/statfs.h>
#include <sys/statvfs.h>

int statvfs(const char *restrict path, struct statvfs *restrict buf)
{
	struct statfs	statfs_buf;

	int ret = statfs(path, &statfs_buf);

	if (-1 == ret) {
		return -1;
	}

	buf->f_bsize   = statfs_buf.f_bsize;
	buf->f_frsize  = statfs_buf.f_frsize;
	buf->f_blocks  = statfs_buf.f_blocks;
	buf->f_bfree   = statfs_buf.f_bfree;
	buf->f_bavail  = statfs_buf.f_bavail;
	buf->f_files   = statfs_buf.f_files;
	buf->f_ffree   = statfs_buf.f_ffree;
	buf->f_favail  = statfs_buf.f_ffree; // Supposedly what we want.
	buf->f_fsid    = statfs_buf.f_fsid;
	buf->f_flag    = statfs_buf.f_flags;
	buf->f_namemax = statfs_buf.f_namelen;
	return 0;
}
