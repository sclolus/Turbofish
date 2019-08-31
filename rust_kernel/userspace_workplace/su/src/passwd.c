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

int32_t	parse_passwd_entry(char *entry, struct passwd_entry *pentry)
{
	assert(entry);
	assert(pentry);
	char **fields = strsplit(entry, ':');

	if (!fields) {
		return -1;
	}

	uint32_t n_fields = array_size(fields);


	if (n_fields != ENTRY_NB_FIELDS) {
		free_array(fields);
		return -1;
	}

	pentry->login_name = fields[0];
	pentry->hashed_passwd = fields[1];
	pentry->uid = atoi(fields[2]);
	free(fields[2]);
	pentry->gid = atoi(fields[3]);
	free(fields[3]);
	pentry->comment_field = fields[4];
	pentry->user_home_directory = fields[5];
	pentry->user_interpreter = fields[6];
	free(fields);
	return 0;
}

struct passwd_entry *parse_passwd_file(uint32_t *n_entries) {
	int fd = open(PASSWORD_FILE, O_RDONLY);

	if (-1 == fd) {
		err_errno("Failed to open: %s", PASSWORD_FILE);
	}
	char *contents = get_file_contents(fd);

	if (!contents) {
		err("Failed to read: %s", PASSWORD_FILE);
	}

	char	**entries = strsplit(contents, '\n');

	free(contents);
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
		if (strlen(entries[i]) == 0) {
			nbr_entries--;
			break;
		}

		int32_t ret = parse_passwd_entry(entries[i], &pentries[i]);

		if (-1 == ret) {
			err("Failed to parse entry %u in %s\n", i, PASSWORD_FILE);
		}
	}
	free_array(entries);
	close(fd);
	*n_entries = nbr_entries;
	return pentries;
}

bool		hashed_passwd_is_in_shadow(struct passwd_entry *entry)
{
	return !strcmp(entry->hashed_passwd, "x");
}
