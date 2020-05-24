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

#ifndef YY_SCR_SCRIPT_PARSER_H_INCLUDED
# define YY_SCR_SCRIPT_PARSER_H_INCLUDED
/* Debug traces.  */
#ifndef YYDEBUG
# define YYDEBUG 0
#endif
#if YYDEBUG
extern int scr_debug;
#endif

/* Token type.  */
#ifndef YYTOKENTYPE
# define YYTOKENTYPE
  enum yytokentype
  {
    FUNCTION = 258,
    TRIGGER = 259,
    EVENT = 260,
    WAIT = 261,
    EVERY = 262,
    INACTIVE = 263,
    INITIALISE = 264,
    LINK = 265,
    REF = 266,
    RET = 267,
    WHILE = 268,
    IF = 269,
    ELSE = 270,
    EXIT = 271,
    PAUSE = 272,
    BOOLEQUAL = 273,
    NOTEQUAL = 274,
    GREATEQUAL = 275,
    LESSEQUAL = 276,
    GREATER = 277,
    LESS = 278,
    _AND = 279,
    _OR = 280,
    _NOT = 281,
    UMINUS = 282,
    BOOLEAN_T = 283,
    INTEGER = 284,
    QTEXT = 285,
    TYPE = 286,
    STORAGE = 287,
    IDENT = 288,
    VAR = 289,
    BOOL_VAR = 290,
    NUM_VAR = 291,
    OBJ_VAR = 292,
    STRING_VAR = 293,
    VAR_ARRAY = 294,
    BOOL_ARRAY = 295,
    NUM_ARRAY = 296,
    OBJ_ARRAY = 297,
    BOOL_OBJVAR = 298,
    NUM_OBJVAR = 299,
    USER_OBJVAR = 300,
    OBJ_OBJVAR = 301,
    BOOL_CONSTANT = 302,
    NUM_CONSTANT = 303,
    USER_CONSTANT = 304,
    OBJ_CONSTANT = 305,
    STRING_CONSTANT = 306,
    FUNC = 307,
    BOOL_FUNC = 308,
    NUM_FUNC = 309,
    USER_FUNC = 310,
    OBJ_FUNC = 311,
    STRING_FUNC = 312,
    VOID_FUNC_CUST = 313,
    BOOL_FUNC_CUST = 314,
    NUM_FUNC_CUST = 315,
    USER_FUNC_CUST = 316,
    OBJ_FUNC_CUST = 317,
    STRING_FUNC_CUST = 318,
    TRIG_SYM = 319,
    EVENT_SYM = 320,
    CALLBACK_SYM = 321
  };
#endif
/* Tokens.  */
#define FUNCTION 258
#define TRIGGER 259
#define EVENT 260
#define WAIT 261
#define EVERY 262
#define INACTIVE 263
#define INITIALISE 264
#define LINK 265
#define REF 266
#define RET 267
#define WHILE 268
#define IF 269
#define ELSE 270
#define EXIT 271
#define PAUSE 272
#define BOOLEQUAL 273
#define NOTEQUAL 274
#define GREATEQUAL 275
#define LESSEQUAL 276
#define GREATER 277
#define LESS 278
#define _AND 279
#define _OR 280
#define _NOT 281
#define UMINUS 282
#define BOOLEAN_T 283
#define INTEGER 284
#define QTEXT 285
#define TYPE 286
#define STORAGE 287
#define IDENT 288
#define VAR 289
#define BOOL_VAR 290
#define NUM_VAR 291
#define OBJ_VAR 292
#define STRING_VAR 293
#define VAR_ARRAY 294
#define BOOL_ARRAY 295
#define NUM_ARRAY 296
#define OBJ_ARRAY 297
#define BOOL_OBJVAR 298
#define NUM_OBJVAR 299
#define USER_OBJVAR 300
#define OBJ_OBJVAR 301
#define BOOL_CONSTANT 302
#define NUM_CONSTANT 303
#define USER_CONSTANT 304
#define OBJ_CONSTANT 305
#define STRING_CONSTANT 306
#define FUNC 307
#define BOOL_FUNC 308
#define NUM_FUNC 309
#define USER_FUNC 310
#define OBJ_FUNC 311
#define STRING_FUNC 312
#define VOID_FUNC_CUST 313
#define BOOL_FUNC_CUST 314
#define NUM_FUNC_CUST 315
#define USER_FUNC_CUST 316
#define OBJ_FUNC_CUST 317
#define STRING_FUNC_CUST 318
#define TRIG_SYM 319
#define EVENT_SYM 320
#define CALLBACK_SYM 321

/* Value type.  */
#if ! defined YYSTYPE && ! defined YYSTYPE_IS_DECLARED
union YYSTYPE
{
#line 1624 "script_parser.y"

	/* Types returned by the lexer */
	BOOL			bval;
	/*	float			fval; */
	SDWORD			ival;
	STRING			*sval;
	INTERP_TYPE		tval;
	STORAGE_TYPE	stype;
	VAR_SYMBOL		*vSymbol;
	CONST_SYMBOL	*cSymbol;
	FUNC_SYMBOL		*fSymbol;
	TRIGGER_SYMBOL	*tSymbol;
	EVENT_SYMBOL	*eSymbol;
	CALLBACK_SYMBOL	*cbSymbol;

	/* Types only returned by rules */
	CODE_BLOCK		*cblock;
	COND_BLOCK		*condBlock;
	OBJVAR_BLOCK	*objVarBlock;
	ARRAY_BLOCK		*arrayBlock;
	PARAM_BLOCK		*pblock;
	PARAM_DECL		*pdecl;
	TRIGGER_DECL	*tdecl;
	UDWORD			integer_val;
	VAR_DECL		*vdecl;
	VAR_IDENT_DECL	*videcl;

#line 217 "script_parser.h"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE scr_lval;

int scr_parse (void);

#endif /* !YY_SCR_SCRIPT_PARSER_H_INCLUDED  */
