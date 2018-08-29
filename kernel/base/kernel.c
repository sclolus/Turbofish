
typedef unsigned int    u32;
typedef signed int		s32;
typedef unsigned short  u16;
typedef signed short	s16;
typedef unsigned char   u8;
typedef signed char		s8;

extern void iddle_mode();
extern void setCursorPosition(u8,u8);
extern void draw_line(u32, u32, u32, u32);
extern void putchar(u8);
extern void print(const char *s);

/*
struct cursor_position {
    u8      X;
    u8      Y;
} cursor;
*/

#define vesa_Info_Location  0x00032000
#define old_Cursor_Location 0x00032200

int main();

void _start(void)
{
	//draw_line(0, 0, 1023, 767);
	//draw_line(1023, 0, 0, 767);

    setCursorPosition(20, 20);
    putchar('C');
    putchar('F');

    print("Les roses de l'europe sont le parfum de satan\n\n");
    while (1)
    {

    }
    main();
}

int main()
{
	//iddle_mode();
    return(0);
}
