%{

#include <stdio.h>

/* include framework */
#include "lib/framework/frame.h"

#include "parser.h"

/* Get the Yacc definitions */
#include "audp_parser.h"

/* Maximum length for any TEXT value */
#define YYLMAX	255

/* global variables */
static	BOOL	g_bParsingSubFile;
static	FILE	*g_fpOld;

/* Pointer to the input buffer */
static char *pInputBuffer = NULL;
static char *pEndBuffer = NULL;

#define YY_INPUT(buf,result,max_size) \
			if (pInputBuffer != pEndBuffer) { \
				buf[0] = *(pInputBuffer++); result = 1; \
			} else { \
				buf[0] = EOF; result = YY_NULL; \
			}

void audp_error(char *pMessage,...);

%}

%option nounput
%option prefix="audp_"
%option yylineno

%x COMMENT
%x QUOTE

%%
	/* Match to key words */
oneshot							{	return ONESHOT;			}
loop							{	return LOOP;			}
audio							{	return AUDIO;			}
anim3dfile						{	return ANIM3DFILE;		}
audio_module					{	return AUDIO_MODULE;	}
anim_module						{	return ANIM_MODULE;		}
ANIM3DFRAMES					{	return ANIM3DFRAMES;	}
ANIM3DTRANS						{	return ANIM3DTRANS;		}
ANIMOBJECT						{	return ANIMOBJECT;		}

	/* Match floating point numbers */
	/* This is a problem with the PSX so is disabled
-?[0-9]*"."[0-9]+				{	audp_lval.fval = (float) atof(audp_text);
									return FLOAT_T;
								}
	*/

	/* Match integer numbers */
-?[0-9]+						{	audp_lval.ival = atoi(audp_text);
									return INTEGER;
								}

	/* Match quoted text */
\"[^\"\n]*[\"\n]				{
									/* skip opening quote */
									strcpy( audp_lval.sval, audp_text+1 );

									/* check for unterminated string */
									if ( audp_text[audp_leng-1] != '"' )
									{
										sprintf( audp_lval.sval, "Unterminated string %s\n", audp_lval.sval );
										audp_error( audp_lval.sval );
										return (1);
									}

									/* set final quote in string to blank */
									audp_lval.sval[audp_leng-2] = (char) NULL;

									return QTEXT;
								}

	/* Skip white space */
[ \t\n\x0d\x0a]						;

	/* Strip comments */
"/*"						{ BEGIN COMMENT; }
<COMMENT>"*/" |
<COMMENT>"*/"\n				{ BEGIN 0; }
<COMMENT>. |
<COMMENT>\n					;

	/* Match anything that's been missed and pass it as a char */
.							return audp_text[0];


%%

/***************************************************************************/

int
audp_wrap( void )
{
	if ( g_bParsingSubFile == TRUE )
	{
		/* close current file and restore old file pointer */
		fclose( audp_in );
		audp_in = g_fpOld;

		g_bParsingSubFile = FALSE;

		return 0;
	}
	else
	{
		return 1;
	}
}

/***************************************************************************/
/* Set the current input buffer for the lexer */
/***************************************************************************/

void parserSetInputBuffer(char *pBuffer, UDWORD size)
{
	pInputBuffer = pBuffer;
	pEndBuffer = pBuffer + size;
}

/***************************************************************************/

void
parseGetErrorData(int *pLine, char **ppText)
{
	*pLine  = audp_lineno;
	*ppText = audp_text;
}

/***************************************************************************/
