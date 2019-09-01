
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <tools.h>
#include <sys/stat.h>
#include <fcntl.h>

static int32_t parse_2d_entry(
			      char *entry,
			      void *target,
			      char delim,
			      int fn(char **raw_fields, void *s))
{
	if (entry == NULL || target == NULL) {
		return -1;
	}

	char **fields = strsplit(entry, delim);
	if (!fields) {
		return -1;
	}

	int32_t ret = fn(fields, target);
	free(fields);
	return ret;
}

/*
 * Parse an organized file with two dimensions delimiters like /etc/passwd or /etc/groups
 */
void **parse_2d_file(
		     const char *filename,
		     char delim_1,
		     char delim_2,
		     size_t structure_len,
		     int fn(char **raw_fields, void *s))
{
	int fd = open(filename, O_RDONLY);
	if (fd < 0) {
		perror("open");
		return NULL;
	}

	struct stat summary;
	if (fstat(fd, &summary) < 0) {
		perror("stat");
		close(fd);
		return NULL;
	}

	size_t len = (size_t)summary.st_size;

	// NMAP should be better for doing that
	char *buf = malloc(len + 1);
	if (buf == NULL) {
		close(fd);
		return NULL;
	}
	buf[len] = '\0';

	size_t readen_bytes = 0;
	while (readen_bytes < len) {
		int ret = read(fd, buf + readen_bytes, len - readen_bytes);
		if (ret < 0) {
			perror("read");
			free(buf);
			close(fd);
			return NULL;
		}
		readen_bytes += (size_t)ret;
	}
	close(fd);

	char **entries = strsplit(buf, delim_1);
	free(buf);
	if (entries == NULL) {
		return NULL;
	}

	uint32_t nbr_entries = 0;
	for (int idx = 0; entries[idx] != NULL; idx++) {
		if (entries[idx][0] != '\0' && entries[idx][0] != '#') {
			nbr_entries += 1;
		}
	}

	void **output = (void **)malloc((nbr_entries + 1) * sizeof(void *));
	if (output == NULL) {
		free_array((void **)entries);
		return NULL;
	}

	output[nbr_entries] = NULL;
	for (uint32_t i = 0; i < nbr_entries; i++) {
		if (entries[i][0] != '\0' && entries[i][0] != '#') {
			output[i] = malloc(structure_len);
			if (output[i] == NULL || parse_2d_entry(entries[i], output[i], delim_2, fn) != 0) {
				free_array((void **)entries);
				free_array((void **)output);
				return NULL;
			}
		}
	}
	free_array((void **)entries);
	return output;
}
