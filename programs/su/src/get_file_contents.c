# include "su.h"

struct s_string {
	char	    *str;
	uint32_t    len;
	uint32_t    capacity;
};

struct s_string *alloc_string(uint32_t size)
{
	size = size + 1;

	char	*str = malloc(size);

	if (!str) {
		return NULL;
	}
	memset(str, 0, size);

	struct s_string *string = malloc(sizeof(struct s_string));


	if (!string) {
		free(str);
		return NULL;
	}

	*string = (struct s_string){
		.str = str,
		.len = 0,
		.capacity = size,
	};
	return string;
}

int32_t	string_push_str(struct s_string *string, const char *const str, uint32_t size)
{
	assert(string);
	assert(str);

	uint32_t    new_len = string->len + size;
	if (new_len > string->capacity) {
		uint32_t	new_capacity = new_len * 2;
		char		*restr = realloc(string->str, new_capacity + 1);

		if (!restr) {
			return -1;
		}

		string->str = restr;
		string->capacity = new_capacity;
	}

	memcpy(string->str + string->len, str, size);
	string->len = new_len;
	string->str[new_len] = '\0';
	return 0;
}

void	free_string(struct s_string *string)
{
	free(string->str);
	free(string);
}

char	*get_file_contents(int fd)
{
	static char read_buf[4096];
	struct s_string *string = alloc_string(4096);

	if (!string) {
		return NULL;
	}

	ssize_t	ret;
	while ((ret = read(fd, read_buf, sizeof(read_buf)))) {
		if (ret == -1) {
			free_string(string);
			return NULL;
		}

		if (-1 == string_push_str(string, read_buf, ret)) {
			free_string(string);
			return NULL;
		}
	}

	char	*str = string->str;
	free(string); // just free the data structure;
	return str;
}
