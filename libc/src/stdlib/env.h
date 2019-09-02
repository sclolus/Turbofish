#ifndef _ENV_H_
# define _ENV_H_
# include <stdint.h>

#ifndef __PRIVATE_USE_ENV_H__
# error "This is a private header of the libc, do not use directly"
#endif

int32_t	handle_null_environ(void);
char	*make_env_entry(const char *envname, const char *envval);
char	**search_env(const char *envname);

#endif /* _ENV_H_ */
