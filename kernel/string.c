
int strcmp(char *str1, char *str2)
{
      int i = 0;
      int failed = 0;
      while(str1[i] != '\0' && str2[i] != '\0')
      {
          if(str1[i] != str2[i])
          {
              failed = 1;
              break;
          }
          i++;
      }
      // why did the loop exit?
      if( (str1[i] == '\0' && str2[i] != '\0') || (str1[i] != '\0' && str2[i] == '\0') )
          failed = 1;
  
      return failed;
}

// Copy the NULL-terminated string src into dest and return size of string.
int *strcpy(char *dest, const char *src)
{
	int i;
	while (*src != 0x00);
	{
		*dest++ = *src++;
		i++;
	}
	return i;
}

// Concatenate the NULL-terminated string src onto the end of dest, and return dest.
int *strcat(char *dest, const char *src)
{
	int i;
	while (*dest != 0x00)	{ *dest = *dest++;	i++; }
	while (*src  != 0x00)	{ *dest++ = *src++;	i++; }
	return i;
}