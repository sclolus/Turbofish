#include "su.h"

/* # ifndef TESTS */
struct	cmd_args    parse_cmd_line(int argc, char **argv) {
	int		opt;
	struct cmd_args	args;

	memset(&args, 0, sizeof(struct cmd_args));
	while ((opt = getopt(argc, argv, OPTIONS_GETOPT)) != -1) {
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
	if (optind == argc) {
		args.login = "root";
		args.is_root = true;
	} else if (optind < argc) {
		args.login = argv[g_optind];
	} else {
		err("Too many arguments provided");
	}
	return args;
}

int main(int argc, char **argv, char **env)
{
	struct cmd_args args = parse_cmd_line(argc, argv);

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

	// Can't find why this does not work...
	/* if (-1 == setgid(entry->gid)) { */
	/* 	err("Failed to setgid(%d (%s)): %s", entry->gid, login, strerror(errno)); */
	/* } */

	print_passwd_entry(entry);
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

	if (!args.preserve_env) {
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

	execve(used_shell, av, env);
	err("Failed to execute command: %s", strerror(errno));
}
/* # endif */
