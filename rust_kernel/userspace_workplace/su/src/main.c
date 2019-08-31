#include "su.h"

/* # ifndef TESTS */
struct	cmd_args    parse_cmd_line(int argc, char **argv) {
	int		opt;
	struct cmd_args	args;

	memset(&args, 0, sizeof(struct cmd_args));
	while ((opt = _getopt(argc, argv, OPTIONS_GETOPT)) != -1) {
		switch (opt) {
		case 'c':
			args.specified_command = true;
			args.command = g_optarg;
			break;
		case 's':
			args.specified_shell = true;
			args.shell = g_optarg;
			break;
		case 'l':
			args.login_shell = true;
			break;
		case 'm':
			args.preserve_env = true;
			break;
		case 'p':
			args.preserve_env = true;
			break;
		default: // which is the '?' value
			err("%s", USAGE);
		}
	}
	if (g_optind == argc) {
		args.login = "root";
		args.is_root = true;
	} else if (optind < argc) {
		args.login = argv[g_optind];
	} else {
		err("Too many arguments provided");
	}
	return args;
}

#ifndef MAKE_PASS
int main(int argc, char **argv, char **env)
{
	struct cmd_args args = parse_cmd_line(argc, argv);

	uint32_t n_entries = 0;
	struct passwd_entry *pentries = parse_passwd_file(&n_entries);
	char	*login = args.login;
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

	char		*input_password = getpass("Password: ");

	if (!input_password) {
		err("Failed to retrieve password");
	}
	size_t		pass_len = strlen(input_password);

	char	    *salt = NULL;
	char	    *entry_passwd = NULL;


	if (hashed_passwd_is_in_shadow(entry)) {
		struct shadow_entry *sentry =
			find_corresponding_shadow_entry(sentries, n_shadow_entries, entry);

		if (!sentry) {
			err("Unable to find hash in shadow file for user: %s", login);
		}
		if (-1 == parse_hashed_password(sentry->hashed_passwd,
						&entry_passwd,
						&salt)) {
			err("Failed to parse hashed password in shadow file");
		}
	} else if (-1 == parse_hashed_password(entry->hashed_passwd,
					       &entry_passwd,
					       &salt)) {
		err("Failed to parse hashed password in shadow file");
	}

	// Bzero for security reasons.
	memset(sentries, 0, sizeof(struct shadow_entry) * n_shadow_entries);
	free(sentries);

	char	    *hash = md5_hash(input_password, salt);

	// Bzero for security reasons.
	memset(input_password, 0, pass_len);
	free(input_password);

	if (!hash) {
		err("Failed to hash password");
	}

	// decoded from base64.
	char	*decoded_entry_passwd = decode_base64(entry_passwd, strlen(entry_passwd));

	// Bzero for security reasons.
	memset(entry_passwd, 0, strlen(entry_passwd));
	free(entry_passwd);

	if (!decoded_entry_passwd) {
		err("Failed to encode hash into base64");
	}

	if (memcmp(hash, decoded_entry_passwd, 16)) {
		err("Authentification failure");
	}

	memset(decoded_entry_passwd, 0, 16);
	free(decoded_entry_passwd);

	// Bzero for security reasons.
	memset(salt, 0, strlen(salt));
	free(salt);
	memset(hash, 0, 16);
	free(hash);

	if (-1 == setgid(entry->gid)) {
		err("Failed to setgid(%d (%s)): %s", entry->gid, login, strerror(errno));
	}

	if (-1 == setuid(entry->uid)) {
		err("Failed to setuid(%d (%s)): %s", entry->uid, login, strerror(errno));
	}

	char	*used_shell = NULL;
	char	*env_shell = NULL;

	if (args.specified_shell) {
		used_shell = args.shell;
	} else if (args.preserve_env && (env_shell = getenv("SHELL"))) {
		/* this is not supported:
		   If the target user has a restricted shell (i.e. not  listed  in  /etc/shells),
		   the  --shell option and the SHELL environment variables are ignored unless the
		   calling user is root.
		*/
		used_shell = env_shell;
	} else if (entry->user_interpreter && strcmp("", entry->user_interpreter)) {
		used_shell = entry->user_interpreter;
	} else {
		used_shell = "/bin/sh";
	}

	char **av;

	if (args.specified_command) {
		av = (char*[]){used_shell, "-c", args.command, NULL};
	} else {
		av = (char*[]){used_shell, NULL};
	}

	if (args.login_shell) {
		// TODO: clear the whole env except TERM.
		/* char *term = getenv("TERM"); */

		/* if (term) { */
		/* 	term = strdup(term); */
		/* } */

		/* clear_environ(); */

		/* if (-1 == setenv("TERM", term, true)) { */
		/* 	err("Failed to setenv(TERM): %s", strerror(errno)); */
		/* } */

		if (-1 == chdir(entry->user_home_directory)) {
			warn("Failed to change to target's home directory: %s", strerror(errno));
		}
		av[0] = "-";
	}

	if (!args.preserve_env || args.login_shell) {
		if (-1 == setenv("HOME", entry->user_home_directory, true)) {
			err("Failed to setenv(HOME): %s", strerror(errno));
		}

		if (-1 == setenv("SHELL", used_shell, true)) {
			err("Failed to setenv(SHELL): %s", strerror(errno));
		}

		if (!args.is_root && -1 == setenv("USER", entry->login_name, true)) {
			err("Failed to setenv(USER): %s", strerror(errno));
		}

		if (!args.is_root && -1 == setenv("LOGNAME", entry->login_name, true)) {
			err("Failed to setenv(LOGNAME): %s", strerror(errno));
		}
	}

	// Bzero for security reasons.
	memset(pentries, 0, sizeof(struct passwd_entry) * n_entries);
	free(pentries);
	execve(used_shell, av, env);
	err("Failed to execute command: %s", strerror(errno));
}
#endif

#ifdef MAKE_PASS

# warning This helper is not meant to be used directly on turbofish to make password. \
	It is not protected against dirty pages sniffing.

int main(int argc, char **argv) {
	if (argc != 3) {
		err("Invalid command line: ./hasher <key> <salt>");
	}

	char	*key = argv[1];
	char	*salt = argv[2];
	/* char	*salt; */

	/* static char salt[SALT_SIZE + 1]; */
	int	    fd = open("/dev/urandom", O_RDONLY);

	if (-1 == fd) {
		err("Failed to open /dev/urandom: %s", strerror(errno));
	}

	/* ssize_t ret = read(fd, salt, SALT_SIZE); */

	/* if (-1 == ret) { */
	/* 	err("Failed to read random salt: %s", strerror(errno)); */
	/* } */

	/* if (ret != SALT_SIZE) { */
	/* 	err("Salt was partially read: %ld != %u", ret, SALT_SIZE); */
	/* } */

	/* char	*password = getpass("Enter password to hash: "); */

	/* if (!password) { */
	/* 	err("Failed to retrieve password from user"); */
	/* } */

	/* char	*hash = md5_hash(password, salt); */
	char	*hash = md5_hash(key, salt);

	if (!hash) {
		err("Failed to hash password");
	}
	char *base64_hash = encode_base64(hash, 16);
	assert(!strcmp(hash, decode_base64(base64_hash, strlen(base64_hash))));

	if (!base64_hash) {
		err("Failed to convert hash into base64");
	}

	printf("Hashed password: %s Salt: %s\n", hash, salt);
	printf("base64: %s\n", base64_hash);
	printf("Formatted hash: $%s$%s\n", salt, base64_hash);

	free(hash);
	/* free(password); */
	return EXIT_SUCCESS;
}
#endif
/* # endif */
