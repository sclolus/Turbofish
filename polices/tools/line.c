#include <stdio.h>
#include <string.h>

char *replace_str(char *str, char *orig, char *rep)
{
	static char buffer[4096];
	char *p;

	if(!(p = strstr(str, orig)))  // Is 'orig' even in 'str'?
	return str;

	strncpy(buffer, str, p-str); // Copy characters from 'str' start to 'orig' st$
	buffer[p-str] = '\0';

	sprintf(buffer+(p-str), "%s%s", rep, p+strlen(orig));

	return buffer;
}

int main(void)
{
	char input_File_Name[]="alpha_part2.txt";
	FILE* input_File = NULL;
	input_File = fopen(input_File_Name,"r");
	if (input_File == NULL) { printf("Impossible d'ouvrir le fichier %s en lecture.\n",input_File_Name); return 1; }
	
	char chaine[800] = "";
// Prototype: char* fgets(char* chaine, int nbreDeCaracteresALire, FILE* pointeurSurFichier);
	
	while (fgets(chaine, 800, input_File) != NULL)
	{
		if(strstr(chaine, "ENDCHAR") 		!= NULL) 	continue;
		if(strstr(chaine, "SWIDTH") 		!= NULL) 	continue;
		if(strstr(chaine, "DWIDTH") 		!= NULL) 	continue;
		if(strstr(chaine, "BBX" )		!= NULL) 	continue;
		if(strstr(chaine, "BITMAP") 		!= NULL) 	continue;
		
		if(strstr(chaine, "STARTCHAR")	!= NULL) 	
		{
			printf("%s",replace_str(replace_str(chaine,"STARTCHAR ","_graphical_char_"),"\n",":")); 
			continue;
		}
		if(strstr(chaine, "ENCODING")	!= NULL) 	
		{
			printf("%s",replace_str(chaine,"ENCODING ","	;")); 
			continue;
		}
			
		char *convert(char caractere)
		{
			static char buffer[4];
			switch (caractere) {
				case '0':  strcpy(buffer,"0000"); break;
				case '1':  strcpy(buffer,"0001"); break;
				case '2':  strcpy(buffer,"0010"); break;
				case '3':  strcpy(buffer,"0011"); break;
				case '4':  strcpy(buffer,"0100"); break;
				case '5':  strcpy(buffer,"0101"); break;
				case '6':  strcpy(buffer,"0110"); break;
				case '7':  strcpy(buffer,"0111"); break;
				case '8':  strcpy(buffer,"1000"); break;
				case '9':  strcpy(buffer,"1001"); break;
				case 'A':  strcpy(buffer,"1010"); break;
				case 'B':  strcpy(buffer,"1011"); break;
				case 'C':  strcpy(buffer,"1100"); break;
				case 'D':  strcpy(buffer,"1101"); break;
				case 'E':  strcpy(buffer,"1110"); break;
				case 'F':  strcpy(buffer,"1111"); break;
			}
			return buffer;	
		}
		printf("db 0b%s",convert(chaine[0]));
		printf("%s\n",convert(chaine[1]));
	}
	fclose(input_File);
	return 0;
}
