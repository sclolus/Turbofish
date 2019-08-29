#include "su.h"

void	print_passwd_entry(struct passwd_entry *pentry)
{
	char	*login_name = pentry->login_name ?: "";
	char	*hashed_passwd = pentry->hashed_passwd ?: "";
	char	*comment_field = pentry->comment_field ?: "";
	char	*user_home_directory = pentry->user_home_directory ?: "";
	char	*user_interpreter = pentry->user_interpreter ?: "";
	printf("%s:%s:%u:%u:%s:%s:%s\n", login_name,
	       hashed_passwd,
	       pentry->uid,
	       pentry->gid,
	       comment_field,
	       user_home_directory,
	       user_interpreter);
}

static uint32_t array_size(char **array) {
	uint32_t i = 0;

	while (array[i]) {
		i++;
	}
	return i;
}

void	free_array(char **array) {
	uint32_t i = 0;

	while (array[i]) {
		free(array[i]);
	}
	free(array);
}

int32_t	parse_passwd_entry(char *entry, struct passwd_entry *pentry)
{
	assert(entry);
	assert(pentry);
	char **fields = strsplit(entry, ':');
	uint32_t n_fields = array_size(fields);

	if (!fields) {
		return -1;
	}

	if (n_fields != ENTRY_NB_FIELDS) {
		/* free_array(fields); */
		return -1;
	}

	pentry->login_name = fields[0];
	pentry->hashed_passwd = fields[1];
	pentry->uid = atoi(fields[2]);
	pentry->gid = atoi(fields[3]);
	pentry->comment_field = fields[4];
	pentry->user_home_directory = fields[5];
	pentry->user_interpreter = fields[6];
	return 0;
}

# ifndef TESTS
int main(int argc, char **argv, char **env)
{
	if (argc != 2) {
		err("%s", USAGE);
	}

	int fd = open(PASSWORD_FILE, O_RDONLY);

	if (-1 == fd) {
		err("Failed to open: %s: %s", PASSWORD_FILE, strerror(errno));
	}
	char *contents = get_file_contents(fd);

	if (!contents) {
		err("Failed to read: %s", PASSWORD_FILE);
	}

	char	**entries = strsplit(contents, '\n');
	uint32_t nbr_entries = 0;

	if (!entries) {
		err("Out of memory");
	}

	while (entries[nbr_entries]) {
		nbr_entries++;
	}


	struct passwd_entry *pentries = malloc(sizeof(struct passwd_entry) * nbr_entries);

	if (!pentries) {
		err("Out of memory");
	}

	for (uint32_t i = 0; i < nbr_entries; i++) {
		int32_t ret = parse_passwd_entry(entries[i], &pentries[i]);

		/* if (-1 == ret) { */
		/* 	err("Failed to parse entry %u in %s\n", i, PASSWORD_FILE); */
		/* } */
	}

	char	*login = argv[1];
	struct passwd_entry *entry = NULL;

	for (uint32_t i = 0; i < nbr_entries; i++) {
		if (0 == strcmp(login, pentries[i].login_name)) {
			entry = &pentries[i];
			break ;
		}
	}

	if (!entry) {
		err("user %s does not exist", login);
	}

	/* check_passwd */

	if (-1 == setuid(entry->uid)) {
		err("Failed to setuid(%d (%s)): %s", entry->uid, login, strerror(errno));
	}

	/* if (-1 == setgid(entry->gid)) { */
	/* 	err("Failed to setgid(%d (%s)): %s", entry->gid, login, strerror(errno)); */
	/* } */

	print_passwd_entry(entry);

	char **av = (char*[]){entry->user_interpreter, NULL};
	execve(entry->user_interpreter, av, env);
	err("Failed to execute command: %s", strerror(errno));
}
# endif
