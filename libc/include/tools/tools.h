#ifndef __TOOLS_H__
# define __TOOLS_H__

#include <sys/types.h>

typedef unsigned int uint32_t;
typedef int int32_t;

char **strsplit(char *const s, char del);

uint32_t array_size(void **array);
void free_array(void **array);

void **parse_2d_file(
		     const char *filename,
		     char delim_1,
		     char delim_2,
		     size_t structure_len,
		     int fn(char **raw_fields, void *s));

# define err(format, ...) do {						\
		dprintf(2, format "\n" __VA_OPT__(,) __VA_ARGS__); \
		     exit(EXIT_FAILURE);				\
	     } while (0);

# define err_errno(format, ...) do {						\
		dprintf(2, format ": %s\n" __VA_OPT__(,) __VA_ARGS__, strerror(errno)); \
		exit(EXIT_FAILURE);					\
	} while (0);

# define warn(format, ...) do {						\
		dprintf(2, "Warning: " format "\n" __VA_OPT__(,) __VA_ARGS__); \
	} while (0);

# define warn_errno(format, ...) do {						\
		dprintf(2, "Warning: " format ": %s\n" __VA_OPT__(,) __VA_ARGS__, strerror(errno)); \
	} while (0);


#endif
