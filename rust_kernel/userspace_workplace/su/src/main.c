#include "su.h"

/* # ifndef TESTS */
int main(int argc, char **argv, char **env)
{
	if (argc != 2) {
		err("%s", USAGE);
	}

	uint32_t n_entries = 0;
	struct passwd_entry *pentries = parse_passwd_file(&n_entries);
	char	*login = argv[1];
	struct passwd_entry *entry = NULL;

	uint32_t n_shadow_entries = 0;
	struct shadow_entry *sentries = parse_shadow_file(&n_shadow_entries);

	for (uint32_t i = 0; i < n_entries; i++) {
		if (0 == strcmp(login, pentries[i].login_name)) {
			entry = &pentries[i];
			break ;
		}
	}

	if (!entry) {
		err("user %s does not exist", login);
	}

	# ifdef TESTS
	t_hash_info info = (t_hash_info) {
		.system_hash = &crypt,
		.hash = &md5_hash,
		.digest_size = 16,
		.salt = "12347980",
	};
	hash_fuzzer(&info);
	# else
	char		*input_password = NULL;
	size_t		input_len = 0;
	ssize_t ret = getline(&input_password, &input_len, stdin);

	if (-1 == ret) {
		err("Input io error");
	}

	input_password[ret - 1] = '\0';

	const char  *salt = "12346789";
	char	    *hash = md5_hash(input_password, salt, strlen(input_password));
	char	    *entry_passwd = NULL;

	if (hashed_passwd_is_in_shadow(entry)) {
		struct shadow_entry *sentry =
			find_corresponding_shadow_entry(sentries, n_shadow_entries, entry);

		if (!sentry) {
			err("Unable to find hash in shadow file for user: %s", login);
		}
		entry_passwd = sentry->hashed_passwd;
	} else {
		entry_passwd = entry->hashed_passwd;
	}

	if (!hash) {
		err("Failed to hash password");
	}
	print_hash((uint32_t*)hash, 16, 0);

	if (strcmp(hash, entry->hashed_passwd)) {
		err("Authentification failure");
	}
	# endif

	if (-1 == setuid(entry->uid)) {
		err("Failed to setuid(%d (%s)): %s", entry->uid, login, strerror(errno));
	}

	if (-1 == setgid(entry->gid)) {
		err("Failed to setgid(%d (%s)): %s", entry->gid, login, strerror(errno));
	}

	print_passwd_entry(entry);

	char **av = (char*[]){entry->user_interpreter, NULL};
	execve(entry->user_interpreter, av, env);
	err("Failed to execute command: %s", strerror(errno));
}
/* # endif */
