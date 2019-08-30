#include "su.h"
#include <termios.h>

 // put this into libc, and discuss with the team about its obsolescence.
char	    *getpass(const char *prompt)
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
	ssize_t	    ret = getline(&pass, &pass_size, stdin);
	write(STDOUT_FILENO, "\n", 1);

	if (-1 == ret) {
		free(pass);
		pass = NULL;
	}

	if (pass && pass_size != 0) {
		pass[pass_size - 1] = '\0';
	}

	if (-1 == tcsetattr(STDIN_FILENO, TCSANOW, &old)) {
		free(pass);
		return NULL;
	}
	return pass;
}
