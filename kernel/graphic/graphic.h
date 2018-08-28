
#include "../lib/types.h"

struct cursor_position {
   // u8      raw[2];
    u8      X;
    u8      Y;
} cursor;

//struct cursor_position getCursorPosition();

void print(char*, ...);             // ... surcharge de fonction, nombre d'arguments variable !
void setTextColor(u8);
void setCursorPosition(u8,u8);
void draw(u32, u32, u32, u32);
void query_old_cursor_position(int);
void getCursorPosition(u8*,u8*);
