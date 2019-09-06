#include <fcntl.h>
#include <stdlib.h>
#include <stdio.h>
#include <stdint.h>
#include <string.h>
#include <unistd.h>

char PULP_FICTION_QUOTE[] = "The path of the righteous man is beset on all sides by the iniquities of the selfish and the tyranny of evil men.\n\
Blessed is he who, in the name of charity and good will, shepherds the weak through the valley of darkness, for he is truly his brother's keeper and the finder of lost children.\n\
And I will strike down upon thee with great vengeance and furious anger those who attempt to poison and destroy my brothers. And you will know my name is the Lord when I lay my vengeance upon thee.\n";

size_t QUOTE_LEN = sizeof(PULP_FICTION_QUOTE);


int main() {
	char buf[QUOTE_LEN];
	int fd = open("/home/pulp_fiction.txt", O_RDONLY);
	if (fd == -1) {
		perror("open");
		exit(1);
	}
	int ret = read(fd, buf, QUOTE_LEN - 1);
	dprintf(2, "bytes readen %d, quote len %lu\n", ret, QUOTE_LEN);
	
	if (ret != (int)QUOTE_LEN - 1) {
		dprintf(2, "bytes readen %d\n", ret);
		exit(1);
	}
	if (ret == -1) {
		perror("read");
		exit(1);
	}
	buf[QUOTE_LEN - 1] = 0;
	dprintf(2, "TRUE PULP FICTION QUOTE: '%s'\n", PULP_FICTION_QUOTE);
	dprintf(2, "PULP FICTION QUOTE: '%s'\n", buf);
	if (strcmp(PULP_FICTION_QUOTE, buf) != 0) {
		exit(1);
	}
}
