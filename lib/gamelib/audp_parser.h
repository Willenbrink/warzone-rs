/* A Bison parser, made by GNU Bison 3.5.4.  */

/* Bison interface for Yacc-like parsers in C

   Copyright (C) 1984, 1989-1990, 2000-2015, 2018-2020 Free Software Foundation,
   Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <http://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* Undocumented macros, especially those whose name start with YY_,
   are private implementation details.  Do not rely on them.  */

#ifndef YY_AUDP_AUDP_PARSER_H_INCLUDED
# define YY_AUDP_AUDP_PARSER_H_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int audp_debug;
#endif

/* Token type.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    FLOAT_T = 258,
    INTEGER = 259,
    QTEXT = 260,
    LOOP = 261,
    ONESHOT = 262,
    TEXT = 263,
    AUDIO = 264,
    ANIM3DFRAMES = 265,
    ANIM3DTRANS = 266,
    ANIM3DFILE = 267,
    AUDIO_MODULE = 268,
    ANIM_MODULE = 269,
    ANIMOBJECT = 270
  };
#endif
/* Tokens.  */
#define FLOAT_T 258
#define INTEGER 259
#define QTEXT 260
#define LOOP 261
#define ONESHOT 262
#define TEXT 263
#define AUDIO 264
#define ANIM3DFRAMES 265
#define ANIM3DTRANS 266
#define ANIM3DFILE 267
#define AUDIO_MODULE 268
#define ANIM_MODULE 269
#define ANIMOBJECT 270

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
#line 19 "audp_parser.y"

	float		fval;
	long		ival;
	signed char	bval;
	char		sval[100];

#line 94 "audp_parser.h"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE audp_lval;

int audp_parse (void);

#endif /* !YY_AUDP_AUDP_PARSER_H_INCLUDED  */
