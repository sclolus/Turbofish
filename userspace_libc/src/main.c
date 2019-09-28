#include "libc.h"

#include <stdio.h>

int test(const char *s, const char *fmt, ...) {
	int ret_turbofish_libc;
	int ret_linux_libc;

	va_list ap1;
	va_list ap2;

	va_start(ap1, fmt);
	va_start(ap2, fmt);

	ret_linux_libc = vsscanf(s, fmt, ap2);
	// it is so difficult to know if our functions modify the entries !
	// We need to parse the format field and check each entries
	ret_turbofish_libc = xvsscanf(s, fmt, ap1);

	printf("turbofish: %i, linux: %i\n", ret_turbofish_libc, ret_linux_libc);
	printf("%s\n", ret_turbofish_libc == ret_linux_libc ? "SUCCESS" : "FAIL");

	if (ret_turbofish_libc != ret_linux_libc) {
		printf("input: '%s', fmt: '%s'\n", s, fmt);
	}

	va_end(ap1);
	va_end(ap2);

	return ret_turbofish_libc == ret_linux_libc;
}

#include <unistd.h>
#include <fcntl.h>

const char filename[] = "tmp";

int main(void)
{
	/*
	unlink(filename);
	int f = open(filename, O_CREAT | O_WRONLY | O_TRUNC, 0666);
	char input[] = "dc       ";
	write(f, input, strlen(input));
	close(f);

	FILE *file = fopen(filename, "r");
	int ret = fscanf(file, "   dc                ");
	printf("scanf return: %i\n", ret);
	char buf[512];
	printf("'");
	while ((ret = fread(buf, 1, 1, file)) > 0) {
		printf("%c", buf[0]);
	}
	printf("'");
	puts("");

	int ret = fread(buf, 512, 1, file);
	if (ret < 0) {
		perror("fread");
		exit(-1);
	}
	buf[ret] = '\0';
	printf("buf => '%s'\n", buf);

	return 0;
	*/

	test("bananes", "bananes");
	test("", "");
	test("", "bananes");
	test("   ", "bananes      ");
	test("   ", "bananes");
	test("   ", "\tbananes");
	test("h", "bananes");
	test("ba   nanes", "ba   nanes");
	test(" ", " ");
	test(" ", "\t");
	test("\t", " ");
	test("      ", "          ");
	test("   ", "  bananes      ");
	test(" \t  ", " bananes      ");
	test("                     h", "ban anes");
	test("dc     ", "   dc                ");
	test("  dc     ", "dc                ");
	test("dc     ", "dc                ");
	test("dc     ", "dc        f        ");
	test("dc   f  ", "dc        f        ");
	test(" dc   f  ", "  dc                ");
	test(" dc   f  ", "  d c                ");
	test(" d    c   f  ", "  d             f    ");
	test(" d         ", "  dh");
	test("                     h", "ban anes");
	test("dc     ", "   dc                ");
	test("  dc     ", "dc                ");
	test("dc     ", "dc                ");
	test("dc     ", "dc        f        ");
	test("dc   f  ", "dc        f        ");
	test(" dc   f  ", "  dc                ");
	test(" dc   f  ", "  d c                ");
	test(" d    c   f  ", "  d             f    ");
	test(" d         ", "  dh");

	char s1[512];
	char s2[512];
	test("        bananes cuites", "%s %s", s1, s2);
	printf("'%s' '%s'\n", s1, s2);

	char nom[512];
	char prenom[512];
	float f;
	float g;

	printf("Entrez votre nom/prenom et deux flotants:\n");
	int ret = xscanf("%s %s %0f %f", nom, prenom, &f, &g);
	printf("scanned buf: `%s %s %f %f` ret = %i\n", nom, prenom, f, g, ret);


	///char s[] = "bananes";
	//xsscanf(s, "%s", buf);
/*
	xprintf("Les %i carrotes sont cuites\n", 42);

	int i;
	int j;
	int k;
	char s[] = "bananes";

	char *fmt;
	char buf_s[512];
	char buf_p[512];	

	fmt = "%i    %i\t\t\t%i%3s";
	int a = 42;
	int b = 84;
	int c = 168;
	char s2[512];
	sprintf(buf_p, fmt, a, b, c, s);
	sscanf(buf_p, fmt, &i, &j, &k, s2);
	sprintf(buf_s, fmt, i, j, k, s2);
	printf("origin: '%s'\n", buf_p);
	printf("final:  '%s'\n", buf_s);
*/
	return 0;
}
