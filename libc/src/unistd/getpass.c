#include <unistd.h>
#include <stdio.h>
#include <string.h>
#include <termios.h>
#include <stdlib.h>

// Discuss with the team about its obsolescence.
char	    *getpass(const char *const prompt)
{
	struct termios	old;
	struct termios	new;
	char		*pass = NULL;

	if (-1 == tcgetattr(STDIN_FILENO, &old)) {
		return NULL;
	}

	new = old;
	new.c_lflag &= ~(ECHO);
	if (-1 == tcsetattr(STDIN_FILENO, TCSANOW, &new)) {
		return NULL;
	}
	size_t    pass_size = 0;

	write(STDOUT_FILENO, prompt, strlen(prompt));
	ssize_t	    ret = ft_getline(&pass, &pass_size, stdin);
	write(STDOUT_FILENO, "\n", 1);

	if (-1 == ret) {
		free(pass);
		pass = NULL;
	}

	char	*newline;
	if (pass && (newline = strchr(pass, '\n'))) {
		*newline = '\0';
	}

	if (-1 == tcsetattr(STDIN_FILENO, TCSANOW, &old)) {
		free(pass);
		return NULL;
	}
	return pass;
}

# warning missing tests
