#include <unistd.h>
#include <stdio.h>

size_t BUFF_SIZE = 20;

int main() {
	char buff[BUFF_SIZE + 1];
	int ret;

	buff[BUFF_SIZE] = '\0';

	printf("enter your text: \n");
	while ((ret = read(0, (char *)buff, BUFF_SIZE)) > 0)
	{
		buff[ret] = '\0';
		printf("text_readen:'%s'\n", buff);
	}
}
