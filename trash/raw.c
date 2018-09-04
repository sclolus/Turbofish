void		clear_screen(void)
{
/* this loops clears the screen
	* there are 25 lines each of 80 columns; each element takes 2 bytes */

	u32 j = 0;
	u32 screensize = 80 * 25 * 2;
	/* video memory begins at address 0xb8000 */
	char *vidptr = (char*)0xb8000;
	while (j < screensize) {
		/* blank character */
		vidptr[j] = ' ';
		/* attribute-byte */
		vidptr[j+1] = 0x07;
		j = j + 2;
	}
}

void		term_putchar(char c)
{
	static int i = 0;

	/* video memory begins at address 0xb8000 */
	char *vidptr = (char*)0xb8000;

	/* the character's ascii */
	vidptr[i++] = c;
	/* attribute-byte: give character black bg and light grey fg */
	vidptr[i++] = 0x07;
}

void		term_putstr(char *str)
{
	while (*str != '\0')
	{
		term_putchar(*str);
		str++;
	}
}
