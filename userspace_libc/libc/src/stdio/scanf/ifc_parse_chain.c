#include "ifc.h"

static int buff = 0;

/*
 * Get the next character on Chain / Stream
 */
int get_content(struct Ctx *ctx)
{
	// Get a new character only is buff is empty
	if (buff != 0) {
		return buff;
	}

	if (ctx->flavor == String) {
		if (*ctx->input.str == '\0') {
			buff = EOF;
		} else {
			char readen_char = *ctx->input.str++;
			buff = (int)readen_char;
		}
	} else {
		buff = getc(ctx->input.stream);
		// I don't know if i must consider the '\0' as an end of input
		if (buff == (char)'\0') {
			buff = EOF;
		}
	}
	return buff;
}

/*
 * The content has been utilized. Unload it
 */
void consume_content(void)
{
	buff = 0;
}

/*
 * Trash all whitespace on the input Chain / Stream
 */
void trash_whitespaces_on_input(struct Ctx *ctx)
{
	int ret = get_content(ctx);
	while (IS_SPACE((char)ret) && ret != EOF) {
		consume_content();
		ret = get_content(ctx);
	}
}

int parse_chain(struct Ctx *ctx, const char *format)
{
	int ret;
	int nb_conversion_done;
	bool conversion_failed;

	conversion_failed = false;
	nb_conversion_done = 0;

	buff = 0;
	while (*format != '\0') {
		bool on_whitespace_sequence = false;
		/*
		 * 1: A sequence of white-space characters (space, tab, newline,
		 * etc.; see isspace(3)).  This directive matches any amount
		 * of white space, including none, in the input.
		 */
		while (IS_SPACE(*format) == true) {
			on_whitespace_sequence = true;
			format++;
		}
		if (on_whitespace_sequence == true) {
			trash_whitespaces_on_input(ctx);
			continue;
		}

		ret = get_content(ctx);
		if (ret == EOF) {
			break;
		}

		char readen_char = (char)ret;
		if (*format != '%' || (*format == '%' && format[1] == '%')) {
			/*
			 * 2: An ordinary character (i.e., one other than white
			 * space or '%').  This character must exactly match
			 * the next character of input
			 */
			if (readen_char != *format) {
				// Normally here. we have to fseek of -1
				conversion_failed = true;
				break;
			} else {
				if (*format == '%' && format[1] == '%') {
					// Manage the '%' specifier
					nb_conversion_done += 1;
					format += 2;
				} else {
					format += 1;
				}
			}
			consume_content();
		} else {
			/*
			 * 3: A conversion specification, which commences with a
			 * '%' (percent) character.  A sequence of characters
			 * from the input is converted according to this
			 * specification, and the result is placed in the
			 * corresponding pointer argument. If the next item of
			 * input does not match the conversion specification,
			 * the conversion fails—this is a matching failure.
			 */
			 struct ConvertResult result = convert(&format);
			 if (result.f == NULL
					 || result.f(ctx, &result.args) < 0) {
				conversion_failed = true;
				break;
			 }
			 nb_conversion_done += 1;
		}
	}

	if (ctx->flavor == Stream) {
		/*
		 * EOF is returned if a read error occurs,
		 */
		if (ferror(ctx->input.stream)) {
			return EOF;
		}
	}
	if (ret == EOF
		&& *format != '\0'
		&& nb_conversion_done == 0
		&& conversion_failed == false) {
		/*
		 * The value EOF is returned if the end of input is reached
		 * before either the first successful conversion or a matching
		 * failure occurs.
		 */
		return EOF;
	}
	/*
	 * On success, these functions return the number of input items
	 * successfully matched and assigned; this can be fewer than provided
	 * for, or even zero, in the event of an early matching failure.
	 */
	return nb_conversion_done;
}
