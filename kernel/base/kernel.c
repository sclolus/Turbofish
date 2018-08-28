
#include "../../kernel/lib/types.h"

void iddle_mode();
void setCursorPosition(u8,u8);
void draw(u32, u32, u32, u32);
void putchar_f(u8);

extern void print(const char *s);

struct cursor_position {
    u8      X;
    u8      Y;
} cursor;

#define vesa_Info_Location  0x00032000
#define old_Cursor_Location 0x00032200

int main();

void _start(void)
{
	draw(0, 0, 1023, 767);
	draw(1023, 0, 0, 767);

    setCursorPosition(20, 20);
    putchar_f('C');
    putchar_f('F');

    print("Les roses de l'europe sont le parfum de satan\n\n");
    while (1)
    {

    }
    main();
}

int main()
{
	iddle_mode();
    return(0);
}
