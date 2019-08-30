#include "su.h"


void	print_shadow_entry(struct shadow_entry *entry)
{
	printf("%s:%s:%u:%u:%u:%u:%u:%u:%s\n",
	       entry->login_name,
	       entry->hashed_passwd,
	       entry->last_password_change,
	       entry->min_password_age,
	       entry->max_password_age,
	       entry->warning_period,
	       entry->inactivity_period,
	       entry->account_expiration_date,
	       entry->_reserved);
}

int32_t	parse_shadow_entry(char *entry, struct shadow_entry *sentry)
{
	assert(entry);
	assert(sentry);
	memset(sentry, 0, sizeof(struct shadow_entry));

	char	    **fields = strsplit(entry, ':');

	if (!fields) {
		return -1;
	}
	uint32_t    n_fields = array_size(fields);

	if (n_fields != SHADOW_ENTRY_NB_FIELDS) {
		free_array(fields);
		return -1;
	}

	sentry->login_name = fields[0];
	sentry->hashed_passwd = fields[1];
	sentry->last_password_change = atoi(fields[2]);

	if (strlen(fields[2]) == 0) {
		sentry->no_aging_features = true;
	}
	else if (sentry->last_password_change == 0) {
		sentry->change_passwd_next_login = true;
	}
	free(fields[2]);


	sentry->min_password_age = atoi(fields[3]);

	if (strlen(fields[3]) == 0 || sentry->min_password_age == 0) {
		sentry->no_min_age = true;
	}
	free(fields[3]);


	sentry->max_password_age = atoi(fields[4]);

	if (strlen(fields[4]) == 0) {
		sentry->no_max_age = true;
	}
	free(fields[4]);


	sentry->warning_period = atoi(fields[5]);

	if (strlen(fields[5]) == 0 || sentry->warning_period == 0) {
		sentry->no_warning_period = true;
	}
	free(fields[5]);


	sentry->inactivity_period = atoi(fields[6]);

	if (strlen(fields[6]) == 0) {
		sentry->no_inactivity_period = true;
	}
	free(fields[6]);

	sentry->account_expiration_date = atoi(fields[7]);
	if (strlen(fields[7]) == 0) {
		sentry->no_account_expiration = true;
	}
	free(fields[7]);

	sentry->_reserved = fields[8];
	free(fields);
	return 0;
}


struct shadow_entry *parse_shadow_file(uint32_t *n_entries) {
	int fd = open(SHADOW_FILE, O_RDONLY);

	if (-1 == fd) {
		err("Failed to open: %s: %s", SHADOW_FILE, strerror(errno));
	}
	char *contents = get_file_contents(fd);

	if (!contents) {
		err("Failed to read: %s", SHADOW_FILE);
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


	struct shadow_entry *sentries = malloc(sizeof(struct shadow_entry) * nbr_entries);

	if (!sentries) {
		err("Out of memory");
	}

	for (uint32_t i = 0; i < nbr_entries; i++) {
		if (strlen(entries[i]) == 0) {
			nbr_entries--;
			break;
		}

		int32_t ret = parse_shadow_entry(entries[i], &sentries[i]);

		if (-1 == ret) {
			err("Failed to parse entry %u in %s:\n%s\n", i, PASSWORD_FILE, entries[i]);
		}
	}
	free_array(entries);
	close(fd);
	*n_entries = nbr_entries;
	return sentries;
}

struct shadow_entry *find_corresponding_shadow_entry(struct shadow_entry *sentries,
						     uint32_t n_entries,
						     struct passwd_entry *entry)
{
	assert(sentries);
	assert(entry);

	for (uint32_t i = 0; i < n_entries; i++) {
		if (!strcmp(entry->login_name, sentries[i].login_name)) {
			return &sentries[i];
		}
	}
	return NULL;
}
