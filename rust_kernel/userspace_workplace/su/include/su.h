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
# include <stdbool.h>

# include "getopt.h" // put this into the libc

# define BIN_NAME "su"
# define PASSWORD_FILE "/etc/passwd"
# define SHADOW_FILE "/etc/shadow"
# define USAGE BIN_NAME " ["OPTIONS"] [--] [user]"
# define OPTIONS "clmps"
# define OPTIONS_GETOPT "c:lmps:"
# define ENTRY_NB_FIELDS 7
# define SHADOW_ENTRY_NB_FIELDS 9

# define INLINE __attribute__((always_inline)) inline
# define NORETURN __attribute__((noreturn)) void

struct	cmd_args {
	uint32_t    specified_command : 1,
		    login_shell : 1,
		    preserve_env: 1,
		specified_shell: 1,
		is_root : 1;
	char	*command;
	char	*shell;
	char	*login;
};

struct	cmd_args    parse_cmd_line(int argc, char **argv);


struct passwd_entry {
	char	*login_name;
	char	*hashed_passwd;
	uid_t	uid;
	gid_t	gid;
	char	*comment_field;
	char	*user_home_directory;
	char	*user_interpreter;
};
int32_t	parse_passwd_entry(char *entry, struct passwd_entry *pentry);
struct passwd_entry *parse_passwd_file(uint32_t *n_entries);
void	print_passwd_entry(struct passwd_entry *pentry);
bool	hashed_passwd_is_in_shadow(struct passwd_entry *entry);

struct shadow_entry {
	char	    *login_name;
	char	    *hashed_passwd;
	uint32_t    last_password_change;
	uint32_t    min_password_age;
	uint32_t    max_password_age;
	uint32_t    warning_period;
	uint32_t    inactivity_period;
	uint32_t    account_expiration_date;
	char	    *_reserved;
	uint32_t    change_passwd_next_login : 1,
		no_aging_features : 1,
		no_min_age : 1,
		no_max_age : 1,
		no_warning_period : 1,
		no_inactivity_period: 1,
		no_account_expiration: 1;

};

int32_t	parse_shadow_entry(char *entry, struct shadow_entry *sentry);
struct shadow_entry *parse_shadow_file(uint32_t *n_entries);
void	print_shadow_entry(struct shadow_entry *entry);
struct shadow_entry *find_corresponding_shadow_entry(struct shadow_entry *sentries,
						     uint32_t n_entries,
						     struct passwd_entry *entry);

# define err(format, ...) do {					\
		     dprintf(2, BIN_NAME ": " format "\n" __VA_OPT__(,) __VA_ARGS__);	\
	     exit(EXIT_FAILURE);		\
	     } while (0);


char			**strsplit(char *const s, char c);
char			*get_file_contents(int fd);
uint32_t		*md5_hash(char *clear, char *salt, uint64_t len);

/*
** Hash Tests
*/

typedef uint32_t *(*t_hash_function)(char*, char*, uint64_t);
typedef char *(*t_system_hash_function)(const char*, const char*);

typedef struct	s_hash_info
{
	t_system_hash_function	system_hash;
	t_hash_function		hash;
	char			*salt;
	uint64_t		digest_size;
}				t_hash_info;

# define MAX_RANDOM_MESSAGE_LEN 512 * 4 + 11
# define RANDOM_INIT 0xBADA55
# ifdef TESTS
int		hash_tester(void *message
					   , uint32_t *to_test_digest
					   , uint64_t len
					   , t_hash_info *hash_info);
NORETURN	hash_fuzzer(t_hash_info *hash_info);
# endif

/*
** Useful functions
*/

/* # ifdef TESTS */
void	print_hash(uint32_t *digest, uint64_t size, int32_t swap_endian);

/* # endif */

uint32_t    left_rotate_32(uint32_t word, uint32_t delta);
void	    print_memory(const void *addr, size_t size);
uint32_t    array_size(char **array);
void	    free_array(char **array);

char	    *getpass(const char *prompt); // put this into libc, and discuss with the team about its obsolescence.

#endif /* __SU_H__ */
