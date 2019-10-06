#include <stdlib.h>
#include <errno.h>
#include <unistd.h>

char *mktemp(char *template)
{
	size_t	i = 0;
	size_t	template_len = strlen(template);

	if (template_len < 6) {
		errno = EINVAL;
		return NULL;
	}

	if (strcmp("XXXXXX", &template[template_len - 6])) {
		errno = EINVAL;
		return NULL;
	}
	char	*end_of_template = &template[template_len - 6];


	do
	{
		char	*snb = atoi((int)i);

		*end_of_template = '\0';
		strcat(end_of_template, snb);
		free(snb);
		i++;
	}	while (0 == access(template, F_OK));

	return template;
}
