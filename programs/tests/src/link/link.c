#include <unistd.h>
#include <limits.h>
#include <stdio.h>
#include <fcntl.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <assert.h>

void debug_stat(struct stat *buf) {
	printf("%d %d %d %d %d %d %d %d %d %d %d %d %d %d\n",
		   buf->st_dev,          // Device ID of device containing file.
		   buf->st_ino,          // File serial number.
		   buf->st_mode,         // Mode of file (see below).
		   buf->st_nlink,        // Number of hard links to the file.
		   buf->st_uid,            // User ID of file.
		   buf->st_gid,            // Group ID of file.
		   buf->st_rdev,           // Device ID (if file is character or block special).
		   buf->st_size,           // For regular files, the file size in bytes.
		   buf->st_atim, // Last data access timestamp.
		   buf->st_mtim, // Last data modification timestamp.
		   buf->st_ctim, // Last file status change timestamp.
		   buf->st_blksize,    // A file system-specific preferred I/O block size
		   buf->st_blocks      // Number of blocks allocated for this object.
		);
}

int main() {
	char filename[100];
	char linkpath[100];

	pid_t pid = getpid();
	sprintf(filename, "./file_%d", pid);

	printf("creating file: %s\n", filename);
	int fd = open(filename, O_RDWR | O_CREAT, 0644);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	sprintf(linkpath, "./file_%d_link_path", pid);
	int ret = link(filename, linkpath);
	if (ret == -1) {
		perror("link");
		exit(1);
	}

	struct stat buf1;
	struct stat buf2;
	int stat_res1 = stat(filename, &buf1);
	int stat_res2 = stat(linkpath, &buf2);
	debug_stat(&buf1);
	debug_stat(&buf2);
	assert(stat_res1 == 0);
	assert(stat_res2 == 0);

	assert(buf1.st_dev == buf2.st_dev );
	assert(buf1.st_ino == buf2.st_ino );
	assert(buf1.st_mode == buf2.st_mode );
	assert(buf1.st_nlink == buf2.st_nlink );
	assert(buf1.st_uid == buf2.st_uid );
	assert(buf1.st_gid == buf2.st_gid );
	assert(buf1.st_rdev == buf2.st_rdev );
	assert(buf1.st_size == buf2.st_size );
	assert(buf1.st_blksize== buf2.st_blksize);
	assert(buf1.st_blocks == buf2.st_blocks );

	assert(unlink(filename) == 0);
	assert(unlink(linkpath) == 0);
}
