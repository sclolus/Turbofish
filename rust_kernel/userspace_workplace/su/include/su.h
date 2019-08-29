#ifndef __SU_H__
# define __SU_H__
# include <stdint.h>
# include <unistd.h>
# include <fcntl.h>
# include <stdlib.h>
# include <string.h>
# include <assert.h>
# include <stdio.h>
# include <errno.h>

# define BIN_NAME "su"
# define PASSWORD_FILE "/etc/passwd"
# define USAGE BIN_NAME ": <login_name>"
# define ENTRY_NB_FIELDS 7

struct passwd_entry {
	char	*login_name;
	char	*hashed_passwd;
	uid_t	uid;
	gid_t	gid;
	char	*comment_field;
	char	*user_home_directory;
	char	*user_interpreter;
};

# define err(format, ...) do {					\
		     dprintf(2, BIN_NAME ": " format "\n" __VA_OPT__(,) __VA_ARGS__);	\
	     exit(EXIT_FAILURE);		\
	     } while (0);


char			**strsplit(char *const s, char c);
char			*get_file_contents(int fd);
#endif /* __SU_H__ */
