/* A Bison parser, made by GNU Bison 3.5.4.  */

/* Bison implementation for Yacc-like parsers in C

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

/* C LALR(1) parser skeleton written by Richard Stallman, by
   simplifying the original so-called "semantic" parser.  */

/* All symbols defined below should begin with yy or YY, to avoid
   infringing on user name space.  This should be done even for local
   variables, as they might otherwise be expanded by user macros.
   There are some unavoidable exceptions within include files to
   define necessary library symbols; they are noted "INFRINGES ON
   USER NAME SPACE" below.  */

/* Undocumented macros, especially those whose name start with YY_,
   are private implementation details.  Do not rely on them.  */

/* Identify Bison output.  */
#define YYBISON 1

/* Bison version.  */
#define YYBISON_VERSION "3.5.4"

/* Skeleton name.  */
#define YYSKELETON_NAME "yacc.c"

/* Pure parsers.  */
#define YYPURE 0

/* Push parsers.  */
#define YYPUSH 0

/* Pull parsers.  */
#define YYPULL 1


/* Substitute the variable and function names.  */
#define yyparse         scr_parse
#define yylex           scr_lex
#define yyerror         scr_error
#define yydebug         scr_debug
#define yynerrs         scr_nerrs
#define yylval          scr_lval
#define yychar          scr_char

/* First part of user prologue.  */
#line 1 "script_parser.y"

/*
 * script.y
 *
 * The yacc grammar for the scipt files.
 */

#include <string.h>
#include <limits.h>
#include <stdio.h>

#include "lib/framework/frame.h"
#include "interp.h"
#include "parse.h"
#include "script.h"

extern int scr_lex(void);

/* Error return codes for code generation functions */
typedef enum _code_error
{
	CE_OK,				// No error
	CE_MEMORY,			// Out of memory
	CE_PARSE			// A parse error occured
} CODE_ERROR;

/* Turn off a couple of warnings that the yacc generated code gives */

/* Pointer to the compiled code */
static SCRIPT_CODE	*psFinalProg=NULL;

/* Pointer to current block of compiled code */
static CODE_BLOCK	*psCurrBlock=NULL;

/* Pointer to current block of conditional code */
static COND_BLOCK	*psCondBlock=NULL;

/* Pointer to current block of object variable code */
static OBJVAR_BLOCK	*psObjVarBlock=NULL;

/* Pointer to current block of compiled parameter code */
static PARAM_BLOCK	*psCurrPBlock=NULL;

//String support
//-----------------------------
char 				msg[MAXSTRLEN];
extern char			STRSTACK[MAXSTACKLEN][MAXSTRLEN];	// just a simple string "stack"
extern UDWORD		CURSTACKSTR;				//Current string index


/* Pointer into the current code block */
static UDWORD		*ip;

/* Pointer to current parameter declaration block */
//static PARAM_DECL	*psCurrParamDecl=NULL;

/* Pointer to current trigger subdeclaration */
static TRIGGER_DECL	*psCurrTDecl=NULL;

/* Pointer to current variable subdeclaration */
static VAR_DECL		*psCurrVDecl=NULL;

/* Pointer to the current variable identifier declaration */
static VAR_IDENT_DECL	*psCurrVIdentDecl=NULL;

/* Pointer to the current array access block */
static ARRAY_BLOCK		*psCurrArrayBlock=NULL;

/* Return code from code generation functions */
static CODE_ERROR	codeRet;

/* The list of global variables */
static VAR_SYMBOL	*psGlobalVars=NULL;

/* The list of global arrays */
static VAR_SYMBOL	*psGlobalArrays=NULL;

/* The list of current local variables */
static VAR_SYMBOL	*psLocalVars=NULL;

#define			maxEventsLocalVars		1200
static VAR_SYMBOL	*psLocalVarsB[maxEventsLocalVars];	/* local var storage */
static UDWORD		numEventLocalVars[maxEventsLocalVars];	/* number of declard local vars for each event */
static VAR_SYMBOL	*psLocalVarsTemp;			/* temporary storage for local vars, before current event declaration is found */
EVENT_SYMBOL		*psCurEvent = NULL;		/* stores current event: for local var declaration */

/* The list of function definitions */
static FUNC_SYMBOL	*psFunctions=NULL;

/* The current object variable context */
static INTERP_TYPE	objVarContext = (INTERP_TYPE)0;

/* Control whether debug info is generated */
static BOOL			genDebugInfo = TRUE;

/* Currently defined triggers */
static TRIGGER_SYMBOL	*psTriggers;
static UDWORD			numTriggers;

/* Currently defined events */
static EVENT_SYMBOL		*psEvents;
static UDWORD			numEvents;

/* This is true when local variables are being defined.
 * (So local variables can have the same name as global ones)
 */
static BOOL localVariableDef=FALSE;

/* The identifier for the current script function being defined */
//static STRING *pCurrFuncIdent=NULL;

/* A temporary store for a line number - used when
 * generating debugging info for functions, conditionals and loops.
 */
static UDWORD		debugLine;

/* The table of user types */
TYPE_SYMBOL		*asScrTypeTab;

/* The table of instinct function type definitions */
FUNC_SYMBOL		*asScrInstinctTab;

/* The table of external variables and their access functions */
VAR_SYMBOL		*asScrExternalTab;

/* The table of object variables and their access functions. */
VAR_SYMBOL		*asScrObjectVarTab;

/* The table of constant variables */
CONST_SYMBOL	*asScrConstantTab;

/* The table of callback triggers */
CALLBACK_SYMBOL	*asScrCallbackTab;

/****************************************************************************************
 *
 * Code Block Macros
 *
 * These macros are used to allocate and free the different types of code
 * block used within the compiler
 */


/* What the macro should do if it has an allocation error.
 * This is different depending on whether the macro is used
 * in a function, or in a rule body.
 *
 * This definition is used within the code generation functions
 * and is then changed for use within a rule body.
 */
#define ALLOC_ERROR_ACTION return CE_MEMORY

/* Macro to allocate a program structure */
#define ALLOC_PROG(psProg, codeSize, pAICode, numGlobs, numArys, numTrigs, numEvnts) \
	(psProg) = (SCRIPT_CODE *)MALLOC(sizeof(SCRIPT_CODE)); \
	if ((psProg) == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psProg)->pCode = (UDWORD *)MALLOC(codeSize); \
	if ((psProg)->pCode == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	if (numGlobs > 0) \
	{ \
		(psProg)->pGlobals = (INTERP_TYPE *)MALLOC(sizeof(INTERP_TYPE) * (numGlobs)); \
		if ((psProg)->pGlobals == NULL) \
		{ \
			debug(LOG_ERROR, "Out of memory"); \
			ALLOC_ERROR_ACTION; \
		} \
	} \
	else \
	{ \
		(psProg)->pGlobals = NULL; \
	} \
	if (numArys > 0) \
	{ \
		(psProg)->psArrayInfo = (ARRAY_DATA *)MALLOC(sizeof(ARRAY_DATA) * (numArys)); \
		if ((psProg)->psArrayInfo == NULL) \
		{ \
			debug(LOG_ERROR, "Out of memory"); \
			ALLOC_ERROR_ACTION; \
		} \
	} \
	else \
	{ \
		(psProg)->psArrayInfo = NULL; \
	} \
	(psProg)->numArrays = (UWORD)(numArys); \
	if ((numTrigs) > 0) \
	{ \
		(psProg)->pTriggerTab = MALLOC(sizeof(UWORD) * ((numTrigs) + 1)); \
		if ((psProg)->pTriggerTab == NULL) \
		{ \
			debug(LOG_ERROR, "Out of memory"); \
			ALLOC_ERROR_ACTION; \
		} \
		(psProg)->psTriggerData = MALLOC(sizeof(TRIGGER_DATA) * (numTrigs)); \
		if ((psProg)->psTriggerData == NULL) \
		{ \
			debug(LOG_ERROR, "Out of memory"); \
			ALLOC_ERROR_ACTION; \
		} \
	} \
	else \
	{ \
		(psProg)->pTriggerTab = NULL; \
		(psProg)->psTriggerData = NULL; \
	} \
	(psProg)->pEventTab = MALLOC(sizeof(UWORD) * ((numEvnts) + 1)); \
	if ((psProg)->pEventTab == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psProg)->pEventLinks = MALLOC(sizeof(SWORD) * (numEvnts)); \
	if ((psProg)->pEventLinks == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psProg)->numGlobals = (UWORD)(numGlobs); \
	(psProg)->numTriggers = (UWORD)(numTriggers); \
	(psProg)->numEvents = (UWORD)(numEvnts); \
	(psProg)->size = (codeSize)

/* Macro to allocate a code block */
#define ALLOC_BLOCK(psBlock, blockSize) \
	(psBlock) = (CODE_BLOCK *)MALLOC(sizeof(CODE_BLOCK)); \
	if ((psBlock) == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psBlock)->pCode = (UDWORD *)MALLOC(blockSize); \
	if ((psBlock)->pCode == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		FREE((psBlock)); \
		ALLOC_ERROR_ACTION; \
	} \
	(psBlock)->size = blockSize

/* Macro to free a code block */
#define FREE_BLOCK(psBlock) \
	FREE((psBlock)->pCode); \
	FREE((psBlock))

/* Macro to allocate a parameter block */
#define ALLOC_PBLOCK(psBlock, codeSize, paramSize) \
	(psBlock) = (PARAM_BLOCK *)MALLOC(sizeof(PARAM_BLOCK)); \
	if ((psBlock) == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psBlock)->pCode = (UDWORD *)MALLOC(codeSize); \
	if ((psBlock)->pCode == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		FREE((psBlock)); \
		ALLOC_ERROR_ACTION; \
	} \
	(psBlock)->aParams = (INTERP_TYPE *)MALLOC(sizeof(INTERP_TYPE) * (paramSize)); \
	if ((psBlock)->aParams == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		FREE((psBlock)->pCode); \
		FREE((psBlock)); \
		ALLOC_ERROR_ACTION; \
	} \
	(psBlock)->size = (codeSize); \
	(psBlock)->numParams = (paramSize)

/* Macro to free a parameter block */
#define FREE_PBLOCK(psBlock) \
	FREE((psBlock)->pCode); \
	FREE((psBlock)->aParams); \
	FREE((psBlock))

/* Macro to allocate a parameter declaration block */
#define ALLOC_PARAMDECL(psPDecl, num) \
	(psPDecl) = (PARAM_DECL *)MALLOC(sizeof(PARAM_DECL)); \
	if ((psPDecl) == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psPDecl)->aParams = (INTERP_TYPE *)MALLOC(sizeof(INTERP_TYPE) * (num)); \
	if ((psPDecl)->aParams == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psPDecl)->numParams = (num)

/* Macro to free a parameter declaration block */
#define FREE_PARAMDECL(psPDecl) \
	FREE((psPDecl)->aParams); \
	FREE((psPDecl))

/* Macro to allocate a conditional block */
#define ALLOC_CONDBLOCK(psCB, num, blockSize) \
	(psCB) = (COND_BLOCK *)MALLOC(sizeof(COND_BLOCK)); \
	if ((psCB) == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psCB)->aOffsets = (UDWORD *)MALLOC(sizeof(SDWORD) * (num)); \
	if ((psCB)->aOffsets == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psCB)->pCode = (UDWORD *)MALLOC(blockSize); \
	if ((psCB)->pCode == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psCB)->size = (blockSize); \
	(psCB)->numOffsets = (num)

/* Macro to free a conditional block */
#define FREE_CONDBLOCK(psCB) \
	FREE((psCB)->aOffsets); \
	FREE((psCB)->pCode); \
	FREE(psCB)

/* Macro to allocate a code block */
#define ALLOC_USERBLOCK(psBlock, blockSize) \
	(psBlock) = (USER_BLOCK *)MALLOC(sizeof(USER_BLOCK)); \
	if ((psBlock) == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psBlock)->pCode = (UDWORD *)MALLOC(blockSize); \
	if ((psBlock)->pCode == NULL) \
	{ \
		debug(LOG_ERROR, "Out of memory"); \
		FREE((psBlock)); \
		ALLOC_ERROR_ACTION; \
	} \
	(psBlock)->size = blockSize

/* Macro to free a code block */
#define FREE_USERBLOCK(psBlock) \
	FREE((psBlock)->pCode); \
	FREE((psBlock))

/* Macro to allocate an object variable block */
#define ALLOC_OBJVARBLOCK(psOV, blockSize, psVar) \
	(psOV) = (OBJVAR_BLOCK *)MALLOC(sizeof(OBJVAR_BLOCK)); \
	if ((psOV) == NULL) \
	{ \
		scr_error("Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psOV)->pCode = (UDWORD *)MALLOC(blockSize); \
	if ((psOV)->pCode == NULL) \
	{ \
		scr_error("Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psOV)->size = (blockSize); \
	(psOV)->psObjVar = (psVar)

/* Macro to free an object variable block */
#define FREE_OBJVARBLOCK(psOV) \
	FREE((psOV)->pCode); \
	FREE(psOV)

/* Macro to allocate an array variable block */
#define ALLOC_ARRAYBLOCK(psAV, blockSize, psVar) \
	(psAV) = (ARRAY_BLOCK *)MALLOC(sizeof(ARRAY_BLOCK)); \
	if ((psAV) == NULL) \
	{ \
		scr_error("Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psAV)->pCode = (UDWORD *)MALLOC(blockSize); \
	if ((psAV)->pCode == NULL) \
	{ \
		scr_error("Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psAV)->size = (blockSize); \
	(psAV)->dimensions = 1; \
	(psAV)->psArrayVar = (psVar)

/* Macro to free an object variable block */
#define FREE_ARRAYBLOCK(psAV) \
	FREE((psAV)->pCode); \
	FREE(psAV)

/* Allocate a trigger subdecl */
#define ALLOC_TSUBDECL(psTSub, blockType, blockSize, blockTime) \
	(psTSub) = MALLOC(sizeof(TRIGGER_DECL)); \
	if ((psTSub) == NULL) \
	{ \
		scr_error("Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	(psTSub)->type = (blockType); \
	(psTSub)->time = (blockTime); \
	if ((blockSize) > 0) \
	{ \
		(psTSub)->pCode = MALLOC(blockSize); \
		if ((psTSub)->pCode == NULL) \
		{ \
			scr_error("Out of memory"); \
			ALLOC_ERROR_ACTION; \
		} \
		(psTSub)->size = (blockSize); \
	} \
	else \
	{ \
		(psTSub)->pCode = NULL; \
		(psTSub)->size = 0; \
	}

/* Free a trigger subdecl */
#define FREE_TSUBDECL(psTSub) \
	if ((psTSub)->pCode) \
	{ \
		FREE((psTSub)->pCode); \
	} \
	FREE(psTSub)

/* Allocate a variable declaration block */
#define ALLOC_VARDECL(psDcl) \
	(psDcl)=MALLOC(sizeof(VAR_DECL)); \
	if ((psDcl) == NULL) \
	{ \
		scr_error("Out of memory"); \
		ALLOC_ERROR_ACTION; \
	}

/* Free a variable declaration block */
#define FREE_VARDECL(psDcl) \
	FREE(psDcl)

/* Allocate a variable declaration block */
#define ALLOC_VARIDENTDECL(psDcl, ident, dim) \
	(psDcl)=MALLOC(sizeof(VAR_IDENT_DECL)); \
	if ((psDcl) == NULL) \
	{ \
		scr_error("Out of memory"); \
		ALLOC_ERROR_ACTION; \
	} \
	if ((ident) != NULL) \
	{ \
		(psDcl)->pIdent=MALLOC(strlen(ident)+1); \
		if ((psDcl)->pIdent == NULL) \
		{ \
			scr_error("Out of memory"); \
			ALLOC_ERROR_ACTION; \
		} \
		strcpy((psDcl)->pIdent, (ident)); \
	} \
	else \
	{ \
		(psDcl)->pIdent = NULL; \
	} \
	(psDcl)->dimensions = (dim)

/* Free a variable declaration block */
#define FREE_VARIDENTDECL(psDcl) \
	FREE(psDcl)

/****************************************************************************************
 *
 * Code block manipulation macros.
 *
 * These are used to copy chunks of code from one block to another
 * or to insert opcodes or other values into a code block.
 * All the macros use the ip parameter.  This is a pointer into the code
 * block that is incremented by the macro. This ensures that it always points
 * to the next free space in the code block.
 */


/* Macro to store an opcode in a code block */
#define PUT_OPCODE(ip, opcode) \
	*ip = (opcode) << OPCODE_SHIFT; \
	ip += 1

/* Macro to put a packed opcode in a code block */
#define PUT_PKOPCODE(ip, opcode, data) \
	*ip = ((UDWORD)(data)) & OPCODE_DATAMASK; \
	*ip = (((UDWORD)(opcode)) << OPCODE_SHIFT) | (*ip); \
	ip += 1

/* Macro to store the data part of an INTERP_VAL in a code block */
#define PUT_DATA(ip, data) \
	*ip = (UDWORD)(data); \
	ip += 1

/* Macros to store a value in a code block */
#define PUT_BOOL(ip, value) \
	((INTERP_VAL *)(ip))->type = VAL_BOOL; \
	((INTERP_VAL *)(ip))->v.bval = (value); \
	((INTERP_VAL *)(ip)) += 1
#define PUT_INT(ip, value) \
	((INTERP_VAL *)(ip))->type = VAL_INT; \
	((INTERP_VAL *)(ip))->v.ival = (value); \
	((INTERP_VAL *)(ip)) += 1
/*#define PUT_FLOAT(ip, value) \
	((INTERP_VAL *)(ip))->type = VAL_FLOAT; \
	((INTERP_VAL *)(ip))->v.fval = (value); \
	((INTERP_VAL *)(ip)) += 1*/
#define PUT_STRING(ip, value) \
	((INTERP_VAL *)(ip))->type = VAL_STRING; \
	((INTERP_VAL *)(ip))->v.sval = (value); \
	((INTERP_VAL *)(ip)) += 1
/*#define PUT_OBJECT(ip, value) \
	((INTERP_VAL *)(ip))->type = VAL_OBJECT; \
	((INTERP_VAL *)(ip))->v.oval = (value); \
	((INTERP_VAL *)(ip)) += 1*/

/* Macro to store a function pointer in a code block */
#define PUT_FUNC(ip, func) \
	*ip = (UDWORD)func; \
	ip += 1

/* Macro to store a function pointer in a code block */
#define PUT_VARFUNC(ip, func) \
	*ip = (UDWORD)func; \
	ip += 1

/* Macro to store a reference to a script function. */
/* This will be converted to a program location at the link stage */
#define PUT_SCRIPTFUNC(ip, func) \
	*ip = (UDWORD)func; \
	ip += 1

/* Macro to store a variable index number in a code block */
#define PUT_INDEX(ip, index) \
	*ip = (index); \
	ip++

/* Macro to store a jump offset in a code block */
#define PUT_OFFSET(ip, offset) \
	*((SDWORD *)ip) = (offset); \
	ip++

/* Macro to copy a code block into another code block */
#define PUT_BLOCK(ip, psBlock) \
	memcpy(ip, (psBlock)->pCode, (psBlock)->size); \
	ip = (UDWORD *)(((UBYTE *)ip) + (psBlock)->size)

/***********************************************************************************
 *
 * Debugging information macros
 *
 * These macros are only used to generate debugging information for scripts.
 */

/* Macro to allocate debugging info for a CODE_BLOCK or a COND_BLOCK */
#define ALLOC_DEBUG(psBlock, num) \
	if (genDebugInfo) \
	{ \
		(psBlock)->psDebug = (SCRIPT_DEBUG *)MALLOC(sizeof(SCRIPT_DEBUG) * (num)); \
		if ((psBlock)->psDebug == NULL) \
		{ \
			scr_error("Out of memory"); \
			ALLOC_ERROR_ACTION; \
		} \
		memset((psBlock)->psDebug, 0, sizeof(SCRIPT_DEBUG) * (num));\
		(psBlock)->debugEntries = (UWORD)(num); \
	} \
	else \
	{ \
		(psBlock)->psDebug = NULL; \
		(psBlock)->debugEntries = 0; \
	}

/* Macro to free debugging info */
#define FREE_DEBUG(psBlock) \
	if (genDebugInfo) \
		FREE((psBlock)->psDebug)


/* Macro to copy the debugging information from one block to another */
#define PUT_DEBUG(psFinal, psBlock) \
	if (genDebugInfo) \
	{ \
		memcpy((psFinal)->psDebug, (psBlock)->psDebug, \
					sizeof(SCRIPT_DEBUG) * (psBlock)->debugEntries); \
		(psFinal)->debugEntries = (psBlock)->debugEntries; \
	}

/* Macro to combine the debugging information in two blocks into a third block */
static UDWORD		_dbEntry;
static SCRIPT_DEBUG	*_psCurr;
static UDWORD		_baseOffset;
#define COMBINE_DEBUG(psFinal, psBlock1, psBlock2) \
	if (genDebugInfo) \
	{ \
		memcpy((psFinal)->psDebug, (psBlock1)->psDebug, \
				 sizeof(SCRIPT_DEBUG) * (psBlock1)->debugEntries); \
		_baseOffset = (psBlock1)->size / sizeof(UDWORD); \
		for(_dbEntry = 0; _dbEntry < (psBlock2)->debugEntries; _dbEntry++) \
		{ \
			_psCurr = (psFinal)->psDebug + (psBlock1)->debugEntries + _dbEntry; \
			_psCurr->line = (psBlock2)->psDebug[_dbEntry].line; \
			_psCurr->offset = (psBlock2)->psDebug[_dbEntry].offset + _baseOffset; \
		} \
		(psFinal)->debugEntries = (psBlock1)->debugEntries + (psBlock2)->debugEntries; \
	}

/* Macro to append some debugging information onto a block, given the instruction
   offset of the debugging information already in the destination block */
#define APPEND_DEBUG(psFinal, baseOffset, psBlock) \
	if (genDebugInfo) \
	{ \
		for(_dbEntry = 0; _dbEntry < (psBlock)->debugEntries; _dbEntry++) \
		{ \
			_psCurr = (psFinal)->psDebug + (psFinal)->debugEntries + _dbEntry; \
			_psCurr->line = (psBlock)->psDebug[_dbEntry].line; \
			_psCurr->offset = (psBlock)->psDebug[_dbEntry].offset + (baseOffset); \
		} \
		(psFinal)->debugEntries = (UWORD)((psFinal)->debugEntries + (psBlock)->debugEntries); \
	}


/* Macro to store a label in the debug info */
#define DEBUG_LABEL(psBlock, offset, pString) \
	if (genDebugInfo) \
	{ \
		(psBlock)->psDebug[offset].pLabel = MALLOC(strlen(pString)+1); \
		if (!(psBlock)->psDebug[offset].pLabel) \
		{ \
			scr_error("Out of memory"); \
			ALLOC_ERROR_ACTION; \
		} \
		strcpy((psBlock)->psDebug[offset].pLabel, pString); \
	}


/***************************************************************************************
 *
 * Code generation functions
 *
 * These functions are used within rule bodies to generate code.
 */

/* Macro to deal with the errors returned by code generation functions.
 * Used within the rule body.
 */
#define CHECK_CODE_ERROR(error) \
	if ((error) == CE_MEMORY) \
	{ \
		YYABORT; \
	} \
	else if ((error) == CE_PARSE) \
	{ \
		YYERROR; \
	}



/* Generate the code for a function call, checking the parameter
 * types match.
 */
CODE_ERROR scriptCodeFunction(FUNC_SYMBOL		*psFSymbol,		// The function being called
							PARAM_BLOCK		*psPBlock,		// The functions parameters
							BOOL			expContext,		// Whether the function is being
															// called in an expression context
							CODE_BLOCK		**ppsCBlock)	// The generated code block
{
	UDWORD		size, i, *ip;
	BOOL		typeError = FALSE;
	STRING		aErrorString[255];

	ASSERT( psFSymbol != NULL, "ais_CodeFunction: Invalid function symbol pointer" );
	ASSERT( PTRVALID(psPBlock, sizeof(PARAM_BLOCK)),
		"scriptCodeFunction: Invalid param block pointer" );
	ASSERT( (psPBlock->size == 0) || PTRVALID(psPBlock->pCode, psPBlock->size),
		"scriptCodeFunction: Invalid parameter code pointer" );
	ASSERT( ppsCBlock != NULL,
		 "scriptCodeFunction: Invalid generated code block pointer" );

	/* Check the parameter types match what the function needs */
	for(i=0; (i<psFSymbol->numParams) && (i<psPBlock->numParams); i++)
	{
/*		if (psFSymbol->aParams[i] != VAL_VOID &&
			psFSymbol->aParams[i] != psPBlock->aParams[i])*/
		//TODO: string support
		if(psFSymbol->aParams[i] != VAL_STRING)	// string - allow mixed types if string is parameter type
		{

			if (!interpCheckEquiv(psFSymbol->aParams[i], psPBlock->aParams[i]))
			{
				debug(LOG_ERROR, "scriptCodeFunction: Type mismatch for paramter %d", i);
				sprintf(aErrorString, "Type mismatch for paramter %d", i);
				scr_error(aErrorString);
				typeError = TRUE;
			}
		}
		//else
		//{
		//	debug(LOG_SCRIPT, "scriptCodeFunction: %s takes string as parameter %d (provided: %d)", psFSymbol->pIdent, i, psPBlock->aParams[i]);
		//}
	}

	/* Check the number of parameters matches that expected */
	if (psFSymbol->numParams != psPBlock->numParams)
	{
		sprintf(aErrorString, "Expected %d parameters", psFSymbol->numParams);
		scr_error(aErrorString);
		*ppsCBlock = NULL;
		return CE_PARSE;
	}
	if (typeError)
	{
		/* Report the error here so all the */
		/* type mismatches are reported */
		*ppsCBlock = NULL;
		return CE_PARSE;
	}

	size = psPBlock->size + sizeof(OPCODE) + sizeof(SCRIPT_FUNC);
	if (!expContext && (psFSymbol->type != VAL_VOID))
	{
		size += sizeof(OPCODE);
	}

	ALLOC_BLOCK(*ppsCBlock, size);
	ip = (*ppsCBlock)->pCode;
	(*ppsCBlock)->type = psFSymbol->type;

	/* Copy in the code for the parameters */
	PUT_BLOCK(ip, psPBlock);
	FREE_PBLOCK(psPBlock);

	/* Make the function call */
	if (psFSymbol->script)
	{
		/* function defined in this script */
//		PUT_OPCODE(ip, OP_FUNC);
//		PUT_SCRIPTFUNC(ip, psFSymbol);
	}
	else
	{
		/* call an instinct function */
		PUT_OPCODE(ip, OP_CALL);
		PUT_FUNC(ip, psFSymbol->pFunc);
	}

	if (!expContext && (psFSymbol->type != VAL_VOID))
	{
		/* Clear the return value from the stack */
		PUT_OPCODE(ip, OP_POP);
	}

	return CE_OK;
}


/* Function call: Check the parameter types match, assumes param count matched */
UDWORD checkFuncParamTypes(EVENT_SYMBOL		*psFSymbol,		// The function being called
							PARAM_BLOCK		*psPBlock)	// The generated code block
{
	UDWORD		size, i, *ip;
	BOOL		typeError = FALSE;
	STRING		aErrorString[255];

	//debug(LOG_SCRIPT,"checkFuncParamTypes");

	/* Check the parameter types match what the function needs */
	for(i=0; (i<psFSymbol->numParams) && (i<psPBlock->numParams); i++)
	{
		//TODO: string support
		//if(psFSymbol->aParams[i] != VAL_STRING)	// string - allow mixed types if string is parameter type
		//{
			if (!interpCheckEquiv(psFSymbol->aParams[i], psPBlock->aParams[i]))
			{
				debug(LOG_ERROR, "checkFuncParamTypes: Type mismatch for paramter %d ('1' based) in Function '%s' (provided type: %d, expected: %d)", (i+1), psFSymbol->pIdent, psPBlock->aParams[i], psFSymbol->aParams[i]);
				return i+1;
			}
		//}
	}

	//debug(LOG_SCRIPT,"END checkFuncParamTypes");

	return 0;	//all ok
}

/*
 *  function call
 */
CODE_ERROR scriptCodeCallFunction(FUNC_SYMBOL	*psFSymbol,		// The function being called
						PARAM_BLOCK		*psPBlock,		// The functions parameters
						CODE_BLOCK		**ppsCBlock)	// The generated code block
{
	UDWORD		size, i, *ip;
	BOOL		typeError = FALSE;
	STRING		aErrorString[255];

	//debug(LOG_SCRIPT, "scriptCodeCallFunction");

	ASSERT( psFSymbol != NULL, "ais_CodeFunction: Invalid function symbol pointer" );
	ASSERT( PTRVALID(psPBlock, sizeof(PARAM_BLOCK)),
		"scriptCodeFunction: Invalid param block pointer" );
	ASSERT( (psPBlock->size == 0) || PTRVALID(psPBlock->pCode, psPBlock->size),
		"scriptCodeFunction: Invalid parameter code pointer" );
	ASSERT( ppsCBlock != NULL,
		 "scriptCodeFunction: Invalid generated code block pointer" );


	size = psPBlock->size + sizeof(OPCODE) + sizeof(SCRIPT_FUNC);

	ALLOC_BLOCK(*ppsCBlock, size);
	ip = (*ppsCBlock)->pCode;
	(*ppsCBlock)->type = psFSymbol->type;

	/* Copy in the code for the parameters */
	PUT_BLOCK(ip, psPBlock);
	FREE_PBLOCK(psPBlock);

	/* Make the function call */
	PUT_OPCODE(ip, OP_CALL);
	PUT_FUNC(ip, psFSymbol->pFunc);

	//debug(LOG_SCRIPT, "END scriptCodeCallFunction");

	return CE_OK;
}


/* Generate the code for a parameter callback, checking the parameter
 * types match.
 */
CODE_ERROR scriptCodeCallbackParams(
							CALLBACK_SYMBOL	*psCBSymbol,	// The callback being called
							PARAM_BLOCK		*psPBlock,		// The callbacks parameters
							TRIGGER_DECL	**ppsTDecl)		// The generated code block
{
	UDWORD		size, i, *ip;
	BOOL		typeError = FALSE;
	STRING		aErrorString[255];

	ASSERT( PTRVALID(psPBlock, sizeof(PARAM_BLOCK)),
		"scriptCodeCallbackParams: Invalid param block pointer" );
	ASSERT( (psPBlock->size == 0) || PTRVALID(psPBlock->pCode, psPBlock->size),
		"scriptCodeCallbackParams: Invalid parameter code pointer" );
	ASSERT( ppsTDecl != NULL,
		 "scriptCodeCallbackParams: Invalid generated code block pointer" );
	ASSERT( psCBSymbol->pFunc != NULL,
		 "scriptCodeCallbackParams: Expected function pointer for callback symbol" );

	/* Check the parameter types match what the function needs */
	for(i=0; (i<psCBSymbol->numParams) && (i<psPBlock->numParams); i++)
	{
		if (!interpCheckEquiv(psCBSymbol->aParams[i], psPBlock->aParams[i]))
		{
			sprintf(aErrorString, "Type mismatch for paramter %d", i);
			scr_error(aErrorString);
			typeError = TRUE;
		}
	}

	/* Check the number of parameters matches that expected */
	if (psPBlock->numParams == 0)
	{
		scr_error("Expected parameters to callback");
		*ppsTDecl = NULL;
		return CE_PARSE;
	}
	else if (psCBSymbol->numParams != psPBlock->numParams)
	{
		sprintf(aErrorString, "Expected %d parameters", psCBSymbol->numParams);
		scr_error(aErrorString);
		*ppsTDecl = NULL;
		return CE_PARSE;
	}
	if (typeError)
	{
		/* Return the error here so all the */
		/* type mismatches are reported */
		*ppsTDecl = NULL;
		return CE_PARSE;
	}

	size = psPBlock->size + sizeof(OPCODE) + sizeof(SCRIPT_FUNC);

	ALLOC_TSUBDECL(*ppsTDecl, psCBSymbol->type, size, 0);
	ip = (*ppsTDecl)->pCode;

	/* Copy in the code for the parameters */
	PUT_BLOCK(ip, psPBlock);
	FREE_PBLOCK(psPBlock);

	/* call the instinct function */
	PUT_OPCODE(ip, OP_CALL);
	PUT_FUNC(ip, psCBSymbol->pFunc);

	return CE_OK;
}


/* Generate code for assigning a value to a variable */
CODE_ERROR scriptCodeAssignment(VAR_SYMBOL	*psVariable,	// The variable to assign to
							  CODE_BLOCK	*psValue,		// The code for the value to
															// assign
							  CODE_BLOCK	**ppsBlock)		// Generated code
{
	SDWORD		size;

	ASSERT( psVariable != NULL,
		"scriptCodeAssignment: Invalid variable symbol pointer" );
	ASSERT( PTRVALID(psValue, sizeof(CODE_BLOCK)),
		"scriptCodeAssignment: Invalid value code block pointer" );
	ASSERT( PTRVALID(psValue->pCode, psValue->size),
		"scriptCodeAssignment: Invalid value code pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeAssignment: Invalid generated code block pointer" );

	size = psValue->size + sizeof(OPCODE);
	if (psVariable->storage == ST_EXTERN)
	{
		// Check there is a set function
		if (psVariable->set == NULL)
		{
			scr_error("No set function for external variable");
			return CE_PARSE;
		}
		size += sizeof(SCRIPT_VARFUNC);
	}
	ALLOC_BLOCK(*ppsBlock, size);
	ip = (*ppsBlock)->pCode;

	/* Copy in the code for the expression */
	PUT_BLOCK(ip, psValue);
	FREE_BLOCK(psValue);

	/* Code to get the value from the stack into the variable */
	switch (psVariable->storage)
	{
	case ST_PUBLIC:
	case ST_PRIVATE:
		PUT_PKOPCODE(ip, OP_POPGLOBAL, psVariable->index);
		break;
	case ST_LOCAL:
		PUT_PKOPCODE(ip, OP_POPLOCAL, psVariable->index);
		break;
	case ST_EXTERN:
		PUT_PKOPCODE(ip, OP_VARCALL, psVariable->index);
		PUT_VARFUNC(ip, psVariable->set);
		break;
	case ST_OBJECT:
		scr_error("Cannot use member variables in this context");
		return CE_PARSE;
		break;
	default:
		scr_error("Unknown storage type");
		return CE_PARSE;
		break;
	}

	return CE_OK;
}


/* Generate code for assigning a value to an object variable */
CODE_ERROR scriptCodeObjAssignment(OBJVAR_BLOCK	*psVariable,// The variable to assign to
								 CODE_BLOCK		*psValue,	// The code for the value to
															// assign
								 CODE_BLOCK		**ppsBlock)	// Generated code
{
	ASSERT( PTRVALID(psVariable, sizeof(OBJVAR_BLOCK)),
		"scriptCodeObjAssignment: Invalid object variable block pointer" );
	ASSERT( PTRVALID(psVariable->pCode, psVariable->size),
		"scriptCodeObjAssignment: Invalid object variable code pointer" );
	ASSERT( psVariable->psObjVar != NULL,
		"scriptCodeObjAssignment: Invalid object variable symbol pointer" );
	ASSERT( PTRVALID(psValue, sizeof(CODE_BLOCK)),
		"scriptCodeObjAssignment: Invalid value code block pointer" );
	ASSERT( PTRVALID(psValue->pCode, psValue->size),
		"scriptCodeObjAssignment: Invalid value code pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeObjAssignment: Invalid generated code block pointer" );

	// Check there is an access function for the variable
	if (psVariable->psObjVar->set == NULL)
	{
		scr_error("No set function for object variable");
		return CE_PARSE;
	}

	ALLOC_BLOCK(*ppsBlock, psVariable->size + psValue->size +
					sizeof(OPCODE) + sizeof(SCRIPT_VARFUNC));
	ip = (*ppsBlock)->pCode;

	/* Copy in the code for the value */
	PUT_BLOCK(ip, psValue);
	FREE_BLOCK(psValue);

	/* Copy in the code for the object */
	PUT_BLOCK(ip, psVariable);

	/* Code to get the value from the stack into the variable */
	PUT_PKOPCODE(ip, OP_VARCALL, psVariable->psObjVar->index);
	PUT_VARFUNC(ip, (psVariable->psObjVar->set));

	/* Free the variable block */
	FREE_OBJVARBLOCK(psVariable);

	return CE_OK;
}


/* Generate code for getting a value from an object variable */
CODE_ERROR scriptCodeObjGet(OBJVAR_BLOCK	*psVariable,// The variable to get from
						  CODE_BLOCK	**ppsBlock)	// Generated code
{
	ASSERT( PTRVALID(psVariable, sizeof(OBJVAR_BLOCK)),
		"scriptCodeObjAssignment: Invalid object variable block pointer" );
	ASSERT( PTRVALID(psVariable->pCode, psVariable->size),
		"scriptCodeObjAssignment: Invalid object variable code pointer" );
	ASSERT( psVariable->psObjVar != NULL,
		"scriptCodeObjAssignment: Invalid object variable symbol pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeObjAssignment: Invalid generated code block pointer" );

	// Check there is an access function for the variable
	if (psVariable->psObjVar->get == NULL)
	{
		scr_error("No get function for object variable");
		return CE_PARSE;
	}

	ALLOC_BLOCK(*ppsBlock, psVariable->size +
					sizeof(OPCODE) + sizeof(SCRIPT_VARFUNC));
	ip = (*ppsBlock)->pCode;

	/* Copy in the code for the object */
	PUT_BLOCK(ip, psVariable);
	(*ppsBlock)->type = psVariable->psObjVar->type;

	/* Code to get the value from the object onto the stack */
	PUT_PKOPCODE(ip, OP_VARCALL, psVariable->psObjVar->index);
	PUT_VARFUNC(ip, psVariable->psObjVar->get);

	/* Free the variable block */
	FREE_OBJVARBLOCK(psVariable);

	return CE_OK;
}


/* Generate code for assigning a value to an array variable */
CODE_ERROR scriptCodeArrayAssignment(ARRAY_BLOCK	*psVariable,// The variable to assign to
								 CODE_BLOCK		*psValue,	// The code for the value to
															// assign
								 CODE_BLOCK		**ppsBlock)	// Generated code
{
//	SDWORD		elementDWords, i;
//	UBYTE		*pElement;

	ASSERT( PTRVALID(psVariable, sizeof(ARRAY_BLOCK)),
		"scriptCodeObjAssignment: Invalid object variable block pointer" );
	ASSERT( PTRVALID(psVariable->pCode, psVariable->size),
		"scriptCodeObjAssignment: Invalid object variable code pointer" );
	ASSERT( psVariable->psArrayVar != NULL,
		"scriptCodeObjAssignment: Invalid object variable symbol pointer" );
	ASSERT( PTRVALID(psValue, sizeof(CODE_BLOCK)),
		"scriptCodeObjAssignment: Invalid value code block pointer" );
	ASSERT( PTRVALID(psValue->pCode, psValue->size),
		"scriptCodeObjAssignment: Invalid value code pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeObjAssignment: Invalid generated code block pointer" );

	// Check this is an array
	if (psVariable->psArrayVar->dimensions == 0)
	{
		scr_error("Not an array variable");
		return CE_PARSE;
	}

	// calculate the number of DWORDs needed to store the number of elements for each dimension of the array
//	elementDWords = (psVariable->psArrayVar->dimensions - 1)/4 + 1;

//	ALLOC_BLOCK(*ppsBlock, psVariable->size + psValue->size + sizeof(OPCODE) + elementDWords*4);
	ALLOC_BLOCK(*ppsBlock, psVariable->size + psValue->size + sizeof(OPCODE));
	ip = (*ppsBlock)->pCode;

	/* Copy in the code for the value */
	PUT_BLOCK(ip, psValue);
	FREE_BLOCK(psValue);

	/* Copy in the code for the array index */
	PUT_BLOCK(ip, psVariable);

	/* Code to get the value from the stack into the variable */
	PUT_PKOPCODE(ip, OP_POPARRAYGLOBAL,
		((psVariable->psArrayVar->dimensions << ARRAY_DIMENSION_SHIFT) & ARRAY_DIMENSION_MASK) |
		(psVariable->psArrayVar->index & ARRAY_BASE_MASK) );

	// store the size of each dimension
/*	pElement = (UBYTE *)ip;
	for(i=0; i<psVariable->psArrayVar->dimensions; i++)
	{
		*pElement = (UBYTE)psVariable->psArrayVar->elements[i];
		pElement += 1;
	}*/

	/* Free the variable block */
	FREE_ARRAYBLOCK(psVariable);

	return CE_OK;
}


/* Generate code for getting a value from an array variable */
CODE_ERROR scriptCodeArrayGet(ARRAY_BLOCK	*psVariable,// The variable to get from
						  CODE_BLOCK	**ppsBlock)	// Generated code
{
//	SDWORD		elementDWords, i;
//	UBYTE		*pElement;

	ASSERT( PTRVALID(psVariable, sizeof(ARRAY_BLOCK)),
		"scriptCodeObjAssignment: Invalid object variable block pointer" );
	ASSERT( PTRVALID(psVariable->pCode, psVariable->size),
		"scriptCodeObjAssignment: Invalid object variable code pointer" );
	ASSERT( psVariable->psArrayVar != NULL,
		"scriptCodeObjAssignment: Invalid object variable symbol pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeObjAssignment: Invalid generated code block pointer" );

	// Check this is an array
	if (psVariable->psArrayVar->dimensions == 0)
	{
		scr_error("Not an array variable");
		return CE_PARSE;
	}

	// calculate the number of DWORDs needed to store the number of elements for each dimension of the array
//	elementDWords = (psVariable->psArrayVar->dimensions - 1)/4 + 1;

//	ALLOC_BLOCK(*ppsBlock, psVariable->size + sizeof(OPCODE) + elementDWords*4);
	ALLOC_BLOCK(*ppsBlock, psVariable->size + sizeof(OPCODE));
	ip = (*ppsBlock)->pCode;

	/* Copy in the code for the array index */
	PUT_BLOCK(ip, psVariable);
	(*ppsBlock)->type = psVariable->psArrayVar->type;

	/* Code to get the value from the array onto the stack */
	PUT_PKOPCODE(ip, OP_PUSHARRAYGLOBAL,
		((psVariable->psArrayVar->dimensions << ARRAY_DIMENSION_SHIFT) & ARRAY_DIMENSION_MASK) |
		(psVariable->psArrayVar->index & ARRAY_BASE_MASK) );

	// store the size of each dimension
/*	pElement = (UBYTE *)ip;
	for(i=0; i<psVariable->psArrayVar->dimensions; i++)
	{
		*pElement = (UBYTE)psVariable->psArrayVar->elements[i];
		pElement += 1;
	}*/

	/* Free the variable block */
	FREE_ARRAYBLOCK(psVariable);

	return CE_OK;
}


/* Generate the final code block for conditional statements */
CODE_ERROR scriptCodeConditional(
					COND_BLOCK *psCondBlock,	// The intermediate conditional code
					CODE_BLOCK **ppsBlock)		// The final conditional code
{
	UDWORD		i;

	ASSERT( PTRVALID(psCondBlock, sizeof(CODE_BLOCK)),
		"scriptCodeConditional: Invalid conditional code block pointer" );
	ASSERT( PTRVALID(psCondBlock->pCode, psCondBlock->size),
		"scriptCodeConditional: Invalid conditional code pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeConditional: Invalid generated code block pointer" );

	/* Allocate the final block */
	ALLOC_BLOCK(*ppsBlock, psCondBlock->size);
	ALLOC_DEBUG(*ppsBlock, psCondBlock->debugEntries);
	ip = (*ppsBlock)->pCode;

	/* Copy the code over */
	PUT_BLOCK(ip, psCondBlock);

	/* Copy the debugging information */
	PUT_DEBUG(*ppsBlock, psCondBlock);

	/* Now set the offsets of jumps in the conditional to the correct value */
	for(i = 0; i < psCondBlock->numOffsets; i++)
	{
		ip = (*ppsBlock)->pCode + psCondBlock->aOffsets[i];
		*ip = ((*ppsBlock)->size / sizeof(UDWORD)) - (ip - (*ppsBlock)->pCode);
		*ip = (OP_JUMP << OPCODE_SHIFT) | ( (*ip) & OPCODE_DATAMASK );
	}

	/* Free the original code */
	FREE_DEBUG(psCondBlock);
	FREE_CONDBLOCK(psCondBlock);

	return CE_OK;
}

/* Generate code for function parameters */
CODE_ERROR scriptCodeParameter(CODE_BLOCK		*psParam,		// Code for the parameter
							 INTERP_TYPE		type,			// Parameter type
							 PARAM_BLOCK	**ppsBlock)		// Generated code
{
	ASSERT( PTRVALID(psParam, sizeof(CODE_BLOCK)),
		"scriptCodeParameter: Invalid parameter code block pointer" );
	ASSERT( PTRVALID(psParam->pCode, psParam->size),
		"scriptCodeParameter: Invalid parameter code pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeParameter: Invalid generated code block pointer" );

	ALLOC_PBLOCK(*ppsBlock, psParam->size, 1);
	ip = (*ppsBlock)->pCode;

	/* Copy in the code for the parameter */
	PUT_BLOCK(ip, psParam);
	FREE_BLOCK(psParam);

	(*ppsBlock)->aParams[0] = type;

	return CE_OK;
}


/* Generate code for binary operators (e.g. 2 + 2) */
CODE_ERROR scriptCodeBinaryOperator(CODE_BLOCK	*psFirst,	// Code for first parameter
								  CODE_BLOCK	*psSecond,	// Code for second parameter
								  OPCODE		opcode,		// Operator function
								  CODE_BLOCK	**ppsBlock) // Generated code
{
	ALLOC_BLOCK(*ppsBlock, psFirst->size + psSecond->size + sizeof(UDWORD));
	ip = (*ppsBlock)->pCode;

	/* Copy the already generated bits of code into the code block */
	PUT_BLOCK(ip, psFirst);
	PUT_BLOCK(ip, psSecond);

	/* Now put an add operator into the code */
	PUT_PKOPCODE(ip, OP_BINARYOP, opcode);

	/* Free the two code blocks that have been copied */
	FREE_BLOCK(psFirst);
	FREE_BLOCK(psSecond);

	return CE_OK;
}

/* check if the arguments in the function definition body match the argument types
and names from function declaration (if there was any) */
BOOL checkFuncParamType(SDWORD argIndex, SDWORD argType)
{
	VAR_SYMBOL		*psCurr;
	SDWORD			i,j;

	if(psCurEvent == NULL)
	{
		debug(LOG_ERROR, "checkFuncParamType() - psCurEvent == NULL");
		return FALSE;
	}

	if(argIndex < psCurEvent->numParams)
	{
		/* find the argument by the index */
		i=psCurEvent->index;
		j=0;
		for(psCurr =psLocalVarsB[i]; psCurr != NULL; psCurr = psCurr->psNext)
		{
			if((psCurEvent->numParams - j - 1)==argIndex)	/* got to the right argument */
			{
				if(argType != psCurr->type)
				{
					debug(LOG_ERROR, "Argument type with index %d in event '%s' doesn't match function declaration (%d/%d)",argIndex,psCurEvent->pIdent,argType,psCurr->type);
					return FALSE;
				}
				else
				{
					//debug(LOG_SCRIPT, "arg matched ");
					return TRUE;
				}
			}
			j++;
		}
	}
	else
	{
		debug(LOG_ERROR, "checkFuncParamType() - argument %d has wrong argument index, event: '%s'", argIndex, psCurEvent->pIdent);
		return FALSE;
	}

	return FALSE;
}


/* Generate code for accessing an object variable.  The variable symbol is
 * stored with the code for the object value so that this block can be used for
 * both setting and retrieving an object value.
 */
CODE_ERROR scriptCodeObjectVariable(CODE_BLOCK	*psObjCode,	// Code for the object value
								  VAR_SYMBOL	*psVar,		// The object variable symbol
								  OBJVAR_BLOCK	**ppsBlock) // Generated code
{
	ASSERT( PTRVALID(psObjCode, sizeof(CODE_BLOCK)),
		"scriptCodeObjectVariable: Invalid object code block pointer" );
	ASSERT( PTRVALID(psObjCode->pCode, psObjCode->size),
		"scriptCodeObjectVariable: Invalid object code pointer" );
	ASSERT( psVar != NULL,
		"scriptCodeObjectVariable: Invalid variable symbol pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeObjectVariable: Invalid generated code block pointer" );

	ALLOC_OBJVARBLOCK(*ppsBlock, psObjCode->size, psVar);
	ip = (*ppsBlock)->pCode;

	/* Copy the already generated bit of code into the code block */
	PUT_BLOCK(ip, psObjCode);
	FREE_BLOCK(psObjCode);

	/* Check the variable is the correct type */
	if (psVar->storage != ST_OBJECT)
	{
		scr_error("Only object variables are valid in this context");
		return CE_PARSE;
	}

	return CE_OK;
}


/* Generate code for accessing an array variable.  The variable symbol is
 * stored with the code for the object value so that this block can be used for
 * both setting and retrieving an array value.
 */
CODE_ERROR scriptCodeArrayVariable(ARRAY_BLOCK	*psArrayCode,	// Code for the array index
								  VAR_SYMBOL	*psVar,			// The array variable symbol
								  ARRAY_BLOCK	**ppsBlock)		// Generated code
{
	ASSERT( PTRVALID(psArrayCode, sizeof(CODE_BLOCK)),
		"scriptCodeObjectVariable: Invalid object code block pointer" );
	ASSERT( PTRVALID(psArrayCode->pCode, psArrayCode->size),
		"scriptCodeObjectVariable: Invalid object code pointer" );
	ASSERT( psVar != NULL,
		"scriptCodeObjectVariable: Invalid variable symbol pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeObjectVariable: Invalid generated code block pointer" );

/*	ALLOC_ARRAYBLOCK(*ppsBlock, psArrayCode->size, psVar);
	ip = (*ppsBlock)->pCode;

	// Copy the already generated bit of code into the code block
	PUT_BLOCK(ip, psArrayCode);
	FREE_BLOCK(psArrayCode);*/

	// Check the variable is the correct type
	if (psVar->dimensions != psArrayCode->dimensions)
	{
		scr_error("Invalid number of array dimensions for this variable");
		return CE_PARSE;
	}

	psArrayCode->psArrayVar = psVar;
	*ppsBlock = psArrayCode;

	return CE_OK;
}


/* Generate code for a constant */
CODE_ERROR scriptCodeConstant(CONST_SYMBOL	*psConst,	// The object variable symbol
							CODE_BLOCK		**ppsBlock)	// Generated code
{
	ASSERT( psConst != NULL,
		"scriptCodeConstant: Invalid constant symbol pointer" );
	ASSERT( ppsBlock != NULL,
		"scriptCodeConstant: Invalid generated code block pointer" );

	ALLOC_BLOCK(*ppsBlock, sizeof(OPCODE) + sizeof(UDWORD));
	ip = (*ppsBlock)->pCode;
	(*ppsBlock)->type = psConst->type;

	/* Put the value onto the stack */
	switch (psConst->type)
	{
	case VAL_BOOL:
		PUT_PKOPCODE(ip, OP_PUSH, VAL_BOOL);
		PUT_DATA(ip, psConst->bval);
		break;
	case VAL_INT:
		PUT_PKOPCODE(ip, OP_PUSH, VAL_INT);
		PUT_DATA(ip, psConst->ival);
		break;
	case VAL_STRING:
		PUT_PKOPCODE(ip, OP_PUSH, VAL_STRING);
		PUT_DATA(ip, psConst->sval);
		break;
	default:
		PUT_PKOPCODE(ip, OP_PUSH, psConst->type);
		PUT_DATA(ip, psConst->oval);
		break;
	}

	return CE_OK;
}


/* Generate code for getting a variables value */
CODE_ERROR scriptCodeVarGet(VAR_SYMBOL		*psVariable,	// The object variable symbol
							CODE_BLOCK		**ppsBlock)		// Generated code
{
	SDWORD	size;

	size = sizeof(OPCODE);
	if (psVariable->storage == ST_EXTERN)
	{
		// Check there is a set function
		if (psVariable->get == NULL)
		{
			scr_error("No get function for external variable");
			return CE_PARSE;
		}
		size += sizeof(SCRIPT_VARFUNC);
	}
	ALLOC_BLOCK(*ppsBlock, size);
	ip = (*ppsBlock)->pCode;
	(*ppsBlock)->type = psVariable->type;

	/* Code to get the value onto the stack */
	switch (psVariable->storage)
	{
	case ST_PUBLIC:
	case ST_PRIVATE:
		PUT_PKOPCODE(ip, OP_PUSHGLOBAL, psVariable->index);
		break;

	case ST_LOCAL:
		PUT_PKOPCODE(ip, OP_PUSHLOCAL, psVariable->index);	//opcode + event index
		break;

	case ST_EXTERN:
		PUT_PKOPCODE(ip, OP_VARCALL, psVariable->index);
		PUT_VARFUNC(ip, psVariable->get);
		break;
	case ST_OBJECT:
		scr_error("Cannot use member variables in this context");
		return CE_PARSE;
		break;
	default:
		scr_error("Unknown storage type");
		return CE_PARSE;
		break;
	}

	return CE_OK;
}


/* Generate code for getting a variables value */
CODE_ERROR scriptCodeVarRef(VAR_SYMBOL		*psVariable,	// The object variable symbol
							PARAM_BLOCK		**ppsBlock)		// Generated code
{
	SDWORD	size;

	size = sizeof(OPCODE) + sizeof(SDWORD);
	ALLOC_PBLOCK(*ppsBlock, size, 1);
	ip = (*ppsBlock)->pCode;

	(*ppsBlock)->aParams[0] = psVariable->type | VAL_REF;

	/* Code to get the value onto the stack */
	switch (psVariable->storage)
	{
	case ST_PUBLIC:
	case ST_PRIVATE:

		PUT_PKOPCODE(ip, OP_PUSHREF, (*ppsBlock)->aParams[0]);
		PUT_DATA(ip, psVariable->index);
		break;

	case ST_LOCAL:
		PUT_PKOPCODE(ip, OP_PUSHLOCALREF, (*ppsBlock)->aParams[0]);
		PUT_DATA(ip, psVariable->index);
		break;
	case ST_EXTERN:
		scr_error("Cannot use external variables in this context");
		return CE_PARSE;
		break;
	case ST_OBJECT:
		scr_error("Cannot use member variables in this context");
		return CE_PARSE;
		break;
	default:
		scr_error("Unknown storage type: %d", psVariable->storage);
		return CE_PARSE;
		break;
	}

	return CE_OK;
}


/* Generate the code for a trigger and store it in the trigger list */
CODE_ERROR scriptCodeTrigger(STRING *pIdent, CODE_BLOCK *psCode)
{
	CODE_BLOCK		*psNewBlock;
	UDWORD			line;
	STRING			*pDummy;

	pIdent = pIdent;

	// Have to add the exit code to the end of the event
	ALLOC_BLOCK(psNewBlock, psCode->size + sizeof(OPCODE));
	ip = psNewBlock->pCode;
	PUT_BLOCK(ip, psCode);
	PUT_OPCODE(ip, OP_EXIT);

	// Add the debug info
	ALLOC_DEBUG(psNewBlock, psCode->debugEntries + 1);
	PUT_DEBUG(psNewBlock, psCode);
	if (genDebugInfo)
	{
		/* Add debugging info for the EXIT instruction */
		scriptGetErrorData((SDWORD *)&line, &pDummy);
		psNewBlock->psDebug[psNewBlock->debugEntries].line = line;
		psNewBlock->psDebug[psNewBlock->debugEntries].offset =
				ip - psNewBlock->pCode;
		psNewBlock->debugEntries ++;
	}
	FREE_BLOCK(psCode);

	// Create the trigger
/*	if (!scriptAddTrigger(pIdent, psNewBlock))
	{
		return CE_MEMORY;
	}*/

	return CE_OK;
}


/* Generate the code for an event and store it in the event list */
CODE_ERROR scriptCodeEvent(EVENT_SYMBOL *psEvent, TRIGGER_SYMBOL *psTrig, CODE_BLOCK *psCode)
{
	CODE_BLOCK		*psNewBlock;
	UDWORD			line;
	STRING			*pDummy;

	// Have to add the exit code to the end of the event
	ALLOC_BLOCK(psNewBlock, psCode->size + sizeof(OPCODE));
	ip = psNewBlock->pCode;
	PUT_BLOCK(ip, psCode);
	PUT_OPCODE(ip, OP_EXIT);

	// Add the debug info
	ALLOC_DEBUG(psNewBlock, psCode->debugEntries + 1);
	PUT_DEBUG(psNewBlock, psCode);
	if (genDebugInfo)
	{
		/* Add debugging info for the EXIT instruction */
		scriptGetErrorData((SDWORD *)&line, &pDummy);
		psNewBlock->psDebug[psNewBlock->debugEntries].line = line;
		psNewBlock->psDebug[psNewBlock->debugEntries].offset =
				ip - psNewBlock->pCode;
		psNewBlock->debugEntries ++;
	}
	FREE_BLOCK(psCode);

	// Create the event
	if (!scriptDefineEvent(psEvent, psNewBlock, psTrig->index))
	{
		return CE_MEMORY;
	}

	return CE_OK;
}


/* Store the types of a list of variables into a code block.
 * The order of the list is reversed so that the type of the
 * first variable defined is stored first.
 */
static void scriptStoreVarTypes(VAR_SYMBOL *psVar)
{
	if (psVar != NULL)
	{
		/* Recurse down the list to get to the end of it */
		scriptStoreVarTypes(psVar->psNext);

		/* Now store the current variable */
		PUT_INDEX(ip, psVar->type);
	}
}


/* Change the error action for the ALLOC macro's to what it
 * should be inside a rule body.
 *
 * NOTE: DO NOT USE THE ALLOC MACRO'S INSIDE ANY FUNCTIONS
 *       ONCE ALLOC_ERROR_ACTION IS SET TO THIS VALUE.
 *       ALL FUNCTIONS THAT USE THESE MACROS MUST BE PLACED
 *       BEFORE THIS #define.
 */
#undef ALLOC_ERROR_ACTION
#define ALLOC_ERROR_ACTION YYABORT


#line 1698 "script_parser.c"

# ifndef YY_CAST
#  ifdef __cplusplus
#   define YY_CAST(Type, Val) static_cast<Type> (Val)
#   define YY_REINTERPRET_CAST(Type, Val) reinterpret_cast<Type> (Val)
#  else
#   define YY_CAST(Type, Val) ((Type) (Val))
#   define YY_REINTERPRET_CAST(Type, Val) ((Type) (Val))
#  endif
# endif
# ifndef YY_NULLPTR
#  if defined __cplusplus
#   if 201103L <= __cplusplus
#    define YY_NULLPTR nullptr
#   else
#    define YY_NULLPTR 0
#   endif
#  else
#   define YY_NULLPTR ((void*)0)
#  endif
# endif

/* Enabling verbose error messages.  */
#ifdef YYERROR_VERBOSE
# undef YYERROR_VERBOSE
# define YYERROR_VERBOSE 1
#else
# define YYERROR_VERBOSE 0
#endif

/* Use api.header.include to #include this header
   instead of duplicating it here.  */
#ifndef YY_SCR_Y_TAB_H_INCLUDED
# define YY_SCR_Y_TAB_H_INCLUDED
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

#line 1910 "script_parser.c"

};
typedef union YYSTYPE YYSTYPE;
# define YYSTYPE_IS_TRIVIAL 1
# define YYSTYPE_IS_DECLARED 1
#endif


extern YYSTYPE scr_lval;

int scr_parse (void);

#endif /* !YY_SCR_Y_TAB_H_INCLUDED  */



#ifdef short
# undef short
#endif

/* On compilers that do not define __PTRDIFF_MAX__ etc., make sure
   <limits.h> and (if available) <stdint.h> are included
   so that the code can choose integer types of a good width.  */

#ifndef __PTRDIFF_MAX__
# include <limits.h> /* INFRINGES ON USER NAME SPACE */
# if defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stdint.h> /* INFRINGES ON USER NAME SPACE */
#  define YY_STDINT_H
# endif
#endif

/* Narrow types that promote to a signed type and that can represent a
   signed or unsigned integer of at least N bits.  In tables they can
   save space and decrease cache pressure.  Promoting to a signed type
   helps avoid bugs in integer arithmetic.  */

#ifdef __INT_LEAST8_MAX__
typedef __INT_LEAST8_TYPE__ yytype_int8;
#elif defined YY_STDINT_H
typedef int_least8_t yytype_int8;
#else
typedef signed char yytype_int8;
#endif

#ifdef __INT_LEAST16_MAX__
typedef __INT_LEAST16_TYPE__ yytype_int16;
#elif defined YY_STDINT_H
typedef int_least16_t yytype_int16;
#else
typedef short yytype_int16;
#endif

#if defined __UINT_LEAST8_MAX__ && __UINT_LEAST8_MAX__ <= __INT_MAX__
typedef __UINT_LEAST8_TYPE__ yytype_uint8;
#elif (!defined __UINT_LEAST8_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST8_MAX <= INT_MAX)
typedef uint_least8_t yytype_uint8;
#elif !defined __UINT_LEAST8_MAX__ && UCHAR_MAX <= INT_MAX
typedef unsigned char yytype_uint8;
#else
typedef short yytype_uint8;
#endif

#if defined __UINT_LEAST16_MAX__ && __UINT_LEAST16_MAX__ <= __INT_MAX__
typedef __UINT_LEAST16_TYPE__ yytype_uint16;
#elif (!defined __UINT_LEAST16_MAX__ && defined YY_STDINT_H \
       && UINT_LEAST16_MAX <= INT_MAX)
typedef uint_least16_t yytype_uint16;
#elif !defined __UINT_LEAST16_MAX__ && USHRT_MAX <= INT_MAX
typedef unsigned short yytype_uint16;
#else
typedef int yytype_uint16;
#endif

#ifndef YYPTRDIFF_T
# if defined __PTRDIFF_TYPE__ && defined __PTRDIFF_MAX__
#  define YYPTRDIFF_T __PTRDIFF_TYPE__
#  define YYPTRDIFF_MAXIMUM __PTRDIFF_MAX__
# elif defined PTRDIFF_MAX
#  ifndef ptrdiff_t
#   include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  endif
#  define YYPTRDIFF_T ptrdiff_t
#  define YYPTRDIFF_MAXIMUM PTRDIFF_MAX
# else
#  define YYPTRDIFF_T long
#  define YYPTRDIFF_MAXIMUM LONG_MAX
# endif
#endif

#ifndef YYSIZE_T
# ifdef __SIZE_TYPE__
#  define YYSIZE_T __SIZE_TYPE__
# elif defined size_t
#  define YYSIZE_T size_t
# elif defined __STDC_VERSION__ && 199901 <= __STDC_VERSION__
#  include <stddef.h> /* INFRINGES ON USER NAME SPACE */
#  define YYSIZE_T size_t
# else
#  define YYSIZE_T unsigned
# endif
#endif

#define YYSIZE_MAXIMUM                                  \
  YY_CAST (YYPTRDIFF_T,                                 \
           (YYPTRDIFF_MAXIMUM < YY_CAST (YYSIZE_T, -1)  \
            ? YYPTRDIFF_MAXIMUM                         \
            : YY_CAST (YYSIZE_T, -1)))

#define YYSIZEOF(X) YY_CAST (YYPTRDIFF_T, sizeof (X))

/* Stored state numbers (used for stacks). */
typedef yytype_int16 yy_state_t;

/* State numbers in computations.  */
typedef int yy_state_fast_t;

#ifndef YY_
# if defined YYENABLE_NLS && YYENABLE_NLS
#  if ENABLE_NLS
#   include <libintl.h> /* INFRINGES ON USER NAME SPACE */
#   define YY_(Msgid) dgettext ("bison-runtime", Msgid)
#  endif
# endif
# ifndef YY_
#  define YY_(Msgid) Msgid
# endif
#endif

#ifndef YY_ATTRIBUTE_PURE
# if defined __GNUC__ && 2 < __GNUC__ + (96 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_PURE __attribute__ ((__pure__))
# else
#  define YY_ATTRIBUTE_PURE
# endif
#endif

#ifndef YY_ATTRIBUTE_UNUSED
# if defined __GNUC__ && 2 < __GNUC__ + (7 <= __GNUC_MINOR__)
#  define YY_ATTRIBUTE_UNUSED __attribute__ ((__unused__))
# else
#  define YY_ATTRIBUTE_UNUSED
# endif
#endif

/* Suppress unused-variable warnings by "using" E.  */
#if ! defined lint || defined __GNUC__
# define YYUSE(E) ((void) (E))
#else
# define YYUSE(E) /* empty */
#endif

#if defined __GNUC__ && ! defined __ICC && 407 <= __GNUC__ * 100 + __GNUC_MINOR__
/* Suppress an incorrect diagnostic about yylval being uninitialized.  */
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN                            \
    _Pragma ("GCC diagnostic push")                                     \
    _Pragma ("GCC diagnostic ignored \"-Wuninitialized\"")              \
    _Pragma ("GCC diagnostic ignored \"-Wmaybe-uninitialized\"")
# define YY_IGNORE_MAYBE_UNINITIALIZED_END      \
    _Pragma ("GCC diagnostic pop")
#else
# define YY_INITIAL_VALUE(Value) Value
#endif
#ifndef YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
# define YY_IGNORE_MAYBE_UNINITIALIZED_END
#endif
#ifndef YY_INITIAL_VALUE
# define YY_INITIAL_VALUE(Value) /* Nothing. */
#endif

#if defined __cplusplus && defined __GNUC__ && ! defined __ICC && 6 <= __GNUC__
# define YY_IGNORE_USELESS_CAST_BEGIN                          \
    _Pragma ("GCC diagnostic push")                            \
    _Pragma ("GCC diagnostic ignored \"-Wuseless-cast\"")
# define YY_IGNORE_USELESS_CAST_END            \
    _Pragma ("GCC diagnostic pop")
#endif
#ifndef YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_BEGIN
# define YY_IGNORE_USELESS_CAST_END
#endif


#define YY_ASSERT(E) ((void) (0 && (E)))

#if ! defined yyoverflow || YYERROR_VERBOSE

/* The parser invokes alloca or malloc; define the necessary symbols.  */

# ifdef YYSTACK_USE_ALLOCA
#  if YYSTACK_USE_ALLOCA
#   ifdef __GNUC__
#    define YYSTACK_ALLOC __builtin_alloca
#   elif defined __BUILTIN_VA_ARG_INCR
#    include <alloca.h> /* INFRINGES ON USER NAME SPACE */
#   elif defined _AIX
#    define YYSTACK_ALLOC __alloca
#   elif defined _MSC_VER
#    include <malloc.h> /* INFRINGES ON USER NAME SPACE */
#    define alloca _alloca
#   else
#    define YYSTACK_ALLOC alloca
#    if ! defined _ALLOCA_H && ! defined EXIT_SUCCESS
#     include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
      /* Use EXIT_SUCCESS as a witness for stdlib.h.  */
#     ifndef EXIT_SUCCESS
#      define EXIT_SUCCESS 0
#     endif
#    endif
#   endif
#  endif
# endif

# ifdef YYSTACK_ALLOC
   /* Pacify GCC's 'empty if-body' warning.  */
#  define YYSTACK_FREE(Ptr) do { /* empty */; } while (0)
#  ifndef YYSTACK_ALLOC_MAXIMUM
    /* The OS might guarantee only one guard page at the bottom of the stack,
       and a page size can be as small as 4096 bytes.  So we cannot safely
       invoke alloca (N) if N exceeds 4096.  Use a slightly smaller number
       to allow for a few compiler-allocated temporary stack slots.  */
#   define YYSTACK_ALLOC_MAXIMUM 4032 /* reasonable circa 2006 */
#  endif
# else
#  define YYSTACK_ALLOC YYMALLOC
#  define YYSTACK_FREE YYFREE
#  ifndef YYSTACK_ALLOC_MAXIMUM
#   define YYSTACK_ALLOC_MAXIMUM YYSIZE_MAXIMUM
#  endif
#  if (defined __cplusplus && ! defined EXIT_SUCCESS \
       && ! ((defined YYMALLOC || defined malloc) \
             && (defined YYFREE || defined free)))
#   include <stdlib.h> /* INFRINGES ON USER NAME SPACE */
#   ifndef EXIT_SUCCESS
#    define EXIT_SUCCESS 0
#   endif
#  endif
#  ifndef YYMALLOC
#   define YYMALLOC malloc
#   if ! defined malloc && ! defined EXIT_SUCCESS
void *malloc (YYSIZE_T); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
#  ifndef YYFREE
#   define YYFREE free
#   if ! defined free && ! defined EXIT_SUCCESS
void free (void *); /* INFRINGES ON USER NAME SPACE */
#   endif
#  endif
# endif
#endif /* ! defined yyoverflow || YYERROR_VERBOSE */


#if (! defined yyoverflow \
     && (! defined __cplusplus \
         || (defined YYSTYPE_IS_TRIVIAL && YYSTYPE_IS_TRIVIAL)))

/* A type that is properly aligned for any stack member.  */
union yyalloc
{
  yy_state_t yyss_alloc;
  YYSTYPE yyvs_alloc;
};

/* The size of the maximum gap between one aligned stack and the next.  */
# define YYSTACK_GAP_MAXIMUM (YYSIZEOF (union yyalloc) - 1)

/* The size of an array large to enough to hold all stacks, each with
   N elements.  */
# define YYSTACK_BYTES(N) \
     ((N) * (YYSIZEOF (yy_state_t) + YYSIZEOF (YYSTYPE)) \
      + YYSTACK_GAP_MAXIMUM)

# define YYCOPY_NEEDED 1

/* Relocate STACK from its old location to the new one.  The
   local variables YYSIZE and YYSTACKSIZE give the old and new number of
   elements in the stack, and YYPTR gives the new location of the
   stack.  Advance YYPTR to a properly aligned location for the next
   stack.  */
# define YYSTACK_RELOCATE(Stack_alloc, Stack)                           \
    do                                                                  \
      {                                                                 \
        YYPTRDIFF_T yynewbytes;                                         \
        YYCOPY (&yyptr->Stack_alloc, Stack, yysize);                    \
        Stack = &yyptr->Stack_alloc;                                    \
        yynewbytes = yystacksize * YYSIZEOF (*Stack) + YYSTACK_GAP_MAXIMUM; \
        yyptr += yynewbytes / YYSIZEOF (*yyptr);                        \
      }                                                                 \
    while (0)

#endif

#if defined YYCOPY_NEEDED && YYCOPY_NEEDED
/* Copy COUNT objects from SRC to DST.  The source and destination do
   not overlap.  */
# ifndef YYCOPY
#  if defined __GNUC__ && 1 < __GNUC__
#   define YYCOPY(Dst, Src, Count) \
      __builtin_memcpy (Dst, Src, YY_CAST (YYSIZE_T, (Count)) * sizeof (*(Src)))
#  else
#   define YYCOPY(Dst, Src, Count)              \
      do                                        \
        {                                       \
          YYPTRDIFF_T yyi;                      \
          for (yyi = 0; yyi < (Count); yyi++)   \
            (Dst)[yyi] = (Src)[yyi];            \
        }                                       \
      while (0)
#  endif
# endif
#endif /* !YYCOPY_NEEDED */

/* YYFINAL -- State number of the termination state.  */
#define YYFINAL  6
/* YYLAST -- Last index in YYTABLE.  */
#define YYLAST   1493

/* YYNTOKENS -- Number of terminals.  */
#define YYNTOKENS  82
/* YYNNTS -- Number of nonterminals.  */
#define YYNNTS  59
/* YYNRULES -- Number of rules.  */
#define YYNRULES  198
/* YYNSTATES -- Number of states.  */
#define YYNSTATES  403

#define YYUNDEFTOK  2
#define YYMAXUTOK   321


/* YYTRANSLATE(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex, with out-of-bounds checking.  */
#define YYTRANSLATE(YYX)                                                \
  (0 <= (YYX) && (YYX) <= YYMAXUTOK ? yytranslate[YYX] : YYUNDEFTOK)

/* YYTRANSLATE[TOKEN-NUM] -- Symbol number corresponding to TOKEN-NUM
   as returned by yylex.  */
static const yytype_int8 yytranslate[] =
{
       0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,    29,     2,
      76,    77,    30,    28,    75,    27,    81,    31,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,    72,
       2,    80,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,    73,     2,    74,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,    78,     2,    79,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    32,    33,    34,    35,    36,    37,    38,    39,
      40,    41,    42,    43,    44,    45,    46,    47,    48,    49,
      50,    51,    52,    53,    54,    55,    56,    57,    58,    59,
      60,    61,    62,    63,    64,    65,    66,    67,    68,    69,
      70,    71
};

#if YYDEBUG
  /* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
static const yytype_int16 yyrline[] =
{
       0,  1798,  1798,  1797,  2052,  2053,  2054,  2057,  2073,  2074,
    2078,  2084,  2094,  2112,  2121,  2131,  2146,  2151,  2171,  2178,
    2192,  2205,  2225,  2226,  2227,  2230,  2239,  2245,  2251,  2257,
    2269,  2278,  2297,  2298,  2301,  2318,  2329,  2330,  2331,  2332,
    2333,  2334,  2338,  2361,  2382,  2386,  2390,  2394,  2398,  2402,
    2406,  2413,  2424,  2439,  2471,  2503,  2504,  2512,  2526,  2544,
    2548,  2555,  2585,  2584,  2628,  2656,  2705,  2756,  2757,  2760,
    2791,  2842,  2849,  2853,  2877,  2893,  2914,  2973,  3025,  3029,
    3033,  3057,  3102,  3135,  3142,  3149,  3156,  3183,  3191,  3199,
    3207,  3220,  3233,  3241,  3249,  3262,  3275,  3283,  3291,  3304,
    3325,  3336,  3347,  3358,  3369,  3391,  3399,  3405,  3435,  3444,
    3453,  3462,  3471,  3482,  3489,  3497,  3505,  3513,  3521,  3536,
    3543,  3579,  3583,  3617,  3637,  3636,  3701,  3700,  3750,  3758,
    3766,  3774,  3783,  3800,  3805,  3815,  3877,  3885,  3893,  3901,
    3909,  3929,  3941,  3949,  3958,  3966,  4020,  4033,  4041,  4066,
    4086,  4094,  4102,  4110,  4118,  4135,  4140,  4149,  4210,  4218,
    4226,  4234,  4242,  4254,  4262,  4275,  4288,  4296,  4309,  4322,
    4330,  4338,  4346,  4361,  4369,  4377,  4384,  4392,  4401,  4415,
    4429,  4450,  4458,  4466,  4475,  4539,  4547,  4561,  4572,  4585,
    4598,  4610,  4629,  4642,  4647,  4663,  4673,  4683,  4693
};
#endif

#if YYDEBUG || YYERROR_VERBOSE || 0
/* YYTNAME[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
   First, the terminals, then, starting at YYNTOKENS, nonterminals.  */
static const char *const yytname[] =
{
  "$end", "error", "$undefined", "FUNCTION", "TRIGGER", "EVENT", "WAIT",
  "EVERY", "INACTIVE", "INITIALISE", "LINK", "REF", "RET", "WHILE", "IF",
  "ELSE", "EXIT", "PAUSE", "BOOLEQUAL", "NOTEQUAL", "GREATEQUAL",
  "LESSEQUAL", "GREATER", "LESS", "_AND", "_OR", "_NOT", "'-'", "'+'",
  "'&'", "'*'", "'/'", "UMINUS", "BOOLEAN_T", "INTEGER", "QTEXT", "TYPE",
  "STORAGE", "IDENT", "VAR", "BOOL_VAR", "NUM_VAR", "OBJ_VAR",
  "STRING_VAR", "VAR_ARRAY", "BOOL_ARRAY", "NUM_ARRAY", "OBJ_ARRAY",
  "BOOL_OBJVAR", "NUM_OBJVAR", "USER_OBJVAR", "OBJ_OBJVAR",
  "BOOL_CONSTANT", "NUM_CONSTANT", "USER_CONSTANT", "OBJ_CONSTANT",
  "STRING_CONSTANT", "FUNC", "BOOL_FUNC", "NUM_FUNC", "USER_FUNC",
  "OBJ_FUNC", "STRING_FUNC", "VOID_FUNC_CUST", "BOOL_FUNC_CUST",
  "NUM_FUNC_CUST", "USER_FUNC_CUST", "OBJ_FUNC_CUST", "STRING_FUNC_CUST",
  "TRIG_SYM", "EVENT_SYM", "CALLBACK_SYM", "';'", "'['", "']'", "','",
  "'('", "')'", "'{'", "'}'", "'='", "'.'", "$accept", "script", "$@1",
  "header", "header_decl", "var_list", "var_line", "variable_decl_head",
  "array_sub_decl", "array_sub_decl_list", "variable_ident",
  "variable_decl", "trigger_list", "trigger_subdecl", "trigger_decl",
  "event_list", "event_subdecl", "function_type", "func_subdecl",
  "funcvar_decl_types", "funcbody_var_def", "argument_decl_head",
  "argument_decl", "function_declaration", "event_decl", "$@2",
  "return_statement_void", "return_statement", "statement_list",
  "statement", "return_exp", "assignment", "func_call", "param_list",
  "parameter", "var_ref", "conditional", "cond_clause_list",
  "terminal_cond", "cond_clause", "$@3", "loop", "$@4", "expression",
  "stringexp", "boolexp", "userexp", "objexp", "objexp_dot", "num_objvar",
  "bool_objvar", "user_objvar", "obj_objvar", "array_index",
  "array_index_list", "num_array_var", "bool_array_var", "obj_array_var",
  "user_array_var", YY_NULLPTR
};
#endif

# ifdef YYPRINT
/* YYTOKNUM[NUM] -- (External) token number corresponding to the
   (internal) symbol number NUM (which must be that of a token).  */
static const yytype_int16 yytoknum[] =
{
       0,   256,   257,   258,   259,   260,   261,   262,   263,   264,
     265,   266,   267,   268,   269,   270,   271,   272,   273,   274,
     275,   276,   277,   278,   279,   280,   281,    45,    43,    38,
      42,    47,   282,   283,   284,   285,   286,   287,   288,   289,
     290,   291,   292,   293,   294,   295,   296,   297,   298,   299,
     300,   301,   302,   303,   304,   305,   306,   307,   308,   309,
     310,   311,   312,   313,   314,   315,   316,   317,   318,   319,
     320,   321,    59,    91,    93,    44,    40,    41,   123,   125,
      61,    46
};
# endif

#define YYPACT_NINF (-317)

#define yypact_value_is_default(Yyn) \
  ((Yyn) == YYPACT_NINF)

#define YYTABLE_NINF (-184)

#define yytable_value_is_error(Yyn) \
  0

  /* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
     STATE-NUM.  */
static const yytype_int16 yypact[] =
{
      40,    36,    76,    23,  -317,    11,  -317,    18,  -317,    51,
    -317,    84,   -48,  -317,  -317,  -317,  -317,   138,  -317,    78,
    -317,  -317,    84,   124,    75,  -317,   134,  -317,    92,  -317,
      95,   149,    -6,  -317,    56,   -38,   120,   126,  -317,   147,
     206,   777,   339,  -317,  -317,  -317,  -317,   843,   -24,   177,
      51,  -317,   162,   169,   184,  -317,  -317,   939,  1417,  -317,
    -317,  -317,  -317,  -317,  -317,   179,   179,   179,   179,  -317,
    -317,  -317,  -317,   205,   216,   221,   223,   226,   229,   230,
    -317,  -317,   207,   939,   203,   745,    12,    47,    -4,    65,
    -317,  -317,  -317,  -317,  -317,  -317,  -317,  -317,  -317,  -317,
    -317,  -317,  -317,  -317,  -317,  -317,   231,   236,   238,   387,
     241,  -317,   -28,     9,  -317,  1346,  -317,   282,   292,  -317,
    1417,  -317,   246,    46,  1417,  -317,   260,   260,   260,   260,
      85,    85,    85,    85,    85,    85,    85,    85,    -2,    33,
     263,  1417,  1417,  1417,  1417,  1417,  1417,  1417,  1417,  1417,
    1417,   939,   939,   939,   939,   302,  1287,  1287,   172,   172,
    -317,  -317,  -317,  -317,  -317,   259,   261,  -317,  -317,  -317,
    -317,  -317,  -317,  -317,  -317,  -317,   305,   264,   311,   270,
     639,   262,   273,   278,   275,   276,   279,   280,   283,   284,
     289,   291,   293,   303,   308,   309,   310,  -317,  -317,  1382,
    -317,   286,   296,  -317,   380,  -317,  -317,   316,   317,   332,
     333,   334,   337,   342,   344,  -317,  -317,   139,    43,  1417,
     168,  -317,  -317,  -317,  -317,   355,   891,    17,  -317,  -317,
     745,   404,   204,    47,    -4,    58,    59,    81,    82,    97,
     102,   360,  -317,  -317,  -317,   301,   301,   301,   301,   301,
     301,    69,    69,  -317,  -317,  -317,  -317,   129,   129,  -317,
    -317,   181,  -317,   246,   388,   246,    51,    51,   362,   434,
    -317,    51,    84,  -317,   373,   745,   404,   204,    -4,   939,
     939,  -317,   416,  1287,   939,  1417,   172,   200,    85,    85,
      85,    85,    85,    85,    85,   372,  -317,  -317,  -317,   -11,
    1417,   939,  1287,   172,  1417,   939,   172,  1287,  -317,   164,
    -317,  -317,  -317,  -317,  -317,    85,   -21,    85,  -317,   891,
    -317,  -317,  -317,  -317,  -317,  -317,  1346,  1346,    51,  1346,
    -317,  -317,    44,    83,   375,  -317,   204,   301,   246,  -317,
    -317,   200,   404,   106,   109,   113,   170,   189,   190,   195,
    -317,  1382,  -317,  -317,   301,   204,  -317,   246,   301,   204,
     246,  -317,  -317,   202,  -317,  -317,   745,  -317,   204,  1000,
    1052,  1346,  1382,  -317,  -317,   381,  -317,  -317,  -317,  -317,
     382,   384,   389,  1104,  -317,  -317,  -317,  1156,   383,   376,
     386,  -317,  -317,  -317,  -317,  -317,  -317,  1382,  1382,  1208,
    1260,  -317,  -317
};

  /* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
     Performed when YYTABLE does not specify something else to do.  Zero
     means the default is an error.  */
static const yytype_uint8 yydefact[] =
{
       4,     0,     0,     8,     5,     0,     1,     0,     6,     2,
       9,     0,     0,     7,    13,    14,    12,    22,    10,    18,
      20,    11,     0,     0,     0,    23,     0,    16,    19,    21,
       0,     0,     0,    24,     3,     0,     0,     0,    32,     0,
       0,     0,     0,    34,    35,    33,    59,     0,     0,     0,
       8,    15,     0,     0,     0,   179,    28,     0,     0,   162,
     140,   173,   158,   136,   181,     0,     0,     0,     0,   159,
     137,   174,   182,     0,     0,     0,     0,     0,     0,     0,
     178,   180,    29,     0,     0,     0,     0,     0,     0,     0,
     138,   160,   175,   185,   139,   161,   186,   176,    42,    36,
      37,    38,    39,    40,    41,    43,   179,   178,     0,     0,
      55,    51,     0,     0,    60,    67,    17,     0,     0,   154,
       0,   132,     0,     0,     0,   193,   198,   196,   195,   197,
     105,   105,   105,   105,   105,   105,   105,   105,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
     187,   189,   188,   190,   191,     0,     0,    62,    50,    45,
      44,    47,    46,    49,    48,    53,     0,     0,     0,    56,
       0,     0,     0,     0,     0,     0,     0,     0,   181,     0,
       0,     0,     0,     0,     0,     0,     0,    69,    81,    67,
      72,     0,     0,    78,   119,   121,    79,     0,     0,     0,
     185,     0,     0,   186,     0,    26,    27,     0,     0,     0,
       0,   140,   148,   136,   146,     0,     0,     0,   106,   113,
     108,   110,   109,   111,   112,     0,     0,     0,     0,     0,
       0,    30,   133,   155,    31,   163,   166,   170,   169,   171,
     172,   129,   128,   130,   131,   152,   153,   150,   151,    25,
     164,     0,   167,   165,     0,   168,     8,     8,     0,     0,
      52,     8,     0,    68,     0,    83,    84,    85,    86,     0,
       0,    80,     0,     0,     0,     0,     0,     0,   105,   105,
     105,   105,   105,   105,   105,    81,    73,    74,    75,     0,
       0,     0,     0,     0,     0,     0,     0,     0,   192,     0,
     117,   115,   114,   118,   116,   105,     0,     0,   156,     0,
     134,   177,   183,   157,   135,   184,    67,    67,     8,    67,
      54,    70,     0,     0,     0,    91,    88,    87,    90,   149,
     147,     0,    89,     0,     0,     0,     0,     0,     0,     0,
      65,    67,   120,   122,    92,    93,    94,    95,    96,    97,
      99,    98,   194,     0,   144,   107,   142,   141,   143,     0,
       0,    67,    67,   126,   124,     0,   104,   101,   100,   102,
     103,     0,     0,     0,   145,    64,    61,     0,    81,     0,
       0,    82,    76,    77,   123,    63,    66,    67,    67,     0,
       0,   127,   125
};

  /* YYPGOTO[NTERM-NUM].  */
static const yytype_int16 yypgoto[] =
{
    -317,  -317,  -317,  -317,   462,   -41,    -9,  -317,  -317,  -317,
     -18,  -317,  -317,   419,   443,  -317,  -317,  -317,  -317,   304,
    -317,  -317,  -317,  -317,   445,  -317,  -317,  -192,  -316,  -194,
    -317,  -317,  -317,   -91,   151,  -317,  -317,  -317,  -317,   187,
    -317,  -317,  -317,   437,  -178,   469,   450,   257,   495,  -114,
    -109,   -76,   -26,  -317,    37,   -17,    49,    72,   131
};

  /* YYDEFGOTO[NTERM-NUM].  */
static const yytype_int16 yydefgoto[] =
{
      -1,     2,    17,     3,     4,     9,    10,    11,    27,    28,
      20,    12,    24,    84,    25,    34,    35,   105,    36,   111,
     112,   113,    49,    37,    38,   268,   197,   198,   199,   200,
     274,   201,   202,   227,   228,   229,   203,   204,   352,   205,
     390,   206,   389,   230,   231,   232,   233,   122,    89,    90,
      91,    92,    93,   125,   126,    94,    95,    96,    97
};

  /* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
     positive, shift that token.  If negative, reduce the rule whose
     number is the opposite.  If YYTABLE_NINF, syntax error.  */
static const yytype_int16 yytable[] =
{
      18,   207,   276,   182,    29,   296,   208,   295,   319,   115,
     369,   370,   109,   372,   158,   159,   141,   142,   143,   144,
     145,   146,    14,    15,    21,   147,   148,    22,   149,   150,
     151,   152,    43,     1,    46,   383,   153,   154,    47,   209,
     235,   236,   237,   238,   239,   240,   241,   176,   316,   177,
       1,   151,   152,   110,    16,   387,   364,   153,   154,    31,
       7,    32,   151,   152,    44,   156,   157,   351,   153,   154,
     147,   148,     5,   149,   150,   242,     6,   160,    31,    23,
      32,   399,   400,    13,   178,   207,   179,   155,     7,   210,
     208,   175,   317,    55,   318,   162,   220,   164,   211,   149,
     150,   151,   152,   127,   128,   129,    18,   153,   154,   342,
     243,    57,    58,   161,   162,   163,   164,   308,    59,   221,
     222,   373,    19,   209,    61,    62,   223,    64,   224,    65,
      66,    67,    68,   317,   317,   320,   321,    69,    70,    71,
      72,   367,    23,    73,    74,    75,    76,   151,   152,    77,
      78,    26,    79,   225,    80,    81,   317,   317,   322,   323,
     374,   226,    30,   316,   212,    40,   147,   148,    39,   149,
     150,    41,   317,   210,   324,   296,   296,   317,   296,   325,
     388,   317,   211,   376,   317,    42,   377,   213,   317,   296,
     378,   147,   148,   296,   149,   150,    48,   343,   344,   345,
     346,   347,   348,   349,    50,   296,   296,   310,   311,   312,
     313,   314,   207,   207,    64,   207,   242,   208,   208,    68,
     208,    51,   151,   152,   363,   326,   327,    72,   153,   154,
     329,   163,   164,    76,   339,   222,   116,   207,   362,    79,
      52,   340,   208,   224,   117,   317,   214,   379,   212,   114,
     209,   209,   124,   209,   330,   207,   207,   207,   207,   118,
     208,   208,   208,   208,   317,   317,   380,   381,   225,   207,
     317,   213,   382,   207,   208,   209,   341,   317,   208,   384,
     140,   130,   137,   207,   207,   207,   207,   371,   208,   208,
     208,   208,   131,   209,   209,   209,   209,   132,    88,   133,
     210,   210,   134,   210,    88,   135,   136,   209,   165,   211,
     211,   209,   211,   166,    88,   167,   215,    18,    18,   -58,
      18,   209,   209,   209,   209,   210,   216,   160,   147,   148,
     214,   149,   150,   219,   211,   244,   259,   266,   279,   267,
      88,   269,   271,   210,   210,   210,   210,   272,   -57,   280,
     281,   282,   211,   211,   211,   211,   283,   210,   297,   284,
     285,   210,    18,   286,   287,   288,   211,   289,   298,   290,
     211,   210,   210,   210,   210,   212,   212,    98,   212,   291,
     211,   211,   211,   211,   292,   293,   294,   234,   234,   234,
     234,   234,   234,   234,   234,   299,   300,   301,   213,   213,
     212,   213,    99,   100,   101,   102,   103,   104,    88,    88,
      88,    88,   302,   303,   304,   263,   265,   305,   212,   212,
     212,   212,   306,   213,   307,    19,   168,   169,   170,   171,
     172,   315,   212,   319,   173,   317,   212,   278,   174,   164,
     328,   213,   213,   213,   213,   331,   212,   212,   212,   212,
     334,   350,   375,   391,   397,   213,   392,   214,   214,   213,
     214,   393,   396,  -183,   398,     8,   108,    33,   365,   213,
     213,   213,   213,   168,   169,   170,   171,   172,    85,    45,
     270,   173,   214,    88,    85,   174,   353,     0,     0,     0,
       0,    87,     0,     0,    85,   121,     0,    87,     0,     0,
     214,   214,   214,   214,     0,     0,     0,    87,     0,     0,
      86,     0,     0,     0,   214,     0,    86,     0,   214,     0,
     138,     0,     0,     0,     0,     0,   119,     0,   214,   214,
     214,   214,     0,    87,     0,     0,    88,    88,     0,     0,
       0,    88,     0,   338,     0,   234,   234,   234,   234,   234,
     234,   234,   139,   123,     0,     0,     0,   217,    88,     0,
     357,   218,    88,   360,     0,     0,     0,     0,     0,     0,
       0,     0,   234,     0,   234,     0,    88,     0,   245,   246,
     247,   248,   249,   250,   251,   252,   253,   254,    85,    85,
      85,    85,     0,     0,     0,     0,     0,     0,     0,     0,
       0,    87,    87,    87,    87,     0,   260,   262,     0,     0,
       0,     0,     0,     0,     0,   123,     0,   275,     0,   123,
     255,   256,   257,   258,     0,     0,     0,     0,     0,     0,
      87,     0,     0,     0,     0,     0,   123,   123,   123,   123,
     123,   123,   123,   123,   123,   123,     0,    55,     0,   277,
       0,   261,   261,   264,   264,     0,   309,     0,     0,     0,
       0,     0,     0,   138,     0,    57,    58,     0,     0,     0,
       0,     0,    59,   221,   222,     0,    87,     0,    61,    62,
     223,    64,   224,    65,    66,    67,    68,     0,     0,     0,
       0,    69,    70,    71,    72,   139,     0,    73,    74,    75,
      76,     0,     0,    77,    78,     0,    79,   225,    80,    81,
       0,   273,     0,     0,   123,   226,    85,    85,     0,     0,
       0,    85,   337,     0,     0,     0,     0,     0,     0,    87,
      87,     0,     0,   335,    87,     0,     0,   354,    85,     0,
       0,   358,    85,     0,     0,     0,     0,     0,   332,   333,
       0,    87,   356,   336,     0,    87,   366,   361,     0,     0,
       0,     0,     0,   141,   142,   143,   144,   145,   146,    87,
     355,     0,   147,   148,   359,   149,   150,     0,   261,     0,
     123,   264,     0,    53,    54,    55,    56,     0,   368,     0,
       0,     0,     0,     0,     0,   123,     0,   261,   264,   123,
       0,   264,   261,    57,    58,     0,     0,     0,     0,     0,
      59,    60,     0,     0,     0,     0,    61,    62,    63,    64,
       0,    65,    66,    67,    68,     0,     0,     0,     0,    69,
      70,    71,    72,     0,     0,    73,    74,    75,    76,     0,
       0,    77,    78,     0,    79,     0,    80,    81,    82,    53,
      54,   106,    56,    83,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,    57,
      58,     0,     0,     0,     0,     0,    59,    60,     0,     0,
       0,     0,    61,    62,    63,    64,     0,    65,    66,    67,
      68,     0,     0,     0,     0,    69,    70,    71,    72,    55,
       0,    73,    74,    75,    76,     0,     0,    77,    78,     0,
      79,     0,   107,    81,    82,     0,     0,    57,    58,    83,
       0,     0,     0,     0,    59,   221,   222,     0,     0,     0,
      61,    62,   223,    64,   224,    65,    66,    67,    68,     0,
       0,     0,     0,    69,    70,    71,    72,    55,     0,    73,
      74,    75,    76,     0,     0,    77,    78,     0,    79,   225,
      80,    81,     0,     0,     0,    57,    58,   226,     0,     0,
       0,     0,    59,    60,     0,     0,     0,     0,    61,    62,
      63,    64,     0,    65,    66,    67,    68,     0,     0,     0,
       0,    69,    70,    71,    72,     0,     0,    73,    74,    75,
      76,     0,     0,    77,    78,     0,    79,     0,    80,    81,
       0,     0,   180,   181,   182,    83,   183,   184,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,   185,
     186,   187,   188,   189,    65,    66,    67,    68,     0,     0,
       0,     0,     0,     0,     0,    72,     0,   190,   191,   192,
     193,   194,     0,   195,   180,   181,   182,    79,   183,   184,
     196,     0,     0,     0,     0,     0,     0,     0,     0,   385,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,   185,   186,   187,   188,   189,    65,    66,    67,    68,
       0,     0,     0,     0,     0,     0,     0,    72,     0,   190,
     191,   192,   193,   194,     0,   195,   180,   181,   182,    79,
     183,   184,   196,     0,     0,     0,     0,     0,     0,     0,
       0,   386,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,   185,   186,   187,   188,   189,    65,    66,
      67,    68,     0,     0,     0,     0,     0,     0,     0,    72,
       0,   190,   191,   192,   193,   194,     0,   195,   180,   181,
     182,    79,   183,   184,   196,     0,     0,     0,     0,     0,
       0,     0,     0,   394,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,   185,   186,   187,   188,   189,
      65,    66,    67,    68,     0,     0,     0,     0,     0,     0,
       0,    72,     0,   190,   191,   192,   193,   194,     0,   195,
     180,   181,   182,    79,   183,   184,   196,     0,     0,     0,
       0,     0,     0,     0,     0,   395,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,   185,   186,   187,
     188,   189,    65,    66,    67,    68,     0,     0,     0,     0,
       0,     0,     0,    72,     0,   190,   191,   192,   193,   194,
       0,   195,   180,   181,   182,    79,   183,   184,   196,     0,
       0,     0,     0,     0,     0,     0,     0,   401,     0,     0,
       0,     0,     0,     0,     0,    55,     0,     0,     0,   185,
     186,   187,   188,   189,    65,    66,    67,    68,     0,     0,
       0,     0,     0,     0,     0,    72,     0,   190,   191,   192,
     193,   194,     0,   195,     0,     0,    61,    79,     0,    64,
     196,    65,     0,     0,    68,     0,     0,     0,     0,   402,
       0,    71,    72,     0,     0,     0,     0,    75,    76,     0,
       0,     0,     0,     0,    79,     0,    80,    81,   180,   181,
     182,     0,   183,   184,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     0,     0,     0,     0,     0,     0,     0,
       0,     0,     0,     7,     0,   185,   186,   187,   188,   189,
      65,    66,    67,    68,   180,   181,   182,     0,   183,   184,
       0,    72,     0,   190,   191,   192,   193,   194,     0,   195,
       0,     0,     0,    79,     0,     0,   196,     0,     0,     0,
       0,   185,   186,   187,   188,   189,    65,    66,    67,    68,
       0,     0,     0,     0,     0,     0,     0,    72,     0,   190,
     191,   192,   193,   194,    58,   195,     0,     0,     0,    79,
       0,    60,   196,     0,     0,     0,     0,     0,    63,    64,
       0,     0,     0,    67,    68,     0,     0,     0,     0,     0,
      70,     0,    72,     0,     0,     0,    74,     0,    76,     0,
       0,     0,    78,     0,    79,     0,     0,     0,     0,     0,
       0,     0,     0,   120
};

static const yytype_int16 yycheck[] =
{
       9,   115,   180,    14,    22,   199,   115,   199,    29,    50,
     326,   327,    36,   329,    18,    19,    18,    19,    20,    21,
      22,    23,     4,     5,    72,    27,    28,    75,    30,    31,
      18,    19,    38,    10,    72,   351,    24,    25,    76,   115,
     131,   132,   133,   134,   135,   136,   137,    75,   226,    77,
      10,    18,    19,    77,    36,   371,    77,    24,    25,     3,
      37,     5,    18,    19,    70,    18,    19,    78,    24,    25,
      27,    28,    36,    30,    31,    77,     0,    81,     3,     4,
       5,   397,   398,    72,    75,   199,    77,    75,    37,   115,
     199,   109,    75,     8,    77,    49,    11,    51,   115,    30,
      31,    18,    19,    66,    67,    68,   115,    24,    25,   287,
      77,    26,    27,    48,    49,    50,    51,    74,    33,    34,
      35,    77,    38,   199,    39,    40,    41,    42,    43,    44,
      45,    46,    47,    75,    75,    77,    77,    52,    53,    54,
      55,   319,     4,    58,    59,    60,    61,    18,    19,    64,
      65,    73,    67,    68,    69,    70,    75,    75,    77,    77,
      77,    76,    38,   341,   115,    73,    27,    28,    34,    30,
      31,    76,    75,   199,    77,   369,   370,    75,   372,    77,
     372,    75,   199,    77,    75,    36,    77,   115,    75,   383,
      77,    27,    28,   387,    30,    31,    76,   288,   289,   290,
     291,   292,   293,   294,    78,   399,   400,    39,    40,    41,
      42,    43,   326,   327,    42,   329,    77,   326,   327,    47,
     329,    74,    18,    19,   315,   266,   267,    55,    24,    25,
     271,    50,    51,    61,    34,    35,    74,   351,    74,    67,
      34,    41,   351,    43,    75,    75,   115,    77,   199,    72,
     326,   327,    73,   329,   272,   369,   370,   371,   372,    75,
     369,   370,   371,   372,    75,    75,    77,    77,    68,   383,
      75,   199,    77,   387,   383,   351,    76,    75,   387,    77,
      77,    76,    75,   397,   398,   399,   400,   328,   397,   398,
     399,   400,    76,   369,   370,   371,   372,    76,    41,    76,
     326,   327,    76,   329,    47,    76,    76,   383,    77,   326,
     327,   387,   329,    77,    57,    77,    34,   326,   327,    78,
     329,   397,   398,   399,   400,   351,    34,    81,    27,    28,
     199,    30,    31,    73,   351,    72,    34,    78,    76,    78,
      83,    36,    78,   369,   370,   371,   372,    36,    78,    76,
      72,    76,   369,   370,   371,   372,    80,   383,    72,    80,
      80,   387,   371,    80,    80,    76,   383,    76,    72,    76,
     387,   397,   398,   399,   400,   326,   327,    38,   329,    76,
     397,   398,   399,   400,    76,    76,    76,   130,   131,   132,
     133,   134,   135,   136,   137,    15,    80,    80,   326,   327,
     351,   329,    63,    64,    65,    66,    67,    68,   151,   152,
     153,   154,    80,    80,    80,   158,   159,    80,   369,   370,
     371,   372,    80,   351,    80,    38,    39,    40,    41,    42,
      43,    76,   383,    29,    47,    75,   387,   180,    51,    51,
      78,   369,   370,   371,   372,    72,   397,   398,   399,   400,
      34,    79,    77,    72,    78,   383,    72,   326,   327,   387,
     329,    72,    79,    81,    78,     3,    47,    24,   317,   397,
     398,   399,   400,    39,    40,    41,    42,    43,    41,    34,
     176,    47,   351,   226,    47,    51,   299,    -1,    -1,    -1,
      -1,    41,    -1,    -1,    57,    58,    -1,    47,    -1,    -1,
     369,   370,   371,   372,    -1,    -1,    -1,    57,    -1,    -1,
      41,    -1,    -1,    -1,   383,    -1,    47,    -1,   387,    -1,
      83,    -1,    -1,    -1,    -1,    -1,    57,    -1,   397,   398,
     399,   400,    -1,    83,    -1,    -1,   279,   280,    -1,    -1,
      -1,   284,    -1,   286,    -1,   288,   289,   290,   291,   292,
     293,   294,    83,    58,    -1,    -1,    -1,   120,   301,    -1,
     303,   124,   305,   306,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,   315,    -1,   317,    -1,   319,    -1,   141,   142,
     143,   144,   145,   146,   147,   148,   149,   150,   151,   152,
     153,   154,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,   151,   152,   153,   154,    -1,   156,   157,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,   120,    -1,   180,    -1,   124,
     151,   152,   153,   154,    -1,    -1,    -1,    -1,    -1,    -1,
     180,    -1,    -1,    -1,    -1,    -1,   141,   142,   143,   144,
     145,   146,   147,   148,   149,   150,    -1,     8,    -1,   180,
      -1,   156,   157,   158,   159,    -1,   219,    -1,    -1,    -1,
      -1,    -1,    -1,   226,    -1,    26,    27,    -1,    -1,    -1,
      -1,    -1,    33,    34,    35,    -1,   226,    -1,    39,    40,
      41,    42,    43,    44,    45,    46,    47,    -1,    -1,    -1,
      -1,    52,    53,    54,    55,   226,    -1,    58,    59,    60,
      61,    -1,    -1,    64,    65,    -1,    67,    68,    69,    70,
      -1,    72,    -1,    -1,   219,    76,   279,   280,    -1,    -1,
      -1,   284,   285,    -1,    -1,    -1,    -1,    -1,    -1,   279,
     280,    -1,    -1,   283,   284,    -1,    -1,   300,   301,    -1,
      -1,   304,   305,    -1,    -1,    -1,    -1,    -1,   279,   280,
      -1,   301,   302,   284,    -1,   305,   319,   307,    -1,    -1,
      -1,    -1,    -1,    18,    19,    20,    21,    22,    23,   319,
     301,    -1,    27,    28,   305,    30,    31,    -1,   283,    -1,
     285,   286,    -1,     6,     7,     8,     9,    -1,   319,    -1,
      -1,    -1,    -1,    -1,    -1,   300,    -1,   302,   303,   304,
      -1,   306,   307,    26,    27,    -1,    -1,    -1,    -1,    -1,
      33,    34,    -1,    -1,    -1,    -1,    39,    40,    41,    42,
      -1,    44,    45,    46,    47,    -1,    -1,    -1,    -1,    52,
      53,    54,    55,    -1,    -1,    58,    59,    60,    61,    -1,
      -1,    64,    65,    -1,    67,    -1,    69,    70,    71,     6,
       7,     8,     9,    76,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    26,
      27,    -1,    -1,    -1,    -1,    -1,    33,    34,    -1,    -1,
      -1,    -1,    39,    40,    41,    42,    -1,    44,    45,    46,
      47,    -1,    -1,    -1,    -1,    52,    53,    54,    55,     8,
      -1,    58,    59,    60,    61,    -1,    -1,    64,    65,    -1,
      67,    -1,    69,    70,    71,    -1,    -1,    26,    27,    76,
      -1,    -1,    -1,    -1,    33,    34,    35,    -1,    -1,    -1,
      39,    40,    41,    42,    43,    44,    45,    46,    47,    -1,
      -1,    -1,    -1,    52,    53,    54,    55,     8,    -1,    58,
      59,    60,    61,    -1,    -1,    64,    65,    -1,    67,    68,
      69,    70,    -1,    -1,    -1,    26,    27,    76,    -1,    -1,
      -1,    -1,    33,    34,    -1,    -1,    -1,    -1,    39,    40,
      41,    42,    -1,    44,    45,    46,    47,    -1,    -1,    -1,
      -1,    52,    53,    54,    55,    -1,    -1,    58,    59,    60,
      61,    -1,    -1,    64,    65,    -1,    67,    -1,    69,    70,
      -1,    -1,    12,    13,    14,    76,    16,    17,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    39,
      40,    41,    42,    43,    44,    45,    46,    47,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    55,    -1,    57,    58,    59,
      60,    61,    -1,    63,    12,    13,    14,    67,    16,    17,
      70,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    79,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    39,    40,    41,    42,    43,    44,    45,    46,    47,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    55,    -1,    57,
      58,    59,    60,    61,    -1,    63,    12,    13,    14,    67,
      16,    17,    70,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    79,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    39,    40,    41,    42,    43,    44,    45,
      46,    47,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    55,
      -1,    57,    58,    59,    60,    61,    -1,    63,    12,    13,
      14,    67,    16,    17,    70,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    79,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    39,    40,    41,    42,    43,
      44,    45,    46,    47,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    55,    -1,    57,    58,    59,    60,    61,    -1,    63,
      12,    13,    14,    67,    16,    17,    70,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    79,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    39,    40,    41,
      42,    43,    44,    45,    46,    47,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    55,    -1,    57,    58,    59,    60,    61,
      -1,    63,    12,    13,    14,    67,    16,    17,    70,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    79,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,     8,    -1,    -1,    -1,    39,
      40,    41,    42,    43,    44,    45,    46,    47,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    55,    -1,    57,    58,    59,
      60,    61,    -1,    63,    -1,    -1,    39,    67,    -1,    42,
      70,    44,    -1,    -1,    47,    -1,    -1,    -1,    -1,    79,
      -1,    54,    55,    -1,    -1,    -1,    -1,    60,    61,    -1,
      -1,    -1,    -1,    -1,    67,    -1,    69,    70,    12,    13,
      14,    -1,    16,    17,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    37,    -1,    39,    40,    41,    42,    43,
      44,    45,    46,    47,    12,    13,    14,    -1,    16,    17,
      -1,    55,    -1,    57,    58,    59,    60,    61,    -1,    63,
      -1,    -1,    -1,    67,    -1,    -1,    70,    -1,    -1,    -1,
      -1,    39,    40,    41,    42,    43,    44,    45,    46,    47,
      -1,    -1,    -1,    -1,    -1,    -1,    -1,    55,    -1,    57,
      58,    59,    60,    61,    27,    63,    -1,    -1,    -1,    67,
      -1,    34,    70,    -1,    -1,    -1,    -1,    -1,    41,    42,
      -1,    -1,    -1,    46,    47,    -1,    -1,    -1,    -1,    -1,
      53,    -1,    55,    -1,    -1,    -1,    59,    -1,    61,    -1,
      -1,    -1,    65,    -1,    67,    -1,    -1,    -1,    -1,    -1,
      -1,    -1,    -1,    76
};

  /* YYSTOS[STATE-NUM] -- The (internal number of the) accessing
     symbol of state STATE-NUM.  */
static const yytype_uint8 yystos[] =
{
       0,    10,    83,    85,    86,    36,     0,    37,    86,    87,
      88,    89,    93,    72,     4,     5,    36,    84,    88,    38,
      92,    72,    75,     4,    94,    96,    73,    90,    91,    92,
      38,     3,     5,    96,    97,    98,   100,   105,   106,    34,
      73,    76,    36,    38,    70,   106,    72,    76,    76,   104,
      78,    74,    34,     6,     7,     8,     9,    26,    27,    33,
      34,    39,    40,    41,    42,    44,    45,    46,    47,    52,
      53,    54,    55,    58,    59,    60,    61,    64,    65,    67,
      69,    70,    71,    76,    95,   125,   127,   128,   129,   130,
     131,   132,   133,   134,   137,   138,   139,   140,    38,    63,
      64,    65,    66,    67,    68,    99,     8,    69,    95,    36,
      77,   101,   102,   103,    72,    87,    74,    75,    75,   127,
      76,   125,   129,   130,    73,   135,   136,   136,   136,   136,
      76,    76,    76,    76,    76,    76,    76,    75,   125,   127,
      77,    18,    19,    20,    21,    22,    23,    27,    28,    30,
      31,    18,    19,    24,    25,    75,    18,    19,    18,    19,
      81,    48,    49,    50,    51,    77,    77,    77,    39,    40,
      41,    42,    43,    47,    51,    92,    75,    77,    75,    77,
      12,    13,    14,    16,    17,    39,    40,    41,    42,    43,
      57,    58,    59,    60,    61,    63,    70,   108,   109,   110,
     111,   113,   114,   118,   119,   121,   123,   131,   132,   133,
     134,   137,   138,   139,   140,    34,    34,   125,   125,    73,
      11,    34,    35,    41,    43,    68,    76,   115,   116,   117,
     125,   126,   127,   128,   129,   115,   115,   115,   115,   115,
     115,   115,    77,    77,    72,   125,   125,   125,   125,   125,
     125,   125,   125,   125,   125,   127,   127,   127,   127,    34,
     128,   130,   128,   129,   130,   129,    78,    78,   107,    36,
     101,    78,    36,    72,   112,   125,   126,   127,   129,    76,
      76,    72,    76,    80,    80,    80,    80,    80,    76,    76,
      76,    76,    76,    76,    76,   109,   111,    72,    72,    15,
      80,    80,    80,    80,    80,    80,    80,    80,    74,   125,
      39,    40,    41,    42,    43,    76,   126,    75,    77,    29,
      77,    77,    77,    77,    77,    77,    87,    87,    78,    87,
      92,    72,   127,   127,    34,   128,   127,   125,   129,    34,
      41,    76,   126,   115,   115,   115,   115,   115,   115,   115,
      79,    78,   120,   121,   125,   127,   128,   129,   125,   127,
     129,   128,    74,   115,    77,   116,   125,   126,   127,   110,
     110,    87,   110,    77,    77,    77,    77,    77,    77,    77,
      77,    77,    77,   110,    77,    79,    79,   110,   109,   124,
     122,    72,    72,    72,    79,    79,    79,    78,    78,   110,
     110,    79,    79
};

  /* YYR1[YYN] -- Symbol number of symbol that rule YYN derives.  */
static const yytype_uint8 yyr1[] =
{
       0,    82,    84,    83,    85,    85,    85,    86,    87,    87,
      87,    88,    89,    89,    89,    90,    91,    91,    92,    92,
      93,    93,    94,    94,    94,    95,    95,    95,    95,    95,
      95,    96,    97,    97,    98,    98,    99,    99,    99,    99,
      99,    99,   100,   100,   101,   101,   101,   101,   101,   101,
     101,   102,   102,   103,   103,   104,   104,   105,   105,   106,
     106,   106,   107,   106,   106,   106,   106,   108,   108,   109,
     109,   110,   110,   110,   111,   111,   111,   111,   111,   111,
     111,   111,   111,   112,   112,   112,   112,   113,   113,   113,
     113,   113,   113,   113,   113,   113,   113,   113,   113,   113,
     114,   114,   114,   114,   114,   115,   115,   115,   116,   116,
     116,   116,   116,   116,   117,   117,   117,   117,   117,   118,
     118,   119,   119,   120,   122,   121,   124,   123,   125,   125,
     125,   125,   125,   125,   125,   125,   125,   125,   125,   125,
     125,   126,   126,   126,   126,   126,   126,   126,   126,   126,
     127,   127,   127,   127,   127,   127,   127,   127,   127,   127,
     127,   127,   127,   127,   127,   127,   127,   127,   127,   127,
     127,   127,   127,   128,   128,   128,   128,   128,   128,   128,
     128,   129,   129,   129,   129,   129,   129,   130,   131,   132,
     133,   134,   135,   136,   136,   137,   138,   139,   140
};

  /* YYR2[YYN] -- Number of symbols on the right hand side of rule YYN.  */
static const yytype_int8 yyr2[] =
{
       0,     2,     0,     5,     0,     1,     2,     3,     0,     1,
       2,     2,     2,     2,     2,     3,     1,     4,     1,     2,
       2,     3,     0,     1,     2,     3,     3,     3,     1,     1,
       3,     6,     1,     2,     2,     2,     1,     1,     1,     1,
       1,     1,     3,     3,     2,     2,     2,     2,     2,     2,
       2,     1,     3,     2,     4,     2,     3,     4,     3,     2,
       3,     8,     0,     9,     8,     6,     9,     0,     2,     1,
       3,     0,     1,     2,     2,     2,     5,     5,     1,     1,
       2,     1,     5,     1,     1,     1,     1,     3,     3,     3,
       3,     3,     3,     3,     3,     3,     3,     3,     3,     3,
       4,     4,     4,     4,     4,     0,     1,     3,     1,     1,
       1,     1,     1,     1,     2,     2,     2,     2,     2,     1,
       3,     1,     3,     3,     0,     8,     0,     8,     3,     3,
       3,     3,     2,     3,     4,     4,     1,     1,     1,     1,
       1,     3,     3,     3,     3,     4,     1,     1,     1,     1,
       3,     3,     3,     3,     2,     3,     4,     4,     1,     1,
       1,     1,     1,     3,     3,     3,     3,     3,     3,     3,
       3,     3,     3,     1,     1,     1,     1,     4,     1,     1,
       1,     1,     1,     4,     4,     1,     1,     2,     2,     2,
       2,     2,     3,     1,     4,     2,     2,     2,     2
};


#define yyerrok         (yyerrstatus = 0)
#define yyclearin       (yychar = YYEMPTY)
#define YYEMPTY         (-2)
#define YYEOF           0

#define YYACCEPT        goto yyacceptlab
#define YYABORT         goto yyabortlab
#define YYERROR         goto yyerrorlab


#define YYRECOVERING()  (!!yyerrstatus)

#define YYBACKUP(Token, Value)                                    \
  do                                                              \
    if (yychar == YYEMPTY)                                        \
      {                                                           \
        yychar = (Token);                                         \
        yylval = (Value);                                         \
        YYPOPSTACK (yylen);                                       \
        yystate = *yyssp;                                         \
        goto yybackup;                                            \
      }                                                           \
    else                                                          \
      {                                                           \
        yyerror (YY_("syntax error: cannot back up")); \
        YYERROR;                                                  \
      }                                                           \
  while (0)

/* Error token number */
#define YYTERROR        1
#define YYERRCODE       256



/* Enable debugging if requested.  */
#if YYDEBUG

# ifndef YYFPRINTF
#  include <stdio.h> /* INFRINGES ON USER NAME SPACE */
#  define YYFPRINTF fprintf
# endif

# define YYDPRINTF(Args)                        \
do {                                            \
  if (yydebug)                                  \
    YYFPRINTF Args;                             \
} while (0)

/* This macro is provided for backward compatibility. */
#ifndef YY_LOCATION_PRINT
# define YY_LOCATION_PRINT(File, Loc) ((void) 0)
#endif


# define YY_SYMBOL_PRINT(Title, Type, Value, Location)                    \
do {                                                                      \
  if (yydebug)                                                            \
    {                                                                     \
      YYFPRINTF (stderr, "%s ", Title);                                   \
      yy_symbol_print (stderr,                                            \
                  Type, Value); \
      YYFPRINTF (stderr, "\n");                                           \
    }                                                                     \
} while (0)


/*-----------------------------------.
| Print this symbol's value on YYO.  |
`-----------------------------------*/

static void
yy_symbol_value_print (FILE *yyo, int yytype, YYSTYPE const * const yyvaluep)
{
  FILE *yyoutput = yyo;
  YYUSE (yyoutput);
  if (!yyvaluep)
    return;
# ifdef YYPRINT
  if (yytype < YYNTOKENS)
    YYPRINT (yyo, yytoknum[yytype], *yyvaluep);
# endif
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YYUSE (yytype);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}


/*---------------------------.
| Print this symbol on YYO.  |
`---------------------------*/

static void
yy_symbol_print (FILE *yyo, int yytype, YYSTYPE const * const yyvaluep)
{
  YYFPRINTF (yyo, "%s %s (",
             yytype < YYNTOKENS ? "token" : "nterm", yytname[yytype]);

  yy_symbol_value_print (yyo, yytype, yyvaluep);
  YYFPRINTF (yyo, ")");
}

/*------------------------------------------------------------------.
| yy_stack_print -- Print the state stack from its BOTTOM up to its |
| TOP (included).                                                   |
`------------------------------------------------------------------*/

static void
yy_stack_print (yy_state_t *yybottom, yy_state_t *yytop)
{
  YYFPRINTF (stderr, "Stack now");
  for (; yybottom <= yytop; yybottom++)
    {
      int yybot = *yybottom;
      YYFPRINTF (stderr, " %d", yybot);
    }
  YYFPRINTF (stderr, "\n");
}

# define YY_STACK_PRINT(Bottom, Top)                            \
do {                                                            \
  if (yydebug)                                                  \
    yy_stack_print ((Bottom), (Top));                           \
} while (0)


/*------------------------------------------------.
| Report that the YYRULE is going to be reduced.  |
`------------------------------------------------*/

static void
yy_reduce_print (yy_state_t *yyssp, YYSTYPE *yyvsp, int yyrule)
{
  int yylno = yyrline[yyrule];
  int yynrhs = yyr2[yyrule];
  int yyi;
  YYFPRINTF (stderr, "Reducing stack by rule %d (line %d):\n",
             yyrule - 1, yylno);
  /* The symbols being reduced.  */
  for (yyi = 0; yyi < yynrhs; yyi++)
    {
      YYFPRINTF (stderr, "   $%d = ", yyi + 1);
      yy_symbol_print (stderr,
                       yystos[+yyssp[yyi + 1 - yynrhs]],
                       &yyvsp[(yyi + 1) - (yynrhs)]
                                              );
      YYFPRINTF (stderr, "\n");
    }
}

# define YY_REDUCE_PRINT(Rule)          \
do {                                    \
  if (yydebug)                          \
    yy_reduce_print (yyssp, yyvsp, Rule); \
} while (0)

/* Nonzero means print parse trace.  It is left uninitialized so that
   multiple parsers can coexist.  */
int yydebug;
#else /* !YYDEBUG */
# define YYDPRINTF(Args)
# define YY_SYMBOL_PRINT(Title, Type, Value, Location)
# define YY_STACK_PRINT(Bottom, Top)
# define YY_REDUCE_PRINT(Rule)
#endif /* !YYDEBUG */


/* YYINITDEPTH -- initial size of the parser's stacks.  */
#ifndef YYINITDEPTH
# define YYINITDEPTH 200
#endif

/* YYMAXDEPTH -- maximum size the stacks can grow to (effective only
   if the built-in stack extension method is used).

   Do not make this value too large; the results are undefined if
   YYSTACK_ALLOC_MAXIMUM < YYSTACK_BYTES (YYMAXDEPTH)
   evaluated with infinite-precision integer arithmetic.  */

#ifndef YYMAXDEPTH
# define YYMAXDEPTH 10000
#endif


#if YYERROR_VERBOSE

# ifndef yystrlen
#  if defined __GLIBC__ && defined _STRING_H
#   define yystrlen(S) (YY_CAST (YYPTRDIFF_T, strlen (S)))
#  else
/* Return the length of YYSTR.  */
static YYPTRDIFF_T
yystrlen (const char *yystr)
{
  YYPTRDIFF_T yylen;
  for (yylen = 0; yystr[yylen]; yylen++)
    continue;
  return yylen;
}
#  endif
# endif

# ifndef yystpcpy
#  if defined __GLIBC__ && defined _STRING_H && defined _GNU_SOURCE
#   define yystpcpy stpcpy
#  else
/* Copy YYSRC to YYDEST, returning the address of the terminating '\0' in
   YYDEST.  */
static char *
yystpcpy (char *yydest, const char *yysrc)
{
  char *yyd = yydest;
  const char *yys = yysrc;

  while ((*yyd++ = *yys++) != '\0')
    continue;

  return yyd - 1;
}
#  endif
# endif

# ifndef yytnamerr
/* Copy to YYRES the contents of YYSTR after stripping away unnecessary
   quotes and backslashes, so that it's suitable for yyerror.  The
   heuristic is that double-quoting is unnecessary unless the string
   contains an apostrophe, a comma, or backslash (other than
   backslash-backslash).  YYSTR is taken from yytname.  If YYRES is
   null, do not copy; instead, return the length of what the result
   would have been.  */
static YYPTRDIFF_T
yytnamerr (char *yyres, const char *yystr)
{
  if (*yystr == '"')
    {
      YYPTRDIFF_T yyn = 0;
      char const *yyp = yystr;

      for (;;)
        switch (*++yyp)
          {
          case '\'':
          case ',':
            goto do_not_strip_quotes;

          case '\\':
            if (*++yyp != '\\')
              goto do_not_strip_quotes;
            else
              goto append;

          append:
          default:
            if (yyres)
              yyres[yyn] = *yyp;
            yyn++;
            break;

          case '"':
            if (yyres)
              yyres[yyn] = '\0';
            return yyn;
          }
    do_not_strip_quotes: ;
    }

  if (yyres)
    return yystpcpy (yyres, yystr) - yyres;
  else
    return yystrlen (yystr);
}
# endif

/* Copy into *YYMSG, which is of size *YYMSG_ALLOC, an error message
   about the unexpected token YYTOKEN for the state stack whose top is
   YYSSP.

   Return 0 if *YYMSG was successfully written.  Return 1 if *YYMSG is
   not large enough to hold the message.  In that case, also set
   *YYMSG_ALLOC to the required number of bytes.  Return 2 if the
   required number of bytes is too large to store.  */
static int
yysyntax_error (YYPTRDIFF_T *yymsg_alloc, char **yymsg,
                yy_state_t *yyssp, int yytoken)
{
  enum { YYERROR_VERBOSE_ARGS_MAXIMUM = 5 };
  /* Internationalized format string. */
  const char *yyformat = YY_NULLPTR;
  /* Arguments of yyformat: reported tokens (one for the "unexpected",
     one per "expected"). */
  char const *yyarg[YYERROR_VERBOSE_ARGS_MAXIMUM];
  /* Actual size of YYARG. */
  int yycount = 0;
  /* Cumulated lengths of YYARG.  */
  YYPTRDIFF_T yysize = 0;

  /* There are many possibilities here to consider:
     - If this state is a consistent state with a default action, then
       the only way this function was invoked is if the default action
       is an error action.  In that case, don't check for expected
       tokens because there are none.
     - The only way there can be no lookahead present (in yychar) is if
       this state is a consistent state with a default action.  Thus,
       detecting the absence of a lookahead is sufficient to determine
       that there is no unexpected or expected token to report.  In that
       case, just report a simple "syntax error".
     - Don't assume there isn't a lookahead just because this state is a
       consistent state with a default action.  There might have been a
       previous inconsistent state, consistent state with a non-default
       action, or user semantic action that manipulated yychar.
     - Of course, the expected token list depends on states to have
       correct lookahead information, and it depends on the parser not
       to perform extra reductions after fetching a lookahead from the
       scanner and before detecting a syntax error.  Thus, state merging
       (from LALR or IELR) and default reductions corrupt the expected
       token list.  However, the list is correct for canonical LR with
       one exception: it will still contain any token that will not be
       accepted due to an error action in a later state.
  */
  if (yytoken != YYEMPTY)
    {
      int yyn = yypact[+*yyssp];
      YYPTRDIFF_T yysize0 = yytnamerr (YY_NULLPTR, yytname[yytoken]);
      yysize = yysize0;
      yyarg[yycount++] = yytname[yytoken];
      if (!yypact_value_is_default (yyn))
        {
          /* Start YYX at -YYN if negative to avoid negative indexes in
             YYCHECK.  In other words, skip the first -YYN actions for
             this state because they are default actions.  */
          int yyxbegin = yyn < 0 ? -yyn : 0;
          /* Stay within bounds of both yycheck and yytname.  */
          int yychecklim = YYLAST - yyn + 1;
          int yyxend = yychecklim < YYNTOKENS ? yychecklim : YYNTOKENS;
          int yyx;

          for (yyx = yyxbegin; yyx < yyxend; ++yyx)
            if (yycheck[yyx + yyn] == yyx && yyx != YYTERROR
                && !yytable_value_is_error (yytable[yyx + yyn]))
              {
                if (yycount == YYERROR_VERBOSE_ARGS_MAXIMUM)
                  {
                    yycount = 1;
                    yysize = yysize0;
                    break;
                  }
                yyarg[yycount++] = yytname[yyx];
                {
                  YYPTRDIFF_T yysize1
                    = yysize + yytnamerr (YY_NULLPTR, yytname[yyx]);
                  if (yysize <= yysize1 && yysize1 <= YYSTACK_ALLOC_MAXIMUM)
                    yysize = yysize1;
                  else
                    return 2;
                }
              }
        }
    }

  switch (yycount)
    {
# define YYCASE_(N, S)                      \
      case N:                               \
        yyformat = S;                       \
      break
    default: /* Avoid compiler warnings. */
      YYCASE_(0, YY_("syntax error"));
      YYCASE_(1, YY_("syntax error, unexpected %s"));
      YYCASE_(2, YY_("syntax error, unexpected %s, expecting %s"));
      YYCASE_(3, YY_("syntax error, unexpected %s, expecting %s or %s"));
      YYCASE_(4, YY_("syntax error, unexpected %s, expecting %s or %s or %s"));
      YYCASE_(5, YY_("syntax error, unexpected %s, expecting %s or %s or %s or %s"));
# undef YYCASE_
    }

  {
    /* Don't count the "%s"s in the final size, but reserve room for
       the terminator.  */
    YYPTRDIFF_T yysize1 = yysize + (yystrlen (yyformat) - 2 * yycount) + 1;
    if (yysize <= yysize1 && yysize1 <= YYSTACK_ALLOC_MAXIMUM)
      yysize = yysize1;
    else
      return 2;
  }

  if (*yymsg_alloc < yysize)
    {
      *yymsg_alloc = 2 * yysize;
      if (! (yysize <= *yymsg_alloc
             && *yymsg_alloc <= YYSTACK_ALLOC_MAXIMUM))
        *yymsg_alloc = YYSTACK_ALLOC_MAXIMUM;
      return 1;
    }

  /* Avoid sprintf, as that infringes on the user's name space.
     Don't have undefined behavior even if the translation
     produced a string with the wrong number of "%s"s.  */
  {
    char *yyp = *yymsg;
    int yyi = 0;
    while ((*yyp = *yyformat) != '\0')
      if (*yyp == '%' && yyformat[1] == 's' && yyi < yycount)
        {
          yyp += yytnamerr (yyp, yyarg[yyi++]);
          yyformat += 2;
        }
      else
        {
          ++yyp;
          ++yyformat;
        }
  }
  return 0;
}
#endif /* YYERROR_VERBOSE */

/*-----------------------------------------------.
| Release the memory associated to this symbol.  |
`-----------------------------------------------*/

static void
yydestruct (const char *yymsg, int yytype, YYSTYPE *yyvaluep)
{
  YYUSE (yyvaluep);
  if (!yymsg)
    yymsg = "Deleting";
  YY_SYMBOL_PRINT (yymsg, yytype, yyvaluep, yylocationp);

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  YYUSE (yytype);
  YY_IGNORE_MAYBE_UNINITIALIZED_END
}




/* The lookahead symbol.  */
int yychar;

/* The semantic value of the lookahead symbol.  */
YYSTYPE yylval;
/* Number of syntax errors so far.  */
int yynerrs;


/*----------.
| yyparse.  |
`----------*/

int
yyparse (void)
{
    yy_state_fast_t yystate;
    /* Number of tokens to shift before error messages enabled.  */
    int yyerrstatus;

    /* The stacks and their tools:
       'yyss': related to states.
       'yyvs': related to semantic values.

       Refer to the stacks through separate pointers, to allow yyoverflow
       to reallocate them elsewhere.  */

    /* The state stack.  */
    yy_state_t yyssa[YYINITDEPTH];
    yy_state_t *yyss;
    yy_state_t *yyssp;

    /* The semantic value stack.  */
    YYSTYPE yyvsa[YYINITDEPTH];
    YYSTYPE *yyvs;
    YYSTYPE *yyvsp;

    YYPTRDIFF_T yystacksize;

  int yyn;
  int yyresult;
  /* Lookahead token as an internal (translated) token number.  */
  int yytoken = 0;
  /* The variables used to return semantic value and location from the
     action routines.  */
  YYSTYPE yyval;

#if YYERROR_VERBOSE
  /* Buffer for error messages, and its allocated size.  */
  char yymsgbuf[128];
  char *yymsg = yymsgbuf;
  YYPTRDIFF_T yymsg_alloc = sizeof yymsgbuf;
#endif

#define YYPOPSTACK(N)   (yyvsp -= (N), yyssp -= (N))

  /* The number of symbols on the RHS of the reduced rule.
     Keep to zero when no symbol should be popped.  */
  int yylen = 0;

  yyssp = yyss = yyssa;
  yyvsp = yyvs = yyvsa;
  yystacksize = YYINITDEPTH;

  YYDPRINTF ((stderr, "Starting parse\n"));

  yystate = 0;
  yyerrstatus = 0;
  yynerrs = 0;
  yychar = YYEMPTY; /* Cause a token to be read.  */
  goto yysetstate;


/*------------------------------------------------------------.
| yynewstate -- push a new state, which is found in yystate.  |
`------------------------------------------------------------*/
yynewstate:
  /* In all cases, when you get here, the value and location stacks
     have just been pushed.  So pushing a state here evens the stacks.  */
  yyssp++;


/*--------------------------------------------------------------------.
| yysetstate -- set current state (the top of the stack) to yystate.  |
`--------------------------------------------------------------------*/
yysetstate:
  YYDPRINTF ((stderr, "Entering state %d\n", yystate));
  YY_ASSERT (0 <= yystate && yystate < YYNSTATES);
  YY_IGNORE_USELESS_CAST_BEGIN
  *yyssp = YY_CAST (yy_state_t, yystate);
  YY_IGNORE_USELESS_CAST_END

  if (yyss + yystacksize - 1 <= yyssp)
#if !defined yyoverflow && !defined YYSTACK_RELOCATE
    goto yyexhaustedlab;
#else
    {
      /* Get the current used size of the three stacks, in elements.  */
      YYPTRDIFF_T yysize = yyssp - yyss + 1;

# if defined yyoverflow
      {
        /* Give user a chance to reallocate the stack.  Use copies of
           these so that the &'s don't force the real ones into
           memory.  */
        yy_state_t *yyss1 = yyss;
        YYSTYPE *yyvs1 = yyvs;

        /* Each stack pointer address is followed by the size of the
           data in use in that stack, in bytes.  This used to be a
           conditional around just the two extra args, but that might
           be undefined if yyoverflow is a macro.  */
        yyoverflow (YY_("memory exhausted"),
                    &yyss1, yysize * YYSIZEOF (*yyssp),
                    &yyvs1, yysize * YYSIZEOF (*yyvsp),
                    &yystacksize);
        yyss = yyss1;
        yyvs = yyvs1;
      }
# else /* defined YYSTACK_RELOCATE */
      /* Extend the stack our own way.  */
      if (YYMAXDEPTH <= yystacksize)
        goto yyexhaustedlab;
      yystacksize *= 2;
      if (YYMAXDEPTH < yystacksize)
        yystacksize = YYMAXDEPTH;

      {
        yy_state_t *yyss1 = yyss;
        union yyalloc *yyptr =
          YY_CAST (union yyalloc *,
                   YYSTACK_ALLOC (YY_CAST (YYSIZE_T, YYSTACK_BYTES (yystacksize))));
        if (! yyptr)
          goto yyexhaustedlab;
        YYSTACK_RELOCATE (yyss_alloc, yyss);
        YYSTACK_RELOCATE (yyvs_alloc, yyvs);
# undef YYSTACK_RELOCATE
        if (yyss1 != yyssa)
          YYSTACK_FREE (yyss1);
      }
# endif

      yyssp = yyss + yysize - 1;
      yyvsp = yyvs + yysize - 1;

      YY_IGNORE_USELESS_CAST_BEGIN
      YYDPRINTF ((stderr, "Stack size increased to %ld\n",
                  YY_CAST (long, yystacksize)));
      YY_IGNORE_USELESS_CAST_END

      if (yyss + yystacksize - 1 <= yyssp)
        YYABORT;
    }
#endif /* !defined yyoverflow && !defined YYSTACK_RELOCATE */

  if (yystate == YYFINAL)
    YYACCEPT;

  goto yybackup;


/*-----------.
| yybackup.  |
`-----------*/
yybackup:
  /* Do appropriate processing given the current state.  Read a
     lookahead token if we need one and don't already have one.  */

  /* First try to decide what to do without reference to lookahead token.  */
  yyn = yypact[yystate];
  if (yypact_value_is_default (yyn))
    goto yydefault;

  /* Not known => get a lookahead token if don't already have one.  */

  /* YYCHAR is either YYEMPTY or YYEOF or a valid lookahead symbol.  */
  if (yychar == YYEMPTY)
    {
      YYDPRINTF ((stderr, "Reading a token: "));
      yychar = yylex ();
    }

  if (yychar <= YYEOF)
    {
      yychar = yytoken = YYEOF;
      YYDPRINTF ((stderr, "Now at end of input.\n"));
    }
  else
    {
      yytoken = YYTRANSLATE (yychar);
      YY_SYMBOL_PRINT ("Next token is", yytoken, &yylval, &yylloc);
    }

  /* If the proper action on seeing token YYTOKEN is to reduce or to
     detect an error, take that action.  */
  yyn += yytoken;
  if (yyn < 0 || YYLAST < yyn || yycheck[yyn] != yytoken)
    goto yydefault;
  yyn = yytable[yyn];
  if (yyn <= 0)
    {
      if (yytable_value_is_error (yyn))
        goto yyerrlab;
      yyn = -yyn;
      goto yyreduce;
    }

  /* Count tokens shifted since error; after three, turn off error
     status.  */
  if (yyerrstatus)
    yyerrstatus--;

  /* Shift the lookahead token.  */
  YY_SYMBOL_PRINT ("Shifting", yytoken, &yylval, &yylloc);
  yystate = yyn;
  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END

  /* Discard the shifted token.  */
  yychar = YYEMPTY;
  goto yynewstate;


/*-----------------------------------------------------------.
| yydefault -- do the default action for the current state.  |
`-----------------------------------------------------------*/
yydefault:
  yyn = yydefact[yystate];
  if (yyn == 0)
    goto yyerrlab;
  goto yyreduce;


/*-----------------------------.
| yyreduce -- do a reduction.  |
`-----------------------------*/
yyreduce:
  /* yyn is the number of a rule to reduce with.  */
  yylen = yyr2[yyn];

  /* If YYLEN is nonzero, implement the default value of the action:
     '$$ = $1'.

     Otherwise, the following line sets YYVAL to garbage.
     This behavior is undocumented and Bison
     users should not rely upon it.  Assigning to YYVAL
     unconditionally makes the parser a bit smaller, and it avoids a
     GCC warning that YYVAL may be used uninitialized.  */
  yyval = yyvsp[1-yylen];


  YY_REDUCE_PRINT (yyn);
  switch (yyn)
    {
  case 2:
#line 1798 "script_parser.y"
                                {
					/* stop parsing here if we are just getting */
					/* the variables in the script */
					/* YYACCEPT */
				}
#line 3602 "script_parser.c"
    break;

  case 3:
#line 1804 "script_parser.y"
                                {
					SDWORD			size, debug_i, i, dimension, arraySize, totalArraySize;
					SDWORD			numArrays;
					UDWORD			base;
					VAR_SYMBOL		*psCurr;
					TRIGGER_SYMBOL	*psTrig;
					EVENT_SYMBOL	*psEvent;
					UDWORD			numVars;

					INTERP_TYPE		*pCurEvLocalVars;
					UDWORD		j;

					// Calculate the code size
					size = 0;
					debug_i = 0;
					for(psTrig = psTriggers; psTrig; psTrig = psTrig->psNext)
					{
						// Add the trigger code size
						size += psTrig->size;
						debug_i += psTrig->debugEntries;
					}

					for(psEvent = psEvents; psEvent; psEvent = psEvent->psNext)
					{
						// Add the trigger code size
						size += psEvent->size;
						debug_i += psEvent->debugEntries;
					}

					// Allocate the program
					numVars = psGlobalVars ? psGlobalVars->index+1 : 0;
					numArrays = psGlobalArrays ? psGlobalArrays->index+1 : 0;
					totalArraySize = 0;
					for(psCurr=psGlobalArrays; psCurr; psCurr=psCurr->psNext)
					{
						arraySize = 1;
						for(dimension = 0; dimension < psCurr->dimensions; dimension += 1)
						{
							arraySize *= psCurr->elements[dimension];
						}
						totalArraySize += arraySize;
					}
					ALLOC_PROG(psFinalProg, size, psCurrBlock->pCode,
						numVars, numArrays, numTriggers, numEvents);

					//store local vars
					//allocate array for holding an array of local vars for each event
					psFinalProg->ppsLocalVars = (INTERP_TYPE **)MALLOC(sizeof(INTERP_TYPE*) * numEvents);
					psFinalProg->numLocalVars = (UDWORD *)MALLOC(sizeof(UDWORD) * numEvents);	//how many local vars each event has
					psFinalProg->numParams = (UDWORD *)MALLOC(sizeof(UDWORD) * numEvents);	//how many arguments each event has

					i=0;
					for(psEvent = psEvents; psEvent; psEvent = psEvent->psNext)
					{
						psEvent->numLocalVars = numEventLocalVars[i];

						psFinalProg->numLocalVars[i] = numEventLocalVars[i];	//remember how many local vars this event has
						psFinalProg->numParams[i] = psEvent->numParams;	//remember how many parameters this event has

						if(numEventLocalVars[i] > 0)
						{
							pCurEvLocalVars = (INTERP_TYPE*)MALLOC(sizeof(INTERP_TYPE) * numEventLocalVars[i]);

							j=0;
							for(psCurr =psLocalVarsB[i]; psCurr != NULL; psCurr = psCurr->psNext)
							{
								//debug(LOG_SCRIPT, "remembering loc var ");
								//debug(LOG_SCRIPT, "%d - %d \n",i,j);
								pCurEvLocalVars[numEventLocalVars[i] - j - 1] = psCurr->type;	//save type, order is reversed
								j++;
							}
						}
						else
						{
							pCurEvLocalVars = NULL;	//this event has no local vars
						}

						psFinalProg->ppsLocalVars[i] = pCurEvLocalVars;
						i++;
					}



					ALLOC_DEBUG(psFinalProg, debug_i);
					psFinalProg->debugEntries = 0;
					ip = psFinalProg->pCode;

					// Add the trigger code
					i=0;
					for(psTrig = psTriggers; psTrig; psTrig = psTrig->psNext)
					{
						// Store the trigger offset
						psFinalProg->pTriggerTab[i] = (UWORD)(ip - psFinalProg->pCode);
						if (psTrig->pCode != NULL)
						{
							// Store the label
							DEBUG_LABEL(psFinalProg, psFinalProg->debugEntries,
								psTrig->pIdent);
							// Store debug info
							APPEND_DEBUG(psFinalProg, ip - psFinalProg->pCode, psTrig);
							// Store the code
							PUT_BLOCK(ip, psTrig);
							psFinalProg->psTriggerData[i].code = TRUE;
						}
						else
						{
							psFinalProg->psTriggerData[i].code = FALSE;
						}
						// Store the data
						psFinalProg->psTriggerData[i].type = (UWORD)psTrig->type;
						psFinalProg->psTriggerData[i].time = psTrig->time;
						i = i+1;
					}
					// Note the end of the final trigger
					psFinalProg->pTriggerTab[i] = (UWORD)(ip - psFinalProg->pCode);

					// Add the event code
					i=0;
					for(psEvent = psEvents; psEvent; psEvent = psEvent->psNext)
					{
						// Check the event was declared and has a code body
						if (psEvent->pCode == NULL)
						{
							scr_error("Event %s declared without being defined",
								psEvent->pIdent);
							YYABORT;
						}

						// Store the event offset
						psFinalProg->pEventTab[i] = (UWORD)(ip - psFinalProg->pCode);
						// Store the trigger link
						psFinalProg->pEventLinks[i] = (SWORD)(psEvent->trigger);
						// Store the label
						DEBUG_LABEL(psFinalProg, psFinalProg->debugEntries,
							psEvent->pIdent);
						// Store debug info
						APPEND_DEBUG(psFinalProg, ip - psFinalProg->pCode, psEvent);
						// Store the code
						PUT_BLOCK(ip, psEvent);
						i = i+1;
					}
					// Note the end of the final event
					psFinalProg->pEventTab[i] = (UWORD)(ip - psFinalProg->pCode);

					// Allocate debug info for the variables if necessary
					if (genDebugInfo)
					{
						if (numVars > 0)
						{
							psFinalProg->psVarDebug = MALLOC(sizeof(VAR_DEBUG) * numVars);
							if (psFinalProg->psVarDebug == NULL)
							{
								scr_error("Out of memory");
								YYABORT;
							}
						}
						else
						{
							psFinalProg->psVarDebug = NULL;
						}
						if (numArrays > 0)
						{
							psFinalProg->psArrayDebug = MALLOC(sizeof(ARRAY_DEBUG) * numArrays);
							if (psFinalProg->psArrayDebug == NULL)
							{
								scr_error("Out of memory");
								YYABORT;
							}
						}
						else
						{
							psFinalProg->psArrayDebug = NULL;
						}
					}
					else
					{
						psFinalProg->psVarDebug = NULL;
						psFinalProg->psArrayDebug = NULL;
					}

					/* Now set the types for the global variables */
					for(psCurr = psGlobalVars; psCurr != NULL; psCurr = psCurr->psNext)
					{
						i = psCurr->index;
						psFinalProg->pGlobals[i] = psCurr->type;

						if (genDebugInfo)
						{
							psFinalProg->psVarDebug[i].pIdent =
										MALLOC(strlen(psCurr->pIdent) + 1);
							if (psFinalProg->psVarDebug[i].pIdent == NULL)
							{
								scr_error("Out of memory");
								YYABORT;
							}
							strcpy(psFinalProg->psVarDebug[i].pIdent, psCurr->pIdent);
							psFinalProg->psVarDebug[i].storage = psCurr->storage;
						}
					}

					/* Now store the array info */
					psFinalProg->arraySize = totalArraySize;
					for(psCurr = psGlobalArrays; psCurr != NULL; psCurr = psCurr->psNext)
					{
						i = psCurr->index;

						psFinalProg->psArrayInfo[i].type = (UBYTE)psCurr->type;
						psFinalProg->psArrayInfo[i].dimensions = (UBYTE)psCurr->dimensions;
						for(dimension=0; dimension < psCurr->dimensions; dimension += 1)
						{
							psFinalProg->psArrayInfo[i].elements[dimension] = (UBYTE)psCurr->elements[dimension];
						}

						if (genDebugInfo)
						{
							psFinalProg->psArrayDebug[i].pIdent =
										MALLOC(strlen(psCurr->pIdent) + 1);
							if (psFinalProg->psArrayDebug[i].pIdent == NULL)
							{
								scr_error("Out of memory");
								YYABORT;
							}
							strcpy(psFinalProg->psArrayDebug[i].pIdent, psCurr->pIdent);
							psFinalProg->psArrayDebug[i].storage = psCurr->storage;
						}
					}
					// calculate the base index of each array
					base = psFinalProg->numGlobals;
					for(i=0; i<numArrays; i++)
					{
						psFinalProg->psArrayInfo[i].base = base;

						arraySize = 1;
						for(dimension = 0; dimension < psFinalProg->psArrayInfo[i].dimensions; dimension += 1)
						{
							arraySize *= psFinalProg->psArrayInfo[i].elements[dimension];
						}

						base += arraySize;
					}
				}
#line 3848 "script_parser.c"
    break;

  case 7:
#line 2058 "script_parser.y"
                                        {
//						if (!scriptAddVariable("owner", $2, ST_PUBLIC, 0))
//						{
							// Out of memory - error already given
//							YYABORT;
//						}
					}
#line 3860 "script_parser.c"
    break;

  case 9:
#line 2075 "script_parser.y"
                                {
					FREE_VARDECL((yyvsp[0].vdecl));
				}
#line 3868 "script_parser.c"
    break;

  case 10:
#line 2079 "script_parser.y"
                                {
					FREE_VARDECL((yyvsp[0].vdecl));
				}
#line 3876 "script_parser.c"
    break;

  case 11:
#line 2085 "script_parser.y"
                {
			/* remember that local var declaration is over */
			localVariableDef = FALSE;
			//debug(LOG_SCRIPT, "localVariableDef = FALSE 0");
			(yyval.vdecl) = (yyvsp[-1].vdecl);
		}
#line 3887 "script_parser.c"
    break;

  case 12:
#line 2095 "script_parser.y"
                                                {
							//debug(LOG_SCRIPT, "variable_decl_head:		STORAGE TYPE");

							ALLOC_VARDECL(psCurrVDecl);
							psCurrVDecl->storage = (yyvsp[-1].stype);
							psCurrVDecl->type = (yyvsp[0].tval);

							/* allow local vars to have the same names as global vars (don't check global vars) */
							if((yyvsp[-1].stype) == ST_LOCAL)
							{
								localVariableDef = TRUE;
								//debug(LOG_SCRIPT, "localVariableDef = TRUE 0");
							}

							(yyval.vdecl) = psCurrVDecl;
							//debug(LOG_SCRIPT, "END variable_decl_head:		STORAGE TYPE (TYPE=%d)", $2);
						}
#line 3909 "script_parser.c"
    break;

  case 13:
#line 2113 "script_parser.y"
                                                {

							ALLOC_VARDECL(psCurrVDecl);
							psCurrVDecl->storage = (yyvsp[-1].stype);
							psCurrVDecl->type = VAL_TRIGGER;

							(yyval.vdecl) = psCurrVDecl;
						}
#line 3922 "script_parser.c"
    break;

  case 14:
#line 2122 "script_parser.y"
                                                {
							ALLOC_VARDECL(psCurrVDecl);
							psCurrVDecl->storage = (yyvsp[-1].stype);
							psCurrVDecl->type = VAL_EVENT;

							(yyval.vdecl) = psCurrVDecl;
						}
#line 3934 "script_parser.c"
    break;

  case 15:
#line 2132 "script_parser.y"
                                        {
						if ((yyvsp[-1].ival) <= 0 || (yyvsp[-1].ival) >= VAR_MAX_ELEMENTS)
						{
							scr_error("Invalid array size %d", (yyvsp[-1].ival));
							YYABORT;
						}

						ALLOC_VARIDENTDECL(psCurrVIdentDecl, NULL, 1);
						psCurrVIdentDecl->elements[0] = (yyvsp[-1].ival);

						(yyval.videcl) = psCurrVIdentDecl;
					}
#line 3951 "script_parser.c"
    break;

  case 16:
#line 2147 "script_parser.y"
                                        {
						(yyval.videcl) = (yyvsp[0].videcl);
					}
#line 3959 "script_parser.c"
    break;

  case 17:
#line 2152 "script_parser.y"
                                        {
						if ((yyvsp[-3].videcl)->dimensions >= VAR_MAX_DIMENSIONS)
						{
							scr_error("Too many dimensions for array");
							YYABORT;
						}
						if ((yyvsp[-1].ival) <= 0 || (yyvsp[-1].ival) >= VAR_MAX_ELEMENTS)
						{
							scr_error("Invalid array size %d", (yyvsp[-1].ival));
							YYABORT;
						}

						(yyvsp[-3].videcl)->elements[(yyvsp[-3].videcl)->dimensions] = (yyvsp[-1].ival);
						(yyvsp[-3].videcl)->dimensions += 1;

						(yyval.videcl) = (yyvsp[-3].videcl);
					}
#line 3981 "script_parser.c"
    break;

  case 18:
#line 2172 "script_parser.y"
                                        {
						ALLOC_VARIDENTDECL(psCurrVIdentDecl, (yyvsp[0].sval), 0);

						(yyval.videcl) = psCurrVIdentDecl;
					}
#line 3991 "script_parser.c"
    break;

  case 19:
#line 2179 "script_parser.y"
                                        {
						(yyvsp[0].videcl)->pIdent = MALLOC(strlen((yyvsp[-1].sval))+1);
						if ((yyvsp[0].videcl)->pIdent == NULL)
						{
							scr_error("Out of memory");
							YYABORT;
						}
						strcpy((yyvsp[0].videcl)->pIdent, (yyvsp[-1].sval));

						(yyval.videcl) = (yyvsp[0].videcl);
					}
#line 4007 "script_parser.c"
    break;

  case 20:
#line 2193 "script_parser.y"
                                        {
						if (!scriptAddVariable((yyvsp[-1].vdecl), (yyvsp[0].videcl)))
						{
							/* Out of memory - error already given */
							YYABORT;
						}

						FREE_VARIDENTDECL((yyvsp[0].videcl));

						/* return the variable type */
						(yyval.vdecl) = (yyvsp[-1].vdecl);
					}
#line 4024 "script_parser.c"
    break;

  case 21:
#line 2206 "script_parser.y"
                                        {
						if (!scriptAddVariable((yyvsp[-2].vdecl), (yyvsp[0].videcl)))
						{
							/* Out of memory - error already given */
							YYABORT;
						}

						FREE_VARIDENTDECL((yyvsp[0].videcl));

						/* return the variable type */
						(yyval.vdecl) = (yyvsp[-2].vdecl);
					}
#line 4041 "script_parser.c"
    break;

  case 25:
#line 2231 "script_parser.y"
                                        {
						ALLOC_TSUBDECL(psCurrTDecl, TR_CODE, (yyvsp[-2].cblock)->size, (yyvsp[0].ival));
						ip = psCurrTDecl->pCode;
						PUT_BLOCK(ip, (yyvsp[-2].cblock));
						FREE_BLOCK((yyvsp[-2].cblock));

						(yyval.tdecl) = psCurrTDecl;
					}
#line 4054 "script_parser.c"
    break;

  case 26:
#line 2240 "script_parser.y"
                                        {
						ALLOC_TSUBDECL(psCurrTDecl, TR_WAIT, 0, (yyvsp[0].ival));

						(yyval.tdecl) = psCurrTDecl;
					}
#line 4064 "script_parser.c"
    break;

  case 27:
#line 2246 "script_parser.y"
                                        {
						ALLOC_TSUBDECL(psCurrTDecl, TR_EVERY, 0, (yyvsp[0].ival));

						(yyval.tdecl) = psCurrTDecl;
					}
#line 4074 "script_parser.c"
    break;

  case 28:
#line 2252 "script_parser.y"
                                        {
						ALLOC_TSUBDECL(psCurrTDecl, TR_INIT, 0, 0);

						(yyval.tdecl) = psCurrTDecl;
					}
#line 4084 "script_parser.c"
    break;

  case 29:
#line 2258 "script_parser.y"
                                        {
						if ((yyvsp[0].cbSymbol)->numParams != 0)
						{
							scr_error("Expected parameters for callback trigger");
							YYABORT;
						}

						ALLOC_TSUBDECL(psCurrTDecl, (yyvsp[0].cbSymbol)->type, 0, 0);

						(yyval.tdecl) = psCurrTDecl;
					}
#line 4100 "script_parser.c"
    break;

  case 30:
#line 2270 "script_parser.y"
                                        {
						codeRet = scriptCodeCallbackParams((yyvsp[-2].cbSymbol), (yyvsp[0].pblock), &psCurrTDecl);
						CHECK_CODE_ERROR(codeRet);

						(yyval.tdecl) = psCurrTDecl;
					}
#line 4111 "script_parser.c"
    break;

  case 31:
#line 2279 "script_parser.y"
                                        {
						SDWORD	line;
						STRING	*pDummy;

						scriptGetErrorData(&line, &pDummy);
						if (!scriptAddTrigger((yyvsp[-4].sval), (yyvsp[-2].tdecl), (UDWORD)line))
						{
							YYABORT;
						}
						FREE_TSUBDECL((yyvsp[-2].tdecl));
					}
#line 4127 "script_parser.c"
    break;

  case 34:
#line 2302 "script_parser.y"
                                        {
						EVENT_SYMBOL	*psEvent;

						//debug(LOG_SCRIPT, "event_subdecl:		EVENT IDENT");

						if (!scriptDeclareEvent((yyvsp[0].sval), &psEvent,0))
						{
							YYABORT;
						}

						psCurEvent = psEvent;

						(yyval.eSymbol) = psEvent;

						//debug(LOG_SCRIPT, "END event_subdecl:		EVENT IDENT");
					}
#line 4148 "script_parser.c"
    break;

  case 35:
#line 2319 "script_parser.y"
                                                {
							//debug(LOG_SCRIPT, "EVENT EVENT_SYM");

							psCurEvent = (yyvsp[0].eSymbol);
							(yyval.eSymbol) = (yyvsp[0].eSymbol);

							//debug(LOG_SCRIPT, "END EVENT EVENT_SYM");
						}
#line 4161 "script_parser.c"
    break;

  case 42:
#line 2339 "script_parser.y"
                                        {
						EVENT_SYMBOL	*psEvent;

						//debug(LOG_SCRIPT, "func_subdecl:FUNCTION TYPE IDENT (type=%d)", $2);

						/* allow local vars to have the same names as global vars (don't check global vars) */
						localVariableDef = TRUE;
						//debug(LOG_SCRIPT, "localVariableDef = TRUE 1");

						if (!scriptDeclareEvent((yyvsp[0].sval), &psEvent,0))
						{
							YYABORT;
						}

						psEvent->retType = (yyvsp[-1].tval);
						psCurEvent = psEvent;
						psCurEvent->bFunction = TRUE;

						(yyval.eSymbol) = psEvent;

						//debug(LOG_SCRIPT, "END func_subdecl:FUNCTION TYPE IDENT. ");
					}
#line 4188 "script_parser.c"
    break;

  case 43:
#line 2362 "script_parser.y"
                                                {
							//debug(LOG_SCRIPT, "func_subdecl:FUNCTION EVENT_SYM ");
							psCurEvent = (yyvsp[0].eSymbol);


							/* check if this event was declared as function before */
							if(!(yyvsp[0].eSymbol)->bFunction)
							{
								debug(LOG_ERROR, "'%s' was declared as event before and can't be redefined to function", (yyvsp[0].eSymbol)->pIdent);
								scr_error("Wrong event definition");
								YYABORT;
							}

							/* psCurEvent->bFunction = TRUE; */
							/* psEvent->retType = $2; */
							(yyval.eSymbol) = (yyvsp[0].eSymbol);
							//debug(LOG_SCRIPT, "func_subdecl:FUNCTION EVENT_SYM. ");
						}
#line 4211 "script_parser.c"
    break;

  case 44:
#line 2383 "script_parser.y"
                                {
					(yyval.integer_val)=(yyvsp[-1].tval);
				}
#line 4219 "script_parser.c"
    break;

  case 45:
#line 2387 "script_parser.y"
                                {
					(yyval.integer_val)=(yyvsp[-1].tval);
				}
#line 4227 "script_parser.c"
    break;

  case 46:
#line 2391 "script_parser.y"
                                {
					(yyval.integer_val)=(yyvsp[-1].tval);
				}
#line 4235 "script_parser.c"
    break;

  case 47:
#line 2395 "script_parser.y"
                                {
					(yyval.integer_val)=(yyvsp[-1].tval);
				}
#line 4243 "script_parser.c"
    break;

  case 48:
#line 2399 "script_parser.y"
                                {
					(yyval.integer_val)=(yyvsp[-1].tval);
				}
#line 4251 "script_parser.c"
    break;

  case 49:
#line 2403 "script_parser.y"
                                {
					(yyval.integer_val)=(yyvsp[-1].tval);
				}
#line 4259 "script_parser.c"
    break;

  case 50:
#line 2407 "script_parser.y"
                                {
					(yyval.integer_val)=(yyvsp[-1].tval);
				}
#line 4267 "script_parser.c"
    break;

  case 51:
#line 2414 "script_parser.y"
                                {
					if(!checkFuncParamType(0, (yyvsp[0].integer_val)))
					{
						scr_error("Wrong event argument definition in '%s'", psCurEvent->pIdent);
						YYABORT;
					}

					//debug(LOG_SCRIPT, "funcbody_var_def=%d \n", $1);
					(yyval.integer_val)=1;
				}
#line 4282 "script_parser.c"
    break;

  case 52:
#line 2425 "script_parser.y"
                                {
					if(!checkFuncParamType((yyvsp[-2].integer_val), (yyvsp[0].integer_val)))
					{
						scr_error("Wrong event argument definition");
						YYABORT;
					}

					//debug(LOG_SCRIPT, "funcbody_var_def2=%d \n", $3);
					(yyval.integer_val)=(yyvsp[-2].integer_val)+1;
				}
#line 4297 "script_parser.c"
    break;

  case 53:
#line 2440 "script_parser.y"
                                {
					//debug(LOG_SCRIPT, "argument_decl_head 0 ");

					/* handle type part */
					ALLOC_VARDECL(psCurrVDecl);
					psCurrVDecl->storage =ST_LOCAL;	/* can only be local */
					psCurrVDecl->type = (yyvsp[-1].tval);

					/* handle ident part */
					if (!scriptAddVariable(psCurrVDecl, (yyvsp[0].videcl)))
					{
						YYABORT;
					}

					FREE_VARIDENTDECL((yyvsp[0].videcl));

					FREE_VARDECL(psCurrVDecl);

					/* return the variable type */
					/* $$ = psCurrVDecl; */ /* not needed? */

					if(psCurEvent == NULL)
						debug(LOG_ERROR, "argument_decl_head 0:  - psCurEvent == NULL");

					psCurEvent->numParams = psCurEvent->numParams + 1;	/* remember a parameter was declared */

					/* remember parameter type */
					psCurEvent->aParams[0] = (yyvsp[-1].tval);

					//debug(LOG_SCRIPT, "argument_decl_head 0. ");
				}
#line 4333 "script_parser.c"
    break;

  case 54:
#line 2472 "script_parser.y"
                                {
					//debug(LOG_SCRIPT, "argument_decl_head 1 ");

					/* handle type part */
					ALLOC_VARDECL(psCurrVDecl);
					psCurrVDecl->storage =ST_LOCAL;	/* can only be local */
					psCurrVDecl->type = (yyvsp[-1].tval);

					/* remember parameter type */
					psCurEvent->aParams[psCurEvent->numParams] = (yyvsp[-1].tval);
					//debug(LOG_SCRIPT, "argument_decl_head 10 ");

					/* handle ident part */
					if (!scriptAddVariable(psCurrVDecl, (yyvsp[0].videcl)))
					{
						YYABORT;
					}
					//debug(LOG_SCRIPT, "argument_decl_head 11 ");
					FREE_VARIDENTDECL((yyvsp[0].videcl));
					FREE_VARDECL(psCurrVDecl);

					/* return the variable type */
					/* $$ = psCurrVDecl; */ /* not needed? */

					if(psCurEvent == NULL)
						debug(LOG_ERROR, "argument_decl_head 0:  - psCurEvent == NULL");
					psCurEvent->numParams = psCurEvent->numParams + 1;	/* remember a parameter was declared */

					//debug(LOG_SCRIPT, "argument_decl_head 1. ");
				}
#line 4368 "script_parser.c"
    break;

  case 56:
#line 2505 "script_parser.y"
                                {
					/* remember that local var declaration is over */
					localVariableDef = FALSE;
					//debug(LOG_SCRIPT, "localVariableDef = FALSE 1");
				}
#line 4378 "script_parser.c"
    break;

  case 57:
#line 2513 "script_parser.y"
                                {
					/* remember that local var declaration is over */
					localVariableDef = FALSE;
					//debug(LOG_SCRIPT, "localVariableDef = FALSE 2");

					if(psCurEvent->bDeclared) /* can only occur if different (new) var names are used in the event definition that don't match with declaration*/
					{
						scr_error("Wrong event definition: \nEvent %s's definition doesn't match with declaration", psCurEvent->pIdent);
						YYABORT;
					}

					(yyval.eSymbol) = (yyvsp[-3].eSymbol);
				}
#line 4396 "script_parser.c"
    break;

  case 58:
#line 2527 "script_parser.y"
                                {
					/* remember that local var declaration is over */
					localVariableDef = FALSE;
					//debug(LOG_SCRIPT, "localVariableDef = FALSE 3");

					if((yyvsp[-2].eSymbol)->numParams > 0 && psCurEvent->bDeclared) /* can only occur if no parameters or different (new) var names are used in the event definition that don't match with declaration */
					{
						scr_error("Wrong event definition: \nEvent %s's definition doesn't match with declaration", psCurEvent->pIdent);
						YYABORT;
					}

					(yyval.eSymbol) = (yyvsp[-2].eSymbol);
				}
#line 4414 "script_parser.c"
    break;

  case 59:
#line 2545 "script_parser.y"
                                        {
						psCurEvent->bDeclared = TRUE;
					}
#line 4422 "script_parser.c"
    break;

  case 60:
#line 2549 "script_parser.y"
                                        {
						//debug(LOG_SCRIPT, "localVariableDef = FALSE new ");
						localVariableDef = FALSE;
						psCurEvent->bDeclared = TRUE;
					}
#line 4432 "script_parser.c"
    break;

  case 61:
#line 2556 "script_parser.y"
                                        {
						//debug(LOG_SCRIPT, "event_decl 0 ");

						/* make sure this event is not declared as function */
						if(psCurEvent->bFunction)
						{
							debug(LOG_ERROR, "Event '%s' is declared as function and can't have a trigger assigned", psCurEvent->pIdent);
							scr_error("Wrong event definition");
							YYABORT;
						}

						/* pop required number of paramerets passed to this event (if any) */
						/* popArguments($1, $7, &psCurrBlock); */

						/* if (!scriptDefineEvent($1, $6, $3->index)) */
						if (!scriptDefineEvent((yyvsp[-7].eSymbol), (yyvsp[-1].cblock), (yyvsp[-5].tSymbol)->index))
						{
							YYABORT;
						}

						/* end of event */
						psCurEvent = NULL;

						FREE_DEBUG((yyvsp[-1].cblock));
						FREE_BLOCK((yyvsp[-1].cblock));

						//debug(LOG_SCRIPT, "event_decl 0. ");
					}
#line 4465 "script_parser.c"
    break;

  case 62:
#line 2585 "script_parser.y"
                                        {
						// Get the line for the implicit trigger declaration
						STRING	*pDummy;

						//debug(LOG_SCRIPT, "event_decl:event_subdecl '(' trigger_subdecl ')' ");

						/* make sure this event is not declared as function */
						if(psCurEvent->bFunction)
						{
							debug(LOG_ERROR, "Event '%s' is declared as function and can't have a trigger assigned", psCurEvent->pIdent);
							scr_error("Wrong event definition");
							YYABORT;
						}

						scriptGetErrorData((SDWORD *)&debugLine, &pDummy);
						//debug(LOG_SCRIPT, "event_decl:event_subdecl '(' trigger_subdecl ')' .");
					}
#line 4487 "script_parser.c"
    break;

  case 63:
#line 2604 "script_parser.y"
                                        {
						// Create a trigger for this event
						if (!scriptAddTrigger("", (yyvsp[-6].tdecl), debugLine))
						{
							YYABORT;
						}
						FREE_TSUBDECL((yyvsp[-6].tdecl));

						/* if (!scriptDefineEvent($1, $7, numTriggers - 1)) */
						if (!scriptDefineEvent((yyvsp[-8].eSymbol), (yyvsp[-1].cblock), numTriggers - 1))
						{
							YYABORT;
						}

						/* end of event */
						psCurEvent = NULL;

						FREE_DEBUG((yyvsp[-1].cblock));
						 FREE_BLOCK((yyvsp[-1].cblock));

						//debug(LOG_SCRIPT, "event_decl:event_subdecl '(' trigger_subdecl ')'. ");
					}
#line 4514 "script_parser.c"
    break;

  case 64:
#line 2629 "script_parser.y"
                                        {
						//debug(LOG_SCRIPT, "event_decl 1 ");

						/* make sure this event is not declared as function */
						if(psCurEvent->bFunction)
						{
							debug(LOG_ERROR, "Event '%s' is declared as function and can't have a trigger assigned", psCurEvent->pIdent);
							scr_error("Wrong event definition");
							YYABORT;
						}

						if (!scriptDefineEvent((yyvsp[-7].eSymbol), (yyvsp[-1].cblock), -1))
						{
							YYABORT;
						}

						/* end of event */
						psCurEvent = NULL;

						FREE_DEBUG((yyvsp[-1].cblock));
						FREE_BLOCK((yyvsp[-1].cblock));

						//debug(LOG_SCRIPT, "event_decl 1. ");
					}
#line 4543 "script_parser.c"
    break;

  case 65:
#line 2657 "script_parser.y"
                                        {
						UDWORD line;
						STRING *pDummy;

						//debug(LOG_SCRIPT, "func_decl 1 ");

						/* stays the same if no params (just gets copied) */
						ALLOC_BLOCK(psCurrBlock, (yyvsp[-2].cblock)->size + (yyvsp[-1].cblock)->size + sizeof(OPCODE) + (sizeof(OPCODE) * (yyvsp[-5].eSymbol)->numParams)); /* statement_list + expression + EXIT + numParams */
						ip = psCurrBlock->pCode;

						/* pop required number of paramerets passed to this event (if any) */

						popArguments(&ip, (yyvsp[-5].eSymbol)->numParams);

						/* Copy the two code blocks */
						PUT_BLOCK(ip, (yyvsp[-2].cblock));

						PUT_BLOCK(ip, (yyvsp[-1].cblock));

						PUT_OPCODE(ip, OP_EXIT);		/* must exit after return */

						ALLOC_DEBUG(psCurrBlock, (yyvsp[-2].cblock)->debugEntries);

						PUT_DEBUG(psCurrBlock, (yyvsp[-2].cblock));

						if (!scriptDefineEvent((yyvsp[-5].eSymbol), psCurrBlock, -1))
						{
							YYABORT;
						}

						FREE_DEBUG((yyvsp[-2].cblock));

						/* FREE_DEBUG($5); */

						FREE_BLOCK((yyvsp[-2].cblock));

						FREE_BLOCK((yyvsp[-1].cblock));


						/* end of event */
						psCurEvent = NULL;

						/* free block since code was copied in scriptDefineEvent() */
						FREE_DEBUG(psCurrBlock);
						FREE_BLOCK(psCurrBlock);
					}
#line 4594 "script_parser.c"
    break;

  case 66:
#line 2706 "script_parser.y"
                                        {
						//debug(LOG_SCRIPT, "func_decl 3 ");

						if(!psCurEvent->bDeclared)
						{
							debug(LOG_ERROR, "Event %s's definition doesn't match with declaration.", psCurEvent->pIdent);
							scr_error("Wrong event definition:\n event %s's definition doesn't match with declaration", psCurEvent->pIdent);
							YYABORT;
						}

						/* check if number of parameter in body defenition match number of params in the declaration */
						if((yyvsp[-6].integer_val) != psCurEvent->numParams)
						{
							scr_error("Wrong number of arguments in function definition (or declaration-definition argument type/names mismatch) \n in event: '%s'", psCurEvent->pIdent);
							YYABORT;
						}

						/* stays the same if no params (just gets copied) */
						ALLOC_BLOCK(psCurrBlock, (yyvsp[-2].cblock)->size + (yyvsp[-1].cblock)->size + sizeof(OPCODE) + (sizeof(OPCODE) * (yyvsp[-8].eSymbol)->numParams));
						ip = psCurrBlock->pCode;

						/* pop required number of paramerets passed to this event (if any) */
						popArguments(&ip, (yyvsp[-8].eSymbol)->numParams);

						/* Copy the old (main) code and free it */
						PUT_BLOCK(ip, (yyvsp[-2].cblock));
						PUT_BLOCK(ip, (yyvsp[-1].cblock));
						PUT_OPCODE(ip, OP_EXIT);		/* must exit after return */

						/* copy debug info */
						ALLOC_DEBUG(psCurrBlock, (yyvsp[-2].cblock)->debugEntries);
						PUT_DEBUG(psCurrBlock, (yyvsp[-2].cblock));

						if (!scriptDefineEvent((yyvsp[-8].eSymbol), psCurrBlock, -1))
						{
							YYABORT;
						}

						FREE_DEBUG((yyvsp[-2].cblock));
						/* FREE_DEBUG($8); */
						FREE_BLOCK((yyvsp[-2].cblock));
						FREE_BLOCK((yyvsp[-1].cblock));

						/* end of event */
						psCurEvent = NULL;

						//debug(LOG_SCRIPT, "END func_decl 3. ");
					}
#line 4647 "script_parser.c"
    break;

  case 69:
#line 2761 "script_parser.y"
                                        {
						if(psCurEvent == NULL)	/* no events declared or defined yet */
						{
							scr_error("return statement outside of function");
							YYABORT;
						}

						if(!psCurEvent->bFunction)
						{

							scr_error("return statement inside of an event '%s'", psCurEvent->pIdent);
							YYABORT;
						}

						if(psCurEvent->retType != VAL_VOID)
						{

							scr_error("wrong return statement syntax for a non-void function '%s'", psCurEvent->pIdent);
							YYABORT;
						}

						/* Allocate code block for exit instruction */
						ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE));
						ip = psCurrBlock->pCode;
						PUT_OPCODE(ip, OP_EXIT);

						psCurrBlock->type = VAL_VOID;	/* make return statement of type VOID manually */

						(yyval.cblock) = psCurrBlock;
					}
#line 4682 "script_parser.c"
    break;

  case 70:
#line 2792 "script_parser.y"
                                        {
						//debug(LOG_SCRIPT, "RET  return_exp  ';'");

						if(psCurEvent == NULL)	/* no events declared or defined yet */
						{
							debug(LOG_ERROR, "return statement outside of function");
							YYABORT;
						}

						if(!psCurEvent->bFunction)
						{
							debug(LOG_ERROR, "return statement inside of an event '%s'", psCurEvent->pIdent);
							YYABORT;
						}

						if(psCurEvent->retType != (yyvsp[-1].cblock)->type)
						{
							if(!interpCheckEquiv(psCurEvent->retType, (yyvsp[-1].cblock)->type))
							{
								debug(LOG_ERROR, "return type mismatch");
								debug(LOG_ERROR, "wrong return statement syntax for function '%s' (%d - %d)", psCurEvent->pIdent, psCurEvent->retType, (yyvsp[-1].cblock)->type);
								YYABORT;
							}

						}

						/* Allocate code block for exit instruction */
						ALLOC_BLOCK(psCurrBlock, (yyvsp[-1].cblock)->size + sizeof(OPCODE));

						ip = psCurrBlock->pCode;

						PUT_BLOCK(ip, (yyvsp[-1].cblock));

						PUT_OPCODE(ip, OP_EXIT);

						/* store the type of the exp */
						psCurrBlock->type = (yyvsp[-1].cblock)->type;

						(yyval.cblock) = psCurrBlock;

						//debug(LOG_SCRIPT, "END RET  return_exp  ';'");
					}
#line 4729 "script_parser.c"
    break;

  case 71:
#line 2842 "script_parser.y"
                                                {
							// Allocate a dummy code block
							ALLOC_BLOCK(psCurrBlock, 1);
							psCurrBlock->size = 0;

							(yyval.cblock) = psCurrBlock;
						}
#line 4741 "script_parser.c"
    break;

  case 72:
#line 2850 "script_parser.y"
                                                {
							(yyval.cblock) = (yyvsp[0].cblock);
						}
#line 4749 "script_parser.c"
    break;

  case 73:
#line 2854 "script_parser.y"
                                                {
							ALLOC_BLOCK(psCurrBlock, (yyvsp[-1].cblock)->size + (yyvsp[0].cblock)->size);
							ALLOC_DEBUG(psCurrBlock, (yyvsp[-1].cblock)->debugEntries +
													 (yyvsp[0].cblock)->debugEntries);
							ip = psCurrBlock->pCode;

							/* Copy the two code blocks */
							PUT_BLOCK(ip, (yyvsp[-1].cblock));
							PUT_BLOCK(ip, (yyvsp[0].cblock));
							PUT_DEBUG(psCurrBlock, (yyvsp[-1].cblock));
							APPEND_DEBUG(psCurrBlock, (yyvsp[-1].cblock)->size / sizeof(UDWORD), (yyvsp[0].cblock));
							FREE_DEBUG((yyvsp[-1].cblock));
							FREE_DEBUG((yyvsp[0].cblock));
							FREE_BLOCK((yyvsp[-1].cblock));
							FREE_BLOCK((yyvsp[0].cblock));

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 4773 "script_parser.c"
    break;

  case 74:
#line 2878 "script_parser.y"
                                        {
						UDWORD line;
						STRING *pDummy;

						/* Put in debugging info */
						if (genDebugInfo)
						{
							ALLOC_DEBUG((yyvsp[-1].cblock), 1);
							(yyvsp[-1].cblock)->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							(yyvsp[-1].cblock)->psDebug[0].line = line;
						}

						(yyval.cblock) = (yyvsp[-1].cblock);
					}
#line 4793 "script_parser.c"
    break;

  case 75:
#line 2894 "script_parser.y"
                                        {
						UDWORD line;
						STRING *pDummy;



						/* Put in debugging info */
						if (genDebugInfo)
						{
							ALLOC_DEBUG((yyvsp[-1].cblock), 1);
							(yyvsp[-1].cblock)->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							(yyvsp[-1].cblock)->psDebug[0].line = line;
						}

						(yyval.cblock) = (yyvsp[-1].cblock);
					}
#line 4815 "script_parser.c"
    break;

  case 76:
#line 2915 "script_parser.y"
                                        {
						UDWORD line,paramNumber;
						STRING *pDummy;

						/* allow to call EVENTs to reuse the code only if no actual parameters are specified in function call, like "myEvent();" */
						if(!(yyvsp[-4].eSymbol)->bFunction && (yyvsp[-2].pblock)->numParams > 0)
						{
							scr_error("Can't pass any parameters in an event call:\nEvent: '%s'", (yyvsp[-4].eSymbol)->pIdent);
							return CE_PARSE;
						}

						if((yyvsp[-2].pblock)->numParams != (yyvsp[-4].eSymbol)->numParams)
						{
							scr_error("Wrong number of arguments for function call: '%s'. Expected %d parameters instead of  %d.", (yyvsp[-4].eSymbol)->pIdent, (yyvsp[-4].eSymbol)->numParams, (yyvsp[-2].pblock)->numParams);
							return CE_PARSE;
						}

						/* check if right parameters were passed */
						paramNumber = checkFuncParamTypes((yyvsp[-4].eSymbol), (yyvsp[-2].pblock));
						if(paramNumber > 0)
						{
							debug(LOG_ERROR, "Parameter mismatch in function call: '%s'. Mismatch in parameter  %d.", (yyvsp[-4].eSymbol)->pIdent, paramNumber);
							YYABORT;
						}

						/* INT functions can't be a statement */
						/* if($1->retType != VAL_VOID)
						{
							scr_error("Return type mismatch, missing assignment:\n'%s' is not a void function and returns a value", $1->pIdent);
							return CE_PARSE;
						} */

						/* Allocate the code block */
						ALLOC_BLOCK(psCurrBlock, (yyvsp[-2].pblock)->size + sizeof(OPCODE) + sizeof(UDWORD));	//Params + Opcode + event index
						ALLOC_DEBUG(psCurrBlock, 1);
						ip = psCurrBlock->pCode;

						if((yyvsp[-2].pblock)->numParams > 0)	/* if any parameters declared */
						{
							/* Copy in the code for the parameters */
							PUT_BLOCK(ip, (yyvsp[-2].pblock));		//PUT_BLOCK(ip, psPBlock);
							FREE_PBLOCK((yyvsp[-2].pblock));		//FREE_PBLOCK(psPBlock);
						}

						/* Store the instruction */
						PUT_OPCODE(ip, OP_FUNC);
						PUT_FUNC(ip,(yyvsp[-4].eSymbol)->index);			//Put event index

						/* Add the debugging information */
						if (genDebugInfo)
						{
							psCurrBlock->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							psCurrBlock->psDebug[0].line = line;
						}

						(yyval.cblock) = psCurrBlock;
					}
#line 4878 "script_parser.c"
    break;

  case 77:
#line 2974 "script_parser.y"
                                        {
						UDWORD line;
						STRING *pDummy;

						/* allow to call EVENTs to reuse the code only if no actual parameters are specified in function call, like "myEvent();" */
						if(!(yyvsp[-4].eSymbol)->bFunction && (yyvsp[-2].pblock)->numParams > 0)
						{
							scr_error("Can't pass any parameters in an event call:\nEvent: '%s'", (yyvsp[-4].eSymbol)->pIdent);
							return CE_PARSE;
						}

						if((yyvsp[-2].pblock)->numParams != (yyvsp[-4].eSymbol)->numParams)
						{
							scr_error("Wrong number of arguments for function call: '%s'. Expected %d parameters instead of  %d.", (yyvsp[-4].eSymbol)->pIdent, (yyvsp[-4].eSymbol)->numParams, (yyvsp[-2].pblock)->numParams);
							return CE_PARSE;
						}

						/* INT functions can't be a statement */
						/* if($1->retType != VAL_VOID)
						{
							scr_error("Return type mismatch, missing assignment:\n'%s' is not a void function and returns a value", $1->pIdent);
							return CE_PARSE;
						} */

						/* Allocate the code block */
						ALLOC_BLOCK(psCurrBlock, (yyvsp[-2].pblock)->size + sizeof(OPCODE) + sizeof(UDWORD));	//Params + Opcode + event index
						ALLOC_DEBUG(psCurrBlock, 1);
						ip = psCurrBlock->pCode;

						if((yyvsp[-2].pblock)->numParams > 0)	/* if any parameters declared */
						{
							/* Copy in the code for the parameters */
							PUT_BLOCK(ip, (yyvsp[-2].pblock));		//PUT_BLOCK(ip, psPBlock);
							FREE_PBLOCK((yyvsp[-2].pblock));		//FREE_PBLOCK(psPBlock);
						}

						/* Store the instruction */
						PUT_OPCODE(ip, OP_FUNC);
						PUT_FUNC(ip,(yyvsp[-4].eSymbol)->index);			//Put event index

						/* Add the debugging information */
						if (genDebugInfo)
						{
							psCurrBlock->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							psCurrBlock->psDebug[0].line = line;
						}

						(yyval.cblock) = psCurrBlock;
					}
#line 4933 "script_parser.c"
    break;

  case 78:
#line 3026 "script_parser.y"
                                        {
						(yyval.cblock) = (yyvsp[0].cblock);
					}
#line 4941 "script_parser.c"
    break;

  case 79:
#line 3030 "script_parser.y"
                                        {
						(yyval.cblock) = (yyvsp[0].cblock);
					}
#line 4949 "script_parser.c"
    break;

  case 80:
#line 3034 "script_parser.y"
                                        {
						UDWORD line;
						STRING *pDummy;

						/* Allocate the code block */
						ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE));
						ALLOC_DEBUG(psCurrBlock, 1);
						ip = psCurrBlock->pCode;

						/* Store the instruction */
						PUT_OPCODE(ip, OP_EXIT);

						/* Add the debugging information */
						if (genDebugInfo)
						{
							psCurrBlock->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							psCurrBlock->psDebug[0].line = line;
						}

						(yyval.cblock) = psCurrBlock;
					}
#line 4976 "script_parser.c"
    break;

  case 81:
#line 3058 "script_parser.y"
                                        {
						UDWORD line;
						STRING *pDummy;

						if(psCurEvent == NULL)
						{
							scr_error("'return' outside of function body");
							YYABORT;
						}

						if(!psCurEvent->bFunction)
						{
							debug(LOG_ERROR, "'return' can only be used in functions, not in events, event: '%s'", psCurEvent->pIdent);
							scr_error("'return' in event");
							YYABORT;
						}

						if((yyvsp[0].cblock)->type != psCurEvent->retType)
						{
							debug(LOG_ERROR, "'return' type doesn't match with function return type, function: '%s' (%d / %d)", psCurEvent->pIdent, (yyvsp[0].cblock)->type, psCurEvent->retType);
							scr_error("'return' type mismatch 0");
							YYABORT;
						}

						ALLOC_BLOCK(psCurrBlock, (yyvsp[0].cblock)->size + sizeof(OPCODE));
						ALLOC_DEBUG(psCurrBlock, 1);
						ip = psCurrBlock->pCode;

						PUT_BLOCK(ip, (yyvsp[0].cblock));
						PUT_OPCODE(ip, OP_EXIT);

						FREE_BLOCK((yyvsp[0].cblock));

						if (genDebugInfo)
						{
							psCurrBlock->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							psCurrBlock->psDebug[0].line = line;
						}

						(yyval.cblock) = psCurrBlock;
					}
#line 5023 "script_parser.c"
    break;

  case 82:
#line 3103 "script_parser.y"
                                        {
						UDWORD line;
						STRING *pDummy;

						// can only have a positive pause
						if ((yyvsp[-2].ival) < 0)
						{
							scr_error("Invalid pause time");
							YYABORT;
						}

						/* Allocate the code block */
						ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE));
						ALLOC_DEBUG(psCurrBlock, 1);
						ip = psCurrBlock->pCode;

						/* Store the instruction */
						PUT_PKOPCODE(ip, OP_PAUSE, (yyvsp[-2].ival));

						/* Add the debugging information */
						if (genDebugInfo)
						{
							psCurrBlock->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							psCurrBlock->psDebug[0].line = line;
						}

						(yyval.cblock) = psCurrBlock;
					}
#line 5057 "script_parser.c"
    break;

  case 83:
#line 3136 "script_parser.y"
                                {
					//debug(LOG_SCRIPT, "return_exp:		expression");
					/* Just pass the code up the tree */
					(yyvsp[0].cblock)->type = VAL_INT;
					(yyval.cblock) = (yyvsp[0].cblock);
				}
#line 5068 "script_parser.c"
    break;

  case 84:
#line 3143 "script_parser.y"
                                {
					//debug(LOG_SCRIPT, "return_exp:		stringexp");
					/* Just pass the code up the tree */
					(yyvsp[0].cblock)->type = VAL_STRING;
					(yyval.cblock) = (yyvsp[0].cblock);
				}
#line 5079 "script_parser.c"
    break;

  case 85:
#line 3150 "script_parser.y"
                                {
					//debug(LOG_SCRIPT, "return_exp:		boolexp");
					/* Just pass the code up the tree */
					(yyvsp[0].cblock)->type = VAL_BOOL;
					(yyval.cblock) = (yyvsp[0].cblock);
				}
#line 5090 "script_parser.c"
    break;

  case 86:
#line 3157 "script_parser.y"
                                {
					//debug(LOG_SCRIPT, "return_exp:		objexp");
					/* Just pass the code up the tree */
					/* $1->type =  */
					(yyval.cblock) = (yyvsp[0].cblock);
					//debug(LOG_SCRIPT, "END return_exp:		objexp");
				}
#line 5102 "script_parser.c"
    break;

  case 87:
#line 3184 "script_parser.y"
                                                {
							codeRet = scriptCodeAssignment((yyvsp[-2].vSymbol), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5114 "script_parser.c"
    break;

  case 88:
#line 3192 "script_parser.y"
                                                {
							codeRet = scriptCodeAssignment((yyvsp[-2].vSymbol), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5126 "script_parser.c"
    break;

  case 89:
#line 3200 "script_parser.y"
                                                {
							codeRet = scriptCodeAssignment((yyvsp[-2].vSymbol), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5138 "script_parser.c"
    break;

  case 90:
#line 3208 "script_parser.y"
                                                {
							if (!interpCheckEquiv((yyvsp[-2].vSymbol)->type, (yyvsp[0].cblock)->type))
							{
								scr_error("User type mismatch for assignment (%d - %d) 4", (yyvsp[-2].vSymbol)->type, (yyvsp[0].cblock)->type);
								YYABORT;
							}
							codeRet = scriptCodeAssignment((yyvsp[-2].vSymbol), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5155 "script_parser.c"
    break;

  case 91:
#line 3221 "script_parser.y"
                                                {
							if (!interpCheckEquiv((yyvsp[-2].vSymbol)->type, (yyvsp[0].cblock)->type))
							{
								scr_error("User type mismatch for assignment (%d - %d) 3", (yyvsp[-2].vSymbol)->type, (yyvsp[0].cblock)->type);
								YYABORT;
							}
							codeRet = scriptCodeAssignment((yyvsp[-2].vSymbol), (CODE_BLOCK *)(yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5172 "script_parser.c"
    break;

  case 92:
#line 3234 "script_parser.y"
                                                {
							codeRet = scriptCodeObjAssignment((yyvsp[-2].objVarBlock), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5184 "script_parser.c"
    break;

  case 93:
#line 3242 "script_parser.y"
                                                {
							codeRet = scriptCodeObjAssignment((yyvsp[-2].objVarBlock), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5196 "script_parser.c"
    break;

  case 94:
#line 3250 "script_parser.y"
                                                {
							if (!interpCheckEquiv((yyvsp[-2].objVarBlock)->psObjVar->type,(yyvsp[0].cblock)->type))
							{
								scr_error("User type mismatch for assignment (%d - %d) 2", (yyvsp[-2].objVarBlock)->psObjVar->type, (yyvsp[0].cblock)->type);
								YYABORT;
							}
							codeRet = scriptCodeObjAssignment((yyvsp[-2].objVarBlock), (CODE_BLOCK *)(yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5213 "script_parser.c"
    break;

  case 95:
#line 3263 "script_parser.y"
                                                {
							if (!interpCheckEquiv((yyvsp[-2].objVarBlock)->psObjVar->type, (yyvsp[0].cblock)->type))
							{
								scr_error("User type mismatch for assignment (%d - %d) 1", (yyvsp[-2].objVarBlock)->psObjVar->type, (yyvsp[0].cblock)->type);
								YYABORT;
							}
							codeRet = scriptCodeObjAssignment((yyvsp[-2].objVarBlock), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5230 "script_parser.c"
    break;

  case 96:
#line 3276 "script_parser.y"
                                                {
							codeRet = scriptCodeArrayAssignment((yyvsp[-2].arrayBlock), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5242 "script_parser.c"
    break;

  case 97:
#line 3284 "script_parser.y"
                                                {
							codeRet = scriptCodeArrayAssignment((yyvsp[-2].arrayBlock), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5254 "script_parser.c"
    break;

  case 98:
#line 3292 "script_parser.y"
                                                {
							if (!interpCheckEquiv((yyvsp[-2].arrayBlock)->psArrayVar->type,(yyvsp[0].cblock)->type))
							{
								scr_error("User type mismatch for assignment (%d - %d) 0", (yyvsp[-2].arrayBlock)->psArrayVar->type, (yyvsp[0].cblock)->type);
								YYABORT;
							}
							codeRet = scriptCodeArrayAssignment((yyvsp[-2].arrayBlock), (CODE_BLOCK *)(yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5271 "script_parser.c"
    break;

  case 99:
#line 3305 "script_parser.y"
                                                {
							if (!interpCheckEquiv((yyvsp[-2].arrayBlock)->psArrayVar->type, (yyvsp[0].cblock)->type))
							{
								scr_error("User type mismatch for assignment");
								YYABORT;
							}
							codeRet = scriptCodeArrayAssignment((yyvsp[-2].arrayBlock), (yyvsp[0].cblock), &psCurrBlock);
							CHECK_CODE_ERROR(codeRet);

							/* Return the code block */
							(yyval.cblock) = psCurrBlock;
						}
#line 5288 "script_parser.c"
    break;

  case 100:
#line 3326 "script_parser.y"
                                        {


						/* Generate the code for the function call */
						codeRet = scriptCodeFunction((yyvsp[-3].fSymbol), (yyvsp[-1].pblock), FALSE, &psCurrBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.cblock) = psCurrBlock;
					}
#line 5303 "script_parser.c"
    break;

  case 101:
#line 3337 "script_parser.y"
                                        {


						/* Generate the code for the function call */
						codeRet = scriptCodeFunction((yyvsp[-3].fSymbol), (yyvsp[-1].pblock), FALSE, &psCurrBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.cblock) = psCurrBlock;
					}
#line 5318 "script_parser.c"
    break;

  case 102:
#line 3348 "script_parser.y"
                                        {


						/* Generate the code for the function call */
						codeRet = scriptCodeFunction((yyvsp[-3].fSymbol), (yyvsp[-1].pblock), FALSE, &psCurrBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.cblock) = psCurrBlock;
					}
#line 5333 "script_parser.c"
    break;

  case 103:
#line 3359 "script_parser.y"
                                        {


						/* Generate the code for the function call */
						codeRet = scriptCodeFunction((yyvsp[-3].fSymbol), (yyvsp[-1].pblock), FALSE, &psCurrBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.cblock) = psCurrBlock;
					}
#line 5348 "script_parser.c"
    break;

  case 104:
#line 3370 "script_parser.y"
                                        {

						/* Generate the code for the function call */
						codeRet = scriptCodeFunction((yyvsp[-3].fSymbol), (yyvsp[-1].pblock), FALSE, &psCurrBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.cblock) = psCurrBlock;
					}
#line 5362 "script_parser.c"
    break;

  case 105:
#line 3391 "script_parser.y"
                                        {
						/* create a dummy pblock containing nothing */
						ALLOC_PBLOCK(psCurrPBlock, sizeof(UDWORD), 1);
						psCurrPBlock->size = 0;
						psCurrPBlock->numParams = 0;

						(yyval.pblock) = psCurrPBlock;
					}
#line 5375 "script_parser.c"
    break;

  case 106:
#line 3400 "script_parser.y"
                                        {
						//debug(LOG_SCRIPT, "param_list: parameter");
						//debug(LOG_SCRIPT, "param_list:param 0 type: %d", $1->aParams[0]);
						(yyval.pblock) = (yyvsp[0].pblock);
					}
#line 5385 "script_parser.c"
    break;

  case 107:
#line 3406 "script_parser.y"
                                        {


						ALLOC_PBLOCK(psCurrPBlock, (yyvsp[-2].pblock)->size + (yyvsp[0].pblock)->size,
												   (yyvsp[-2].pblock)->numParams + (yyvsp[0].pblock)->numParams);
						ip = psCurrPBlock->pCode;

						/* Copy in the code for the parameters */
						PUT_BLOCK(ip, (yyvsp[-2].pblock));
						PUT_BLOCK(ip, (yyvsp[0].pblock));

						/* Copy the parameter types */
						memcpy(psCurrPBlock->aParams, (yyvsp[-2].pblock)->aParams,
								(yyvsp[-2].pblock)->numParams * sizeof(INTERP_TYPE));
						memcpy(psCurrPBlock->aParams + (yyvsp[-2].pblock)->numParams,
								(yyvsp[0].pblock)->aParams,
								(yyvsp[0].pblock)->numParams * sizeof(INTERP_TYPE));

						/* Free the old pblocks */
						FREE_PBLOCK((yyvsp[-2].pblock));
						FREE_PBLOCK((yyvsp[0].pblock));

						//if(psCurrPBlock->numParams > 0)
						//	debug(LOG_SCRIPT, "param_list:param %d type: %d", psCurrPBlock->numParams, $3->aParams[0]);

						/* return the pblock */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5418 "script_parser.c"
    break;

  case 108:
#line 3436 "script_parser.y"
                                        {
						/* Generate the code for the parameter                     */
						codeRet = scriptCodeParameter((yyvsp[0].cblock), VAL_INT, &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5431 "script_parser.c"
    break;

  case 109:
#line 3445 "script_parser.y"
                                        {
						/* Generate the code for the parameter */
						codeRet = scriptCodeParameter((yyvsp[0].cblock), VAL_BOOL, &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5444 "script_parser.c"
    break;

  case 110:
#line 3454 "script_parser.y"
                                        {
						/* Generate the code for the parameter */
						codeRet = scriptCodeParameter((yyvsp[0].cblock), VAL_STRING, &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5457 "script_parser.c"
    break;

  case 111:
#line 3463 "script_parser.y"
                                        {
						/* Generate the code for the parameter */
						codeRet = scriptCodeParameter((CODE_BLOCK *)(yyvsp[0].cblock), (yyvsp[0].cblock)->type, &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5470 "script_parser.c"
    break;

  case 112:
#line 3472 "script_parser.y"
                                        {
						//debug(LOG_SCRIPT, "objexp, scriptCodeParameter");

						/* Generate the code for the parameter */
						codeRet = scriptCodeParameter((yyvsp[0].cblock), (yyvsp[0].cblock)->type, &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5485 "script_parser.c"
    break;

  case 113:
#line 3483 "script_parser.y"
                                        {
						/* just pass the variable reference up the tree */
						(yyval.pblock) = (yyvsp[0].pblock);
					}
#line 5494 "script_parser.c"
    break;

  case 114:
#line 3490 "script_parser.y"
                                        {
						codeRet = scriptCodeVarRef((yyvsp[0].vSymbol), &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5506 "script_parser.c"
    break;

  case 115:
#line 3498 "script_parser.y"
                                        {
						codeRet = scriptCodeVarRef((yyvsp[0].vSymbol), &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5518 "script_parser.c"
    break;

  case 116:
#line 3506 "script_parser.y"
                                        {
						codeRet = scriptCodeVarRef((yyvsp[0].vSymbol), &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5530 "script_parser.c"
    break;

  case 117:
#line 3514 "script_parser.y"
                                        {
						codeRet = scriptCodeVarRef((yyvsp[0].vSymbol), &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5542 "script_parser.c"
    break;

  case 118:
#line 3522 "script_parser.y"
                                        {
						codeRet = scriptCodeVarRef((yyvsp[0].vSymbol), &psCurrPBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.pblock) = psCurrPBlock;
					}
#line 5554 "script_parser.c"
    break;

  case 119:
#line 3537 "script_parser.y"
                                        {
						codeRet = scriptCodeConditional((yyvsp[0].condBlock), &psCurrBlock);
						CHECK_CODE_ERROR(codeRet);

						(yyval.cblock) = psCurrBlock;
					}
#line 5565 "script_parser.c"
    break;

  case 120:
#line 3544 "script_parser.y"
                                        {
						ALLOC_CONDBLOCK(psCondBlock,
										(yyvsp[-2].condBlock)->numOffsets + (yyvsp[0].condBlock)->numOffsets,
										(yyvsp[-2].condBlock)->size + (yyvsp[0].condBlock)->size);
						ALLOC_DEBUG(psCondBlock, (yyvsp[-2].condBlock)->debugEntries + (yyvsp[0].condBlock)->debugEntries);
						ip = psCondBlock->pCode;

						/* Store the two blocks of code */
						PUT_BLOCK(ip, (yyvsp[-2].condBlock));
						PUT_BLOCK(ip, (yyvsp[0].condBlock));

						/* Copy over the offset information       */
						/* (There isn't any in the terminal_cond) */
						memcpy(psCondBlock->aOffsets, (yyvsp[-2].condBlock)->aOffsets,
							   (yyvsp[-2].condBlock)->numOffsets * sizeof(SDWORD));
						psCondBlock->numOffsets = (yyvsp[-2].condBlock)->numOffsets;

						/* Put in the debugging information */
						PUT_DEBUG(psCondBlock, (yyvsp[-2].condBlock));
						APPEND_DEBUG(psCondBlock, (yyvsp[-2].condBlock)->size / sizeof(UDWORD), (yyvsp[0].condBlock));

						/* Free the code blocks */
						FREE_DEBUG((yyvsp[-2].condBlock));
						FREE_DEBUG((yyvsp[0].condBlock));
						FREE_CONDBLOCK((yyvsp[-2].condBlock));
						FREE_CONDBLOCK((yyvsp[0].condBlock));

						/* Do the final processing of the conditional */
						codeRet = scriptCodeConditional(psCondBlock, &psCurrBlock);
						CHECK_CODE_ERROR(codeRet);

						(yyval.cblock) = psCurrBlock;
					}
#line 5603 "script_parser.c"
    break;

  case 121:
#line 3580 "script_parser.y"
                                        {
						(yyval.condBlock) = (yyvsp[0].condBlock);
					}
#line 5611 "script_parser.c"
    break;

  case 122:
#line 3584 "script_parser.y"
                                        {
						ALLOC_CONDBLOCK(psCondBlock,
										(yyvsp[-2].condBlock)->numOffsets + (yyvsp[0].condBlock)->numOffsets,
										(yyvsp[-2].condBlock)->size + (yyvsp[0].condBlock)->size);
						ALLOC_DEBUG(psCondBlock, (yyvsp[-2].condBlock)->debugEntries + (yyvsp[0].condBlock)->debugEntries);
						ip = psCondBlock->pCode;

						/* Store the two blocks of code */
						PUT_BLOCK(ip, (yyvsp[-2].condBlock));
						PUT_BLOCK(ip, (yyvsp[0].condBlock));

						/* Copy over the offset information */
						memcpy(psCondBlock->aOffsets, (yyvsp[-2].condBlock)->aOffsets,
							   (yyvsp[-2].condBlock)->numOffsets * sizeof(SDWORD));
						psCondBlock->aOffsets[(yyvsp[-2].condBlock)->numOffsets] =
							(yyvsp[0].condBlock)->aOffsets[0] + (yyvsp[-2].condBlock)->size / sizeof(UDWORD);
						psCondBlock->numOffsets = (yyvsp[-2].condBlock)->numOffsets + 1;

						/* Put in the debugging information */
						PUT_DEBUG(psCondBlock, (yyvsp[-2].condBlock));
						APPEND_DEBUG(psCondBlock, (yyvsp[-2].condBlock)->size / sizeof(UDWORD), (yyvsp[0].condBlock));

						/* Free the code blocks */
						FREE_DEBUG((yyvsp[-2].condBlock));
						FREE_DEBUG((yyvsp[0].condBlock));
						FREE_CONDBLOCK((yyvsp[-2].condBlock));
						FREE_CONDBLOCK((yyvsp[0].condBlock));

						(yyval.condBlock) = psCondBlock;
					}
#line 5646 "script_parser.c"
    break;

  case 123:
#line 3618 "script_parser.y"
                                        {
						/* Allocate the block */
						ALLOC_CONDBLOCK(psCondBlock, 1, (yyvsp[-1].cblock)->size);
						ALLOC_DEBUG(psCondBlock, (yyvsp[-1].cblock)->debugEntries);
						ip = psCondBlock->pCode;

						/* Put in the debugging information */
						PUT_DEBUG(psCondBlock, (yyvsp[-1].cblock));

						/* Store the statements */
						PUT_BLOCK(ip, (yyvsp[-1].cblock));
						FREE_DEBUG((yyvsp[-1].cblock));
						FREE_BLOCK((yyvsp[-1].cblock));

						(yyval.condBlock) = psCondBlock;
					}
#line 5667 "script_parser.c"
    break;

  case 124:
#line 3637 "script_parser.y"
                                        {
						STRING *pDummy;

						/* Get the line number for the end of the boolean expression */
						/* and store it in debugLine.                                 */
						scriptGetErrorData((SDWORD *)&debugLine, &pDummy);
					}
#line 5679 "script_parser.c"
    break;

  case 125:
#line 3645 "script_parser.y"
                                        {
						/* Allocate the block */
						ALLOC_CONDBLOCK(psCondBlock, 1,
										(yyvsp[-5].cblock)->size + (yyvsp[-1].cblock)->size +
										sizeof(OPCODE)*2);
						ALLOC_DEBUG(psCondBlock, (yyvsp[-1].cblock)->debugEntries + 1);
						ip = psCondBlock->pCode;

						/* Store the boolean expression code */
						PUT_BLOCK(ip, (yyvsp[-5].cblock));
						FREE_BLOCK((yyvsp[-5].cblock));

						/* Put in the jump to the end of the block if the */
						/* condition is false */
						PUT_PKOPCODE(ip, OP_JUMPFALSE, ((yyvsp[-1].cblock)->size / sizeof(UDWORD)) + 2);

						/* Put in the debugging information */
						if (genDebugInfo)
						{
							psCondBlock->debugEntries = 1;
							psCondBlock->psDebug->line = debugLine;
							psCondBlock->psDebug->offset = 0;
							APPEND_DEBUG(psCondBlock, ip - psCondBlock->pCode, (yyvsp[-1].cblock));
						}

						/* Store the statements */
						PUT_BLOCK(ip, (yyvsp[-1].cblock));
						FREE_DEBUG((yyvsp[-1].cblock));
						FREE_BLOCK((yyvsp[-1].cblock));

						/* Store the location that has to be filled in   */
						psCondBlock->aOffsets[0] = ip - psCondBlock->pCode;

						/* Put in a jump to skip the rest of the conditional */
						/* The correct offset will be set once the whole   */
						/* conditional has been parsed                     */
						/* The jump should be to the instruction after the */
						/* entire conditonal block                         */
						PUT_PKOPCODE(ip, OP_JUMP, 0);

						(yyval.condBlock) = psCondBlock;
					}
#line 5726 "script_parser.c"
    break;

  case 126:
#line 3701 "script_parser.y"
                                {
					STRING *pDummy;

					/* Get the line number for the end of the boolean expression */
					/* and store it in debugLine.                                 */
					scriptGetErrorData((SDWORD *)&debugLine, &pDummy);
				}
#line 5738 "script_parser.c"
    break;

  case 127:
#line 3709 "script_parser.y"
                                {

					ALLOC_BLOCK(psCurrBlock, (yyvsp[-5].cblock)->size + (yyvsp[-1].cblock)->size + sizeof(OPCODE) * 2);
					ALLOC_DEBUG(psCurrBlock, (yyvsp[-1].cblock)->debugEntries + 1);
					ip = psCurrBlock->pCode;

					/* Copy in the loop expression */
					PUT_BLOCK(ip, (yyvsp[-5].cblock));
					FREE_BLOCK((yyvsp[-5].cblock));

					/* Now a conditional jump out of the loop if the */
					/* expression is false.                          */
					PUT_PKOPCODE(ip, OP_JUMPFALSE, ((yyvsp[-1].cblock)->size / sizeof(UDWORD)) + 2);

					/* Now put in the debugging information */
					if (genDebugInfo)
					{
						psCurrBlock->debugEntries = 1;
						psCurrBlock->psDebug->line = debugLine;
						psCurrBlock->psDebug->offset = 0;
						APPEND_DEBUG(psCurrBlock, ip - psCurrBlock->pCode, (yyvsp[-1].cblock));
					}

					/* Copy in the body of the loop */
					PUT_BLOCK(ip, (yyvsp[-1].cblock));
					FREE_DEBUG((yyvsp[-1].cblock));
					FREE_BLOCK((yyvsp[-1].cblock));

					/* Put in a jump back to the start of the loop expression */
					PUT_PKOPCODE(ip, OP_JUMP, (SWORD)( -(SWORD)(psCurrBlock->size / sizeof(UDWORD)) + 1));

					(yyval.cblock) = psCurrBlock;
				}
#line 5776 "script_parser.c"
    break;

  case 128:
#line 3751 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_ADD, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5788 "script_parser.c"
    break;

  case 129:
#line 3759 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_SUB, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5800 "script_parser.c"
    break;

  case 130:
#line 3767 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_MUL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5812 "script_parser.c"
    break;

  case 131:
#line 3775 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_DIV, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5824 "script_parser.c"
    break;

  case 132:
#line 3784 "script_parser.y"
                                {
					ALLOC_BLOCK(psCurrBlock, (yyvsp[0].cblock)->size + sizeof(OPCODE));
					ip = psCurrBlock->pCode;

					/* Copy the already generated bits of code into the code block */
					PUT_BLOCK(ip, (yyvsp[0].cblock));

					/* Now put a negation operator into the code */
					PUT_PKOPCODE(ip, OP_UNARYOP, OP_NEG);

					/* Free the two code blocks that have been copied */
					FREE_BLOCK((yyvsp[0].cblock));

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5845 "script_parser.c"
    break;

  case 133:
#line 3801 "script_parser.y"
                                {
					/* Just pass the code up the tree */
					(yyval.cblock) = (yyvsp[-1].cblock);
				}
#line 5854 "script_parser.c"
    break;

  case 134:
#line 3806 "script_parser.y"
                                {
					/* Generate the code for the function call */
					codeRet = scriptCodeFunction((yyvsp[-3].fSymbol), (yyvsp[-1].pblock), TRUE, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5867 "script_parser.c"
    break;

  case 135:
#line 3816 "script_parser.y"
                                {
						UDWORD line,paramNumber;
						STRING *pDummy;

						/* if($4->numParams != $3->numParams) */
						if((yyvsp[-1].pblock)->numParams != (yyvsp[-3].eSymbol)->numParams)
						{
							debug(LOG_ERROR, "Wrong number of arguments for function call: '%s'. Expected %d parameters instead of  %d.", (yyvsp[-3].eSymbol)->pIdent, (yyvsp[-3].eSymbol)->numParams, (yyvsp[-1].pblock)->numParams);
							scr_error("Wrong number of arguments in function call");
							return CE_PARSE;
						}

						if(!(yyvsp[-3].eSymbol)->bFunction)
						{
							debug(LOG_ERROR, "'%s' is not a function", (yyvsp[-3].eSymbol)->pIdent);
							scr_error("Can't cann an event");
							return CE_PARSE;
						}

						/* make sure function has a return type */
						if((yyvsp[-3].eSymbol)->retType != VAL_INT)
						{
							debug(LOG_ERROR, "'%s' does not return an integer value", (yyvsp[-3].eSymbol)->pIdent);
							scr_error("assignment type conflict");
							return CE_PARSE;
						}

						/* check if right parameters were passed */
						paramNumber = checkFuncParamTypes((yyvsp[-3].eSymbol), (yyvsp[-1].pblock));
						if(paramNumber > 0)
						{
							debug(LOG_ERROR, "Parameter mismatch in function call: '%s'. Mismatch in parameter  %d.", (yyvsp[-3].eSymbol)->pIdent, paramNumber);
							YYABORT;
						}

						/* Allocate the code block */
						ALLOC_BLOCK(psCurrBlock, (yyvsp[-1].pblock)->size + sizeof(OPCODE) + sizeof(UDWORD));	//Params + Opcode + event index
						ALLOC_DEBUG(psCurrBlock, 1);
						ip = psCurrBlock->pCode;

						if((yyvsp[-1].pblock)->numParams > 0)	/* if any parameters declared */
						{
							/* Copy in the code for the parameters */
							PUT_BLOCK(ip, (yyvsp[-1].pblock));		//PUT_BLOCK(ip, psPBlock);
							FREE_PBLOCK((yyvsp[-1].pblock));		//FREE_PBLOCK(psPBlock);
						}

						/* Store the instruction */
						PUT_OPCODE(ip, OP_FUNC);
						PUT_FUNC(ip,(yyvsp[-3].eSymbol)->index);			//Put event index

						/* Add the debugging information */
						if (genDebugInfo)
						{
							psCurrBlock->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							psCurrBlock->psDebug[0].line = line;
						}

						(yyval.cblock) = psCurrBlock;
				}
#line 5933 "script_parser.c"
    break;

  case 136:
#line 3878 "script_parser.y"
                                {
					codeRet = scriptCodeVarGet((yyvsp[0].vSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5945 "script_parser.c"
    break;

  case 137:
#line 3886 "script_parser.y"
                                {
					codeRet = scriptCodeConstant((yyvsp[0].cSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5957 "script_parser.c"
    break;

  case 138:
#line 3894 "script_parser.y"
                                {
					codeRet = scriptCodeObjGet((yyvsp[0].objVarBlock), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5969 "script_parser.c"
    break;

  case 139:
#line 3902 "script_parser.y"
                                {
					codeRet = scriptCodeArrayGet((yyvsp[0].arrayBlock), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5981 "script_parser.c"
    break;

  case 140:
#line 3910 "script_parser.y"
                                {
					ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE) + sizeof(UDWORD));
					ip = psCurrBlock->pCode;

					/* Code to store the value on the stack */
					PUT_PKOPCODE(ip, OP_PUSH, VAL_INT);
					PUT_DATA(ip, (yyvsp[0].ival));

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 5997 "script_parser.c"
    break;

  case 141:
#line 3930 "script_parser.y"
                                {
					//debug(LOG_SCRIPT, "stringexp: stringexp '&' stringexp");

					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_CANC, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;

					//debug(LOG_SCRIPT, "END stringexp: stringexp '&' stringexp");
				}
#line 6013 "script_parser.c"
    break;

  case 142:
#line 3942 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_CANC, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6025 "script_parser.c"
    break;

  case 143:
#line 3950 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_CANC, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6037 "script_parser.c"
    break;

  case 144:
#line 3959 "script_parser.y"
                                {
					//debug(LOG_SCRIPT, "'(' stringexp ')'");
					/* Just pass the code up the tree */
					(yyval.cblock) = (yyvsp[-1].cblock);
					//debug(LOG_SCRIPT, "END '(' stringexp ')'");
				}
#line 6048 "script_parser.c"
    break;

  case 145:
#line 3967 "script_parser.y"
                                {
						UDWORD line;
						STRING *pDummy;

						if((yyvsp[-1].pblock)->numParams != (yyvsp[-3].eSymbol)->numParams)
						{
							debug(LOG_ERROR, "Wrong number of arguments for function call: '%s'. Expected %d parameters instead of  %d.", (yyvsp[-3].eSymbol)->pIdent, (yyvsp[-3].eSymbol)->numParams, (yyvsp[-1].pblock)->numParams);
							scr_error("Wrong number of arguments in function call");
							return CE_PARSE;
						}

						if(!(yyvsp[-3].eSymbol)->bFunction)
						{
							debug(LOG_ERROR, "'%s' is not a function", (yyvsp[-3].eSymbol)->pIdent);
							scr_error("Can't cann an event");
							return CE_PARSE;
						}

						/* make sure function has a return type */
						if((yyvsp[-3].eSymbol)->retType != VAL_STRING)
						{
							debug(LOG_ERROR, "'%s' does not return a string value", (yyvsp[-3].eSymbol)->pIdent);
							scr_error("assignment type conflict");
							return CE_PARSE;
						}

						/* Allocate the code block */
						ALLOC_BLOCK(psCurrBlock, (yyvsp[-1].pblock)->size + sizeof(OPCODE) + sizeof(UDWORD));	//Params + Opcode + event index
						ALLOC_DEBUG(psCurrBlock, 1);
						ip = psCurrBlock->pCode;

						if((yyvsp[-1].pblock)->numParams > 0)	/* if any parameters declared */
						{
							/* Copy in the code for the parameters */
							PUT_BLOCK(ip, (yyvsp[-1].pblock));		//PUT_BLOCK(ip, psPBlock);
							FREE_PBLOCK((yyvsp[-1].pblock));		//FREE_PBLOCK(psPBlock);
						}

						/* Store the instruction */
						PUT_OPCODE(ip, OP_FUNC);
						PUT_FUNC(ip,(yyvsp[-3].eSymbol)->index);			//Put event index

						/* Add the debugging information */
						if (genDebugInfo)
						{
							psCurrBlock->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							psCurrBlock->psDebug[0].line = line;
						}

						(yyval.cblock) = psCurrBlock;
				}
#line 6105 "script_parser.c"
    break;

  case 146:
#line 4021 "script_parser.y"
                                {

					//debug(LOG_SCRIPT, "STRING_VAR");

					codeRet = scriptCodeVarGet((yyvsp[0].vSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;

					//debug(LOG_SCRIPT, "END STRING_VAR (%s)", $1->pIdent);
				}
#line 6122 "script_parser.c"
    break;

  case 147:
#line 4034 "script_parser.y"
                                {
					codeRet = scriptCodeVarGet((yyvsp[0].vSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6134 "script_parser.c"
    break;

  case 148:
#line 4042 "script_parser.y"
                                {
					//debug(LOG_SCRIPT, "QTEXT found (%s)", yyvsp[0].sval);

					ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE) + sizeof(UDWORD));
					ip = psCurrBlock->pCode;

					/* Code to store the value on the stack */
					PUT_PKOPCODE(ip, OP_PUSH, VAL_STRING);
					PUT_DATA(ip, STRSTACK[CURSTACKSTR]);		/* was $1 */

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;

					/* DEBUG */
					//sprintf(msg,"[YACC]: PUT_DATA VAL_STRING, val: '%s'\n",yyvsp[0].sval);
					//debug(LOG_SCRIPT,"[YACC]: PUT_DATA VAL_STRING, val: '%s'",yyvsp[0].sval);

					/* Manage string stack */
					*STRSTACK[CURSTACKSTR] = (char)MALLOC(MAXSTRLEN);
					widgCopyString(STRSTACK[CURSTACKSTR],yyvsp[0].sval);
					CURSTACKSTR = CURSTACKSTR + 1;		/* Increment 'pointer' to the top of the string stack */

					//debug(LOG_SCRIPT, "END QTEXT found");
				}
#line 6163 "script_parser.c"
    break;

  case 149:
#line 4067 "script_parser.y"
                                {
					ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE) + sizeof(UDWORD));
					ip = psCurrBlock->pCode;

					/* Code to store the value on the stack */
					PUT_PKOPCODE(ip, OP_PUSH, VAL_INT);
					PUT_DATA(ip, (yyvsp[0].ival));

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6179 "script_parser.c"
    break;

  case 150:
#line 4087 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_AND, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6191 "script_parser.c"
    break;

  case 151:
#line 4095 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_OR, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6203 "script_parser.c"
    break;

  case 152:
#line 4103 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_EQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6215 "script_parser.c"
    break;

  case 153:
#line 4111 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_NOTEQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6227 "script_parser.c"
    break;

  case 154:
#line 4119 "script_parser.y"
                                {
					ALLOC_BLOCK(psCurrBlock, (yyvsp[0].cblock)->size + sizeof(OPCODE));
					ip = psCurrBlock->pCode;

					/* Copy the already generated bits of code into the code block */
					PUT_BLOCK(ip, (yyvsp[0].cblock));

					/* Now put a negation operator into the code */
					PUT_PKOPCODE(ip, OP_UNARYOP, OP_NOT);

					/* Free the two code blocks that have been copied */
					FREE_BLOCK((yyvsp[0].cblock));

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6248 "script_parser.c"
    break;

  case 155:
#line 4136 "script_parser.y"
                                {
					/* Just pass the code up the tree */
					(yyval.cblock) = (yyvsp[-1].cblock);
				}
#line 6257 "script_parser.c"
    break;

  case 156:
#line 4141 "script_parser.y"
                                {
					/* Generate the code for the function call */
					codeRet = scriptCodeFunction((yyvsp[-3].fSymbol), (yyvsp[-1].pblock), TRUE, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6270 "script_parser.c"
    break;

  case 157:
#line 4150 "script_parser.y"
                                {
						UDWORD line,paramNumber;
						STRING *pDummy;

						if((yyvsp[-1].pblock)->numParams != (yyvsp[-3].eSymbol)->numParams)
						{
							debug(LOG_ERROR, "Wrong number of arguments for function call: '%s'. Expected %d parameters instead of  %d.", (yyvsp[-3].eSymbol)->pIdent, (yyvsp[-3].eSymbol)->numParams, (yyvsp[-1].pblock)->numParams);
							scr_error("Wrong number of arguments in function call");
							return CE_PARSE;
						}

						if(!(yyvsp[-3].eSymbol)->bFunction)
						{
							debug(LOG_ERROR, "'%s' is not a function", (yyvsp[-3].eSymbol)->pIdent);
							scr_error("Can't cann an event");
							return CE_PARSE;
						}

						/* make sure function has a return type */
						if((yyvsp[-3].eSymbol)->retType != VAL_BOOL)
						{
							debug(LOG_ERROR, "'%s' does not return a boolean value", (yyvsp[-3].eSymbol)->pIdent);
							scr_error("assignment type conflict");
							return CE_PARSE;
						}

						/* check if right parameters were passed */
						paramNumber = checkFuncParamTypes((yyvsp[-3].eSymbol), (yyvsp[-1].pblock));
						if(paramNumber > 0)
						{
							debug(LOG_ERROR, "Parameter mismatch in function call: '%s'. Mismatch in parameter  %d.", (yyvsp[-3].eSymbol)->pIdent, paramNumber);
							YYABORT;
						}

						/* Allocate the code block */
						ALLOC_BLOCK(psCurrBlock, (yyvsp[-1].pblock)->size + sizeof(OPCODE) + sizeof(UDWORD));	//Params + Opcode + event index
						ALLOC_DEBUG(psCurrBlock, 1);
						ip = psCurrBlock->pCode;

						if((yyvsp[-1].pblock)->numParams > 0)	/* if any parameters declared */
						{
							/* Copy in the code for the parameters */
							PUT_BLOCK(ip, (yyvsp[-1].pblock));		//PUT_BLOCK(ip, psPBlock);
							FREE_PBLOCK((yyvsp[-1].pblock));		//FREE_PBLOCK(psPBlock);
						}

						/* Store the instruction */
						PUT_OPCODE(ip, OP_FUNC);
						PUT_FUNC(ip,(yyvsp[-3].eSymbol)->index);			//Put event index

						/* Add the debugging information */
						if (genDebugInfo)
						{
							psCurrBlock->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							psCurrBlock->psDebug[0].line = line;
						}

						(yyval.cblock) = psCurrBlock;
				}
#line 6335 "script_parser.c"
    break;

  case 158:
#line 4211 "script_parser.y"
                                {
					codeRet = scriptCodeVarGet((yyvsp[0].vSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6347 "script_parser.c"
    break;

  case 159:
#line 4219 "script_parser.y"
                                {
					codeRet = scriptCodeConstant((yyvsp[0].cSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6359 "script_parser.c"
    break;

  case 160:
#line 4227 "script_parser.y"
                                {
					codeRet = scriptCodeObjGet((yyvsp[0].objVarBlock), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6371 "script_parser.c"
    break;

  case 161:
#line 4235 "script_parser.y"
                                {
					codeRet = scriptCodeArrayGet((yyvsp[0].arrayBlock), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6383 "script_parser.c"
    break;

  case 162:
#line 4243 "script_parser.y"
                                {
					ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE) + sizeof(UDWORD));
					ip = psCurrBlock->pCode;

					/* Code to store the value on the stack */
					PUT_PKOPCODE(ip, OP_PUSH, VAL_BOOL);
					PUT_DATA(ip, (yyvsp[0].bval));

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6399 "script_parser.c"
    break;

  case 163:
#line 4255 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_EQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6411 "script_parser.c"
    break;

  case 164:
#line 4263 "script_parser.y"
                                {
					if (!interpCheckEquiv((yyvsp[-2].cblock)->type,(yyvsp[0].cblock)->type))
					{
						scr_error("Type mismatch for equality");
						YYABORT;
					}
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_EQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6428 "script_parser.c"
    break;

  case 165:
#line 4276 "script_parser.y"
                                {
					if (!interpCheckEquiv((yyvsp[-2].cblock)->type,(yyvsp[0].cblock)->type))
					{
						scr_error("Type mismatch for equality");
						YYABORT;
					}
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_EQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6445 "script_parser.c"
    break;

  case 166:
#line 4289 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_NOTEQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6457 "script_parser.c"
    break;

  case 167:
#line 4297 "script_parser.y"
                                {
					if (!interpCheckEquiv((yyvsp[-2].cblock)->type,(yyvsp[0].cblock)->type))
					{
						scr_error("Type mismatch for inequality");
						YYABORT;
					}
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_NOTEQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6474 "script_parser.c"
    break;

  case 168:
#line 4310 "script_parser.y"
                                {
					if (!interpCheckEquiv((yyvsp[-2].cblock)->type,(yyvsp[0].cblock)->type))
					{
						scr_error("Type mismatch for inequality");
						YYABORT;
					}
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_NOTEQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6491 "script_parser.c"
    break;

  case 169:
#line 4323 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_LESSEQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6503 "script_parser.c"
    break;

  case 170:
#line 4331 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_GREATEREQUAL, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6515 "script_parser.c"
    break;

  case 171:
#line 4339 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_GREATER, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6527 "script_parser.c"
    break;

  case 172:
#line 4347 "script_parser.y"
                                {
					codeRet = scriptCodeBinaryOperator((yyvsp[-2].cblock), (yyvsp[0].cblock), OP_LESS, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6539 "script_parser.c"
    break;

  case 173:
#line 4362 "script_parser.y"
                                {
					codeRet = scriptCodeVarGet((yyvsp[0].vSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6551 "script_parser.c"
    break;

  case 174:
#line 4370 "script_parser.y"
                                {
					codeRet = scriptCodeConstant((yyvsp[0].cSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6563 "script_parser.c"
    break;

  case 175:
#line 4378 "script_parser.y"
                                {
					codeRet = scriptCodeObjGet((yyvsp[0].objVarBlock), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);
					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6574 "script_parser.c"
    break;

  case 176:
#line 4385 "script_parser.y"
                                {
					codeRet = scriptCodeArrayGet((yyvsp[0].arrayBlock), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6586 "script_parser.c"
    break;

  case 177:
#line 4393 "script_parser.y"
                                {
					/* Generate the code for the function call */
					codeRet = scriptCodeFunction((yyvsp[-3].fSymbol), (yyvsp[-1].pblock), TRUE, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6599 "script_parser.c"
    break;

  case 178:
#line 4402 "script_parser.y"
                                {
					ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE) + sizeof(UDWORD));
					ip = psCurrBlock->pCode;

					/* Code to store the value on the stack */
					PUT_PKOPCODE(ip, OP_PUSH, VAL_TRIGGER);
					PUT_DATA(ip, (yyvsp[0].tSymbol)->index);

					psCurrBlock->type = VAL_TRIGGER;

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6617 "script_parser.c"
    break;

  case 179:
#line 4416 "script_parser.y"
                                {
					ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE) + sizeof(UDWORD));
					ip = psCurrBlock->pCode;

					/* Code to store the value on the stack */
					PUT_PKOPCODE(ip, OP_PUSH, VAL_TRIGGER);
					PUT_DATA(ip, -1);

					psCurrBlock->type = VAL_TRIGGER;

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6635 "script_parser.c"
    break;

  case 180:
#line 4430 "script_parser.y"
                                {
					ALLOC_BLOCK(psCurrBlock, sizeof(OPCODE) + sizeof(UDWORD));
					ip = psCurrBlock->pCode;

					/* Code to store the value on the stack */
					PUT_PKOPCODE(ip, OP_PUSH, VAL_EVENT);
					PUT_DATA(ip, (yyvsp[0].eSymbol)->index);

					psCurrBlock->type = VAL_EVENT;

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6653 "script_parser.c"
    break;

  case 181:
#line 4451 "script_parser.y"
                                {
					codeRet = scriptCodeVarGet((yyvsp[0].vSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6665 "script_parser.c"
    break;

  case 182:
#line 4459 "script_parser.y"
                                {
					codeRet = scriptCodeConstant((yyvsp[0].cSymbol), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6677 "script_parser.c"
    break;

  case 183:
#line 4467 "script_parser.y"
                                {
					/* Generate the code for the function call */
					codeRet = scriptCodeFunction((yyvsp[-3].fSymbol), (yyvsp[-1].pblock), TRUE, &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6690 "script_parser.c"
    break;

  case 184:
#line 4476 "script_parser.y"
                                {
						UDWORD line,paramNumber;
						STRING *pDummy;

						if((yyvsp[-1].pblock)->numParams != (yyvsp[-3].eSymbol)->numParams)
						{
							debug(LOG_ERROR, "Wrong number of arguments for function call: '%s'. Expected %d parameters instead of  %d.", (yyvsp[-3].eSymbol)->pIdent, (yyvsp[-3].eSymbol)->numParams, (yyvsp[-1].pblock)->numParams);
							scr_error("Wrong number of arguments in function call");
							return CE_PARSE;
						}

						if(!(yyvsp[-3].eSymbol)->bFunction)
						{
							debug(LOG_ERROR, "'%s' is not a function", (yyvsp[-3].eSymbol)->pIdent);
							scr_error("Can't cann an event");
							return CE_PARSE;
						}

						/* make sure function has a return type */
						/* if($1->retType != OBJ_VAR) */
						if(asScrTypeTab[(yyvsp[-3].eSymbol)->retType - VAL_USERTYPESTART].accessType != AT_OBJECT)
						{
							scr_error("'%s' does not return an object value (%d )", (yyvsp[-3].eSymbol)->pIdent, (yyvsp[-3].eSymbol)->retType);
							return CE_PARSE;
						}

						/* check if right parameters were passed */
						paramNumber = checkFuncParamTypes((yyvsp[-3].eSymbol), (yyvsp[-1].pblock));
						if(paramNumber > 0)
						{
							debug(LOG_ERROR, "Parameter mismatch in function call: '%s'. Mismatch in parameter  %d.", (yyvsp[-3].eSymbol)->pIdent, paramNumber);
							YYABORT;
						}

						/* Allocate the code block */
						ALLOC_BLOCK(psCurrBlock, (yyvsp[-1].pblock)->size + sizeof(OPCODE) + sizeof(UDWORD));	//Params + Opcode + event index
						ALLOC_DEBUG(psCurrBlock, 1);
						ip = psCurrBlock->pCode;

						if((yyvsp[-1].pblock)->numParams > 0)	/* if any parameters declared */
						{
							/* Copy in the code for the parameters */
							PUT_BLOCK(ip, (yyvsp[-1].pblock));		//PUT_BLOCK(ip, psPBlock);
							FREE_PBLOCK((yyvsp[-1].pblock));		//FREE_PBLOCK(psPBlock);
						}

						/* Store the instruction */
						PUT_OPCODE(ip, OP_FUNC);
						PUT_FUNC(ip,(yyvsp[-3].eSymbol)->index);			//Put event index

						/* Add the debugging information */
						if (genDebugInfo)
						{
							psCurrBlock->psDebug[0].offset = 0;
							scriptGetErrorData((SDWORD *)&line, &pDummy);
							psCurrBlock->psDebug[0].line = line;
						}

						/* remember objexp type for further stuff, like myVar = objFunc(); to be able to check type equivalency */
						psCurrBlock->type = (yyvsp[-3].eSymbol)->retType;

						(yyval.cblock) = psCurrBlock;
				}
#line 6758 "script_parser.c"
    break;

  case 185:
#line 4540 "script_parser.y"
                                {
					codeRet = scriptCodeObjGet((yyvsp[0].objVarBlock), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6770 "script_parser.c"
    break;

  case 186:
#line 4548 "script_parser.y"
                                {
					codeRet = scriptCodeArrayGet((yyvsp[0].arrayBlock), &psCurrBlock);
					CHECK_CODE_ERROR(codeRet);

					/* Return the code block */
					(yyval.cblock) = psCurrBlock;
				}
#line 6782 "script_parser.c"
    break;

  case 187:
#line 4562 "script_parser.y"
                                {
					// Store the object type for the variable lookup
					objVarContext = (yyvsp[-1].cblock)->type;
				}
#line 6791 "script_parser.c"
    break;

  case 188:
#line 4573 "script_parser.y"
                                {
					codeRet = scriptCodeObjectVariable((yyvsp[-1].cblock), (yyvsp[0].vSymbol), &psObjVarBlock);
					CHECK_CODE_ERROR(codeRet);

					// reset the object type
					objVarContext = 0;

					/* Return the code block */
					(yyval.objVarBlock) = psObjVarBlock;
				}
#line 6806 "script_parser.c"
    break;

  case 189:
#line 4586 "script_parser.y"
                                {
					codeRet = scriptCodeObjectVariable((yyvsp[-1].cblock), (yyvsp[0].vSymbol), &psObjVarBlock);
					CHECK_CODE_ERROR(codeRet);

					// reset the object type
					objVarContext = 0;

					/* Return the code block */
					(yyval.objVarBlock) = psObjVarBlock;
				}
#line 6821 "script_parser.c"
    break;

  case 190:
#line 4599 "script_parser.y"
                                {
					codeRet = scriptCodeObjectVariable((yyvsp[-1].cblock), (yyvsp[0].vSymbol), &psObjVarBlock);
					CHECK_CODE_ERROR(codeRet);

					// reset the object type
					objVarContext = 0;

					/* Return the code block */
					(yyval.objVarBlock) = psObjVarBlock;
				}
#line 6836 "script_parser.c"
    break;

  case 191:
#line 4611 "script_parser.y"
                                {
					codeRet = scriptCodeObjectVariable((yyvsp[-1].cblock), (yyvsp[0].vSymbol), &psObjVarBlock);
					CHECK_CODE_ERROR(codeRet);

					// reset the object type
					objVarContext = 0;

					/* Return the code block */
					(yyval.objVarBlock) = psObjVarBlock;
				}
#line 6851 "script_parser.c"
    break;

  case 192:
#line 4630 "script_parser.y"
                                        {
						ALLOC_ARRAYBLOCK(psCurrArrayBlock, (yyvsp[-1].cblock)->size, NULL);
						ip = psCurrArrayBlock->pCode;

						/* Copy the index expression code into the code block */
						PUT_BLOCK(ip, (yyvsp[-1].cblock));
						FREE_BLOCK((yyvsp[-1].cblock));

						(yyval.arrayBlock) = psCurrArrayBlock;
					}
#line 6866 "script_parser.c"
    break;

  case 193:
#line 4643 "script_parser.y"
                                        {
						(yyval.arrayBlock) = (yyvsp[0].arrayBlock);
					}
#line 6874 "script_parser.c"
    break;

  case 194:
#line 4648 "script_parser.y"
                                        {
						ALLOC_ARRAYBLOCK(psCurrArrayBlock, (yyvsp[-3].arrayBlock)->size + (yyvsp[-1].cblock)->size, NULL);
						ip = psCurrArrayBlock->pCode;

						/* Copy the index expression code into the code block */
						psCurrArrayBlock->dimensions = (yyvsp[-3].arrayBlock)->dimensions + 1;
						PUT_BLOCK(ip, (yyvsp[-3].arrayBlock));
						PUT_BLOCK(ip, (yyvsp[-1].cblock));
						FREE_ARRAYBLOCK((yyvsp[-3].arrayBlock));
						FREE_ARRAYBLOCK((yyvsp[-1].cblock));

						(yyval.arrayBlock) = psCurrArrayBlock;
					}
#line 6892 "script_parser.c"
    break;

  case 195:
#line 4664 "script_parser.y"
                                        {
						codeRet = scriptCodeArrayVariable((yyvsp[0].arrayBlock), (yyvsp[-1].vSymbol), &psCurrArrayBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.arrayBlock) = psCurrArrayBlock;
					}
#line 6904 "script_parser.c"
    break;

  case 196:
#line 4674 "script_parser.y"
                                        {
						codeRet = scriptCodeArrayVariable((yyvsp[0].arrayBlock), (yyvsp[-1].vSymbol), &psCurrArrayBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.arrayBlock) = psCurrArrayBlock;
					}
#line 6916 "script_parser.c"
    break;

  case 197:
#line 4684 "script_parser.y"
                                        {
						codeRet = scriptCodeArrayVariable((yyvsp[0].arrayBlock), (yyvsp[-1].vSymbol), &psCurrArrayBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.arrayBlock) = psCurrArrayBlock;
					}
#line 6928 "script_parser.c"
    break;

  case 198:
#line 4694 "script_parser.y"
                                        {
						codeRet = scriptCodeArrayVariable((yyvsp[0].arrayBlock), (yyvsp[-1].vSymbol), &psCurrArrayBlock);
						CHECK_CODE_ERROR(codeRet);

						/* Return the code block */
						(yyval.arrayBlock) = psCurrArrayBlock;
					}
#line 6940 "script_parser.c"
    break;


#line 6944 "script_parser.c"

      default: break;
    }
  /* User semantic actions sometimes alter yychar, and that requires
     that yytoken be updated with the new translation.  We take the
     approach of translating immediately before every use of yytoken.
     One alternative is translating here after every semantic action,
     but that translation would be missed if the semantic action invokes
     YYABORT, YYACCEPT, or YYERROR immediately after altering yychar or
     if it invokes YYBACKUP.  In the case of YYABORT or YYACCEPT, an
     incorrect destructor might then be invoked immediately.  In the
     case of YYERROR or YYBACKUP, subsequent parser actions might lead
     to an incorrect destructor call or verbose syntax error message
     before the lookahead is translated.  */
  YY_SYMBOL_PRINT ("-> $$ =", yyr1[yyn], &yyval, &yyloc);

  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);

  *++yyvsp = yyval;

  /* Now 'shift' the result of the reduction.  Determine what state
     that goes to, based on the state we popped back to and the rule
     number reduced by.  */
  {
    const int yylhs = yyr1[yyn] - YYNTOKENS;
    const int yyi = yypgoto[yylhs] + *yyssp;
    yystate = (0 <= yyi && yyi <= YYLAST && yycheck[yyi] == *yyssp
               ? yytable[yyi]
               : yydefgoto[yylhs]);
  }

  goto yynewstate;


/*--------------------------------------.
| yyerrlab -- here on detecting error.  |
`--------------------------------------*/
yyerrlab:
  /* Make sure we have latest lookahead translation.  See comments at
     user semantic actions for why this is necessary.  */
  yytoken = yychar == YYEMPTY ? YYEMPTY : YYTRANSLATE (yychar);

  /* If not already recovering from an error, report this error.  */
  if (!yyerrstatus)
    {
      ++yynerrs;
#if ! YYERROR_VERBOSE
      yyerror (YY_("syntax error"));
#else
# define YYSYNTAX_ERROR yysyntax_error (&yymsg_alloc, &yymsg, \
                                        yyssp, yytoken)
      {
        char const *yymsgp = YY_("syntax error");
        int yysyntax_error_status;
        yysyntax_error_status = YYSYNTAX_ERROR;
        if (yysyntax_error_status == 0)
          yymsgp = yymsg;
        else if (yysyntax_error_status == 1)
          {
            if (yymsg != yymsgbuf)
              YYSTACK_FREE (yymsg);
            yymsg = YY_CAST (char *, YYSTACK_ALLOC (YY_CAST (YYSIZE_T, yymsg_alloc)));
            if (!yymsg)
              {
                yymsg = yymsgbuf;
                yymsg_alloc = sizeof yymsgbuf;
                yysyntax_error_status = 2;
              }
            else
              {
                yysyntax_error_status = YYSYNTAX_ERROR;
                yymsgp = yymsg;
              }
          }
        yyerror (yymsgp);
        if (yysyntax_error_status == 2)
          goto yyexhaustedlab;
      }
# undef YYSYNTAX_ERROR
#endif
    }



  if (yyerrstatus == 3)
    {
      /* If just tried and failed to reuse lookahead token after an
         error, discard it.  */

      if (yychar <= YYEOF)
        {
          /* Return failure if at end of input.  */
          if (yychar == YYEOF)
            YYABORT;
        }
      else
        {
          yydestruct ("Error: discarding",
                      yytoken, &yylval);
          yychar = YYEMPTY;
        }
    }

  /* Else will try to reuse lookahead token after shifting the error
     token.  */
  goto yyerrlab1;


/*---------------------------------------------------.
| yyerrorlab -- error raised explicitly by YYERROR.  |
`---------------------------------------------------*/
yyerrorlab:
  /* Pacify compilers when the user code never invokes YYERROR and the
     label yyerrorlab therefore never appears in user code.  */
  if (0)
    YYERROR;

  /* Do not reclaim the symbols of the rule whose action triggered
     this YYERROR.  */
  YYPOPSTACK (yylen);
  yylen = 0;
  YY_STACK_PRINT (yyss, yyssp);
  yystate = *yyssp;
  goto yyerrlab1;


/*-------------------------------------------------------------.
| yyerrlab1 -- common code for both syntax error and YYERROR.  |
`-------------------------------------------------------------*/
yyerrlab1:
  yyerrstatus = 3;      /* Each real token shifted decrements this.  */

  for (;;)
    {
      yyn = yypact[yystate];
      if (!yypact_value_is_default (yyn))
        {
          yyn += YYTERROR;
          if (0 <= yyn && yyn <= YYLAST && yycheck[yyn] == YYTERROR)
            {
              yyn = yytable[yyn];
              if (0 < yyn)
                break;
            }
        }

      /* Pop the current state because it cannot handle the error token.  */
      if (yyssp == yyss)
        YYABORT;


      yydestruct ("Error: popping",
                  yystos[yystate], yyvsp);
      YYPOPSTACK (1);
      yystate = *yyssp;
      YY_STACK_PRINT (yyss, yyssp);
    }

  YY_IGNORE_MAYBE_UNINITIALIZED_BEGIN
  *++yyvsp = yylval;
  YY_IGNORE_MAYBE_UNINITIALIZED_END


  /* Shift the error token.  */
  YY_SYMBOL_PRINT ("Shifting", yystos[yyn], yyvsp, yylsp);

  yystate = yyn;
  goto yynewstate;


/*-------------------------------------.
| yyacceptlab -- YYACCEPT comes here.  |
`-------------------------------------*/
yyacceptlab:
  yyresult = 0;
  goto yyreturn;


/*-----------------------------------.
| yyabortlab -- YYABORT comes here.  |
`-----------------------------------*/
yyabortlab:
  yyresult = 1;
  goto yyreturn;


#if !defined yyoverflow || YYERROR_VERBOSE
/*-------------------------------------------------.
| yyexhaustedlab -- memory exhaustion comes here.  |
`-------------------------------------------------*/
yyexhaustedlab:
  yyerror (YY_("memory exhausted"));
  yyresult = 2;
  /* Fall through.  */
#endif


/*-----------------------------------------------------.
| yyreturn -- parsing is finished, return the result.  |
`-----------------------------------------------------*/
yyreturn:
  if (yychar != YYEMPTY)
    {
      /* Make sure we have latest lookahead translation.  See comments at
         user semantic actions for why this is necessary.  */
      yytoken = YYTRANSLATE (yychar);
      yydestruct ("Cleanup: discarding lookahead",
                  yytoken, &yylval);
    }
  /* Do not reclaim the symbols of the rule whose action triggered
     this YYABORT or YYACCEPT.  */
  YYPOPSTACK (yylen);
  YY_STACK_PRINT (yyss, yyssp);
  while (yyssp != yyss)
    {
      yydestruct ("Cleanup: popping",
                  yystos[+*yyssp], yyvsp);
      YYPOPSTACK (1);
    }
#ifndef yyoverflow
  if (yyss != yyssa)
    YYSTACK_FREE (yyss);
#endif
#if YYERROR_VERBOSE
  if (yymsg != yymsgbuf)
    YYSTACK_FREE (yymsg);
#endif
  return yyresult;
}
#line 4703 "script_parser.y"


// Reset all the symbol tables
static void scriptResetTables(void)
{
	VAR_SYMBOL		*psCurr, *psNext;
	TRIGGER_SYMBOL	*psTCurr, *psTNext;
	EVENT_SYMBOL	*psECurr, *psENext;
	FUNC_SYMBOL		*psFCurr, *psFNext;

	SDWORD			i;

	/* start with global vars definition */
	localVariableDef = FALSE;
	//debug(LOG_SCRIPT, "localVariableDef = FALSE 4");

	/* Reset the global variable symbol table */
	for(psCurr = psGlobalVars; psCurr != NULL; psCurr = psNext)
	{
		psNext = psCurr->psNext;
		FREE(psCurr->pIdent);
		FREE(psCurr);
	}
	psGlobalVars = NULL;

	/* Reset the global variable symbol table */
	psCurEvent = NULL;
	for(i=0; i<maxEventsLocalVars; i++)
	{
		numEventLocalVars[i] = 0;

		for(psCurr = psLocalVarsB[i]; psCurr != NULL; psCurr = psNext)
		{
			psNext = psCurr->psNext;
			FREE(psCurr->pIdent);
			FREE(psCurr);
		}
		psLocalVarsB[i] = NULL;
	}

	/* Reset the temp local variable symbol table */
	for(psCurr = psLocalVarsTemp; psCurr != NULL; psCurr = psNext)
	{
		psNext = psCurr->psNext;
		FREE(psCurr->pIdent);
		FREE(psCurr);
	}
	psLocalVarsTemp = NULL;


	/* Reset the global array symbol table */
	for(psCurr = psGlobalArrays; psCurr != NULL; psCurr = psNext)
	{
		psNext = psCurr->psNext;
		FREE(psCurr->pIdent);
		FREE(psCurr);
	}
	psGlobalArrays = NULL;

	// Reset the trigger table
	for(psTCurr = psTriggers; psTCurr; psTCurr = psTNext)
	{
		psTNext = psTCurr->psNext;
		if (psTCurr->psDebug)
		{
			FREE(psTCurr->psDebug);
		}
		if (psTCurr->pCode)
		{
			FREE(psTCurr->pCode);
		}
		FREE(psTCurr->pIdent);
		FREE(psTCurr);
	}
	psTriggers = NULL;
	numTriggers = 0;

	// Reset the event table
	for(psECurr = psEvents; psECurr; psECurr = psENext)
	{
		psENext = psECurr->psNext;
		if (psECurr->psDebug)
		{
			FREE(psECurr->psDebug);
		}
		FREE(psECurr->pIdent);
		FREE(psECurr->pCode);
		FREE(psECurr);
	}
	psEvents = NULL;
	numEvents = 0;

	/* Reset the function symbol table */
	for(psFCurr = psFunctions; psFCurr != NULL; psFCurr = psFNext)
	{
		psFNext = psFCurr->psNext;
		FREE_DEBUG(psFCurr);
		FREE(psFCurr->pIdent);
		FREE(psFCurr->pCode);
		FREE(psFCurr);
	}
	psFunctions = NULL;
}

/* Compile a script program */
BOOL scriptCompile(char *pData, UDWORD fileSize,
				   SCRIPT_CODE **ppsProg, SCR_DEBUGTYPE debugType)
{
	// Tell lex about the input buffer
	scriptSetInputBuffer(pData, fileSize);

	scriptResetTables();
	psFinalProg = NULL;
	if (debugType == SCR_DEBUGINFO)
	{
		genDebugInfo = TRUE;
	}
	else
	{
		genDebugInfo = FALSE;
	}

	if (scr_parse() != 0)
	{
		return FALSE;
	}

	scriptResetTables();

	*ppsProg = psFinalProg;

	return TRUE;
}


/* A simple error reporting routine */
void scr_error(char *pMessage, ...)
{
	int		line;
	char	*text;
	va_list	args;
	STRING	aBuff[1024];

	va_start(args, pMessage);
	vsprintf(aBuff, pMessage, args);
	va_end(args);
	scriptGetErrorData(&line, &text);
#ifdef DEBUG
	debug( LOG_ERROR, "script parse error:\n%s at %s:%d\nToken: %d, Text: '%s'\n",
			  aBuff, GetLastResourceFilename(), line, scr_char, text );
	ASSERT( FALSE, "script parse error:\n%s at %s:%d\nToken: %d, Text: '%s'\n",
			  aBuff, GetLastResourceFilename(), line, scr_char, text );
#else
	//DBERROR(("script parse error:\n%s at line %d\nToken: %d, Text: '%s'\n",
	//		  pMessage, line, scr_char, text));
	debug( LOG_ERROR, "script parse error:\n'%s' at %s:%d\nToken: %d, Text: '%s'\n",
			  aBuff, GetLastResourceFilename(), line, scr_char, text );
	abort();
#endif
}


/* Look up a type symbol */
BOOL scriptLookUpType(STRING *pIdent, INTERP_TYPE *pType)
{
	UDWORD	i;

	//debug(LOG_SCRIPT, "scriptLookUpType");

	if (asScrTypeTab)
	{
		for(i=0; asScrTypeTab[i].typeID != 0; i++)
		{
			if (strcmp(asScrTypeTab[i].pIdent, pIdent) == 0)
			{
				*pType = asScrTypeTab[i].typeID;
				return TRUE;
			}
		}
	}

	//debug(LOG_SCRIPT, "END scriptLookUpType");

	return FALSE;
}


/* Reset the local variable symbol table at the end of a function */
void scriptClearLocalVariables(void)
{
	VAR_SYMBOL	*psCurr, *psNext;

	for(psCurr = psLocalVars; psCurr != NULL; psCurr = psNext)
	{
		psNext = psCurr->psNext;
		FREE(psCurr->pIdent);
		FREE(psCurr);
	}
}

/* pop passed argumrnts (if any) */
BOOL popArguments(UDWORD **ip_temp, SDWORD numParams)
{
	SDWORD			i;

	/* code to pop passed params right before the main code begins */
	for(i = numParams-1; i >= 0 ; i--)
	{
		PUT_PKOPCODE(*ip_temp, OP_POPLOCAL, i);		//pop paramerets into first i local params (must be declared manually)
	}

	return TRUE;
}


/* Add a new variable symbol.
 * If localVariableDef is true a local variable symbol is defined,
 * otherwise a global variable symbol is defined.
 */
//BOOL scriptAddVariable(STRING *pIdent, INTERP_TYPE type, STORAGE_TYPE storage, SDWORD elements)
BOOL scriptAddVariable(VAR_DECL *psStorage, VAR_IDENT_DECL *psVarIdent)
{
	VAR_SYMBOL		*psNew;
	SDWORD			i;//, size;

	VAR_SYMBOL **ppsVarSym;

	if(psStorage->storage == ST_LOCAL)
	{
		if(scriptLookUpVariable(psVarIdent->pIdent, &ppsVarSym))
		{
			debug(LOG_ERROR, "var found");
			debug(LOG_ERROR, "var=%s, index=%d of %d", psVarIdent->pIdent, (*ppsVarSym)->index, psCurEvent->numParams);
		}
	}

	/* Allocate the memory for the symbol structure */
	psNew = (VAR_SYMBOL *)MALLOC(sizeof(VAR_SYMBOL));
	if (psNew == NULL)
	{
		scr_error("Out of memory");
		return FALSE;
	}

	psNew->pIdent = psVarIdent->pIdent; //(STRING *)MALLOC(strlen(pIdent) + 1);
/*	if (psNew->pIdent == NULL)
	{
		scr_error("Out of memory");
		return FALSE;
	}*/

	/* Intialise the symbol structure */
//	strcpy(psNew->pIdent, pIdent);

	psNew->type = psStorage->type;
	psNew->storage = psStorage->storage;
	psNew->dimensions = psVarIdent->dimensions;


	for(i=0; i<psNew->dimensions; i++)
	{
		psNew->elements[i] = psVarIdent->elements[i];
	}
	if (psNew->dimensions == 0)
	{
		if(psStorage->storage != ST_LOCAL)	//if not a local var
		{
			if (psGlobalVars == NULL)
			{
				psNew->index = 0;
			}
			else
			{
				psNew->index = psGlobalVars->index + 1;
			}

			/* Add the symbol to the list */
			psNew->psNext = psGlobalVars;
			psGlobalVars = psNew;
		}
		else		//local var
		{
			if(psCurEvent == NULL)
				debug(LOG_ERROR, "Can't declare local variables before defining an event");

			//debug(LOG_SCRIPT, "local variable declared for event %d, type=%d \n", psCurEvent->index, psNew->type);
			//debug(LOG_SCRIPT, "%s \n", psNew->pIdent);

			if (psLocalVarsB[psCurEvent->index] == NULL)
			{
				psNew->index = 0;
			}
			else
			{
				psNew->index = psLocalVarsB[psCurEvent->index]->index + 1;
			}

			numEventLocalVars[psCurEvent->index] = numEventLocalVars[psCurEvent->index] + 1;

			psNew->psNext = psLocalVarsB[psCurEvent->index];
			psLocalVarsB[psCurEvent->index] = psNew;

			//debug(LOG_SCRIPT, "local variable declared. ");
		}
	}
	else
	{
		if (psGlobalArrays == NULL)
		{
			psNew->index = 0;
		}
		else
		{
			psNew->index = psGlobalArrays->index + 1;
		}

		psNew->psNext = psGlobalArrays;
		psGlobalArrays = psNew;
	}


	return TRUE;
}


/* Look up a variable symbol */
BOOL scriptLookUpVariable(STRING *pIdent, VAR_SYMBOL **ppsSym)
{
	VAR_SYMBOL		*psCurr;
	UDWORD			i;

	//debug(LOG_SCRIPT, "scriptLookUpVariable");

	/* See if the symbol is an object variable */
	if (asScrObjectVarTab && objVarContext != 0)
	{
		for(psCurr = asScrObjectVarTab; psCurr->pIdent != NULL; psCurr++)
		{
			if (interpCheckEquiv(psCurr->objType, objVarContext) &&
				strcmp(psCurr->pIdent, pIdent) == 0)
			{
				//debug(LOG_SCRIPT, "scriptLookUpVariable: object");
				*ppsSym = psCurr;
				return TRUE;
			}
		}
	}

	//debug(LOG_SCRIPT, "scriptLookUpVariable 1");

	/* See if the symbol is an external variable */
	if (asScrExternalTab)
	{
		for(psCurr = asScrExternalTab; psCurr->pIdent != NULL; psCurr++)
		{
			if (strcmp(psCurr->pIdent, pIdent) == 0)
			{
				//debug(LOG_SCRIPT, "scriptLookUpVariable: extern");
				*ppsSym = psCurr;
				return TRUE;
			}
		}
	}

	//debug(LOG_SCRIPT, "scriptLookUpVariable 2");

	/* See if the symbol is in the local variable list */
	for(psCurr = psLocalVars; psCurr != NULL; psCurr = psCurr->psNext)
	{
		if (strcmp(psCurr->pIdent, pIdent) == 0)
		{
			//debug(LOG_SCRIPT, "scriptLookUpVariable: local");
			*ppsSym = psCurr;
			return TRUE;
		}
	}

	//debug(LOG_SCRIPT, "scriptLookUpVariable 3");

	/* check local vars if we are inside of an event */
	if(psCurEvent != NULL)
	{
		if(psCurEvent->index >= maxEventsLocalVars)
			debug(LOG_ERROR, "psCurEvent->index (%d) >= maxEventsLocalVars", psCurEvent->index);

		i = psCurEvent->index;

		if(psLocalVarsB[i] != NULL)	//any vars stored for this event
		{
		int		line;
		char	*text;

			//debug(LOG_SCRIPT, "now checking event %s; index = %d\n", psCurEvent->pIdent, psCurEvent->index);
			scriptGetErrorData(&line, &text);
			for(psCurr =psLocalVarsB[i]; psCurr != NULL; psCurr = psCurr->psNext)
			{

				if(psCurr->pIdent == NULL)
				{
					debug(LOG_ERROR, "psCurr->pIdent == NULL");
					debug(LOG_ERROR, "psCurr->index = %d", psCurr->index);
				}

				//debug(LOG_SCRIPT, "start comparing, num local vars=%d, at line %d\n", numEventLocalVars[i], line);
				//debug(LOG_SCRIPT, "current var=%s\n", psCurr->pIdent);
				//debug(LOG_SCRIPT, "passed string=%s\n", pIdent);

				//debug(LOG_SCRIPT, "comparing %s with %s \n", psCurr->pIdent, pIdent);
				if (strcmp(psCurr->pIdent, pIdent) == 0)
				{
					//debug(LOG_SCRIPT, "4");
					//debug(LOG_SCRIPT, "scriptLookUpVariable - local var found, type=%d\n", psCurr->type);
					*ppsSym = psCurr;
					return TRUE;
				}
			}
		}
	}

	//debug(LOG_SCRIPT, "scriptLookUpVariable 4");


	/* See if the symbol is in the global variable list.
	 * This is not checked for when local variables are being defined.
	 * This allows local variables to have the same name as global ones.
	 */
	if (!localVariableDef)
	{
		for(psCurr = psGlobalVars; psCurr != NULL; psCurr = psCurr->psNext)
		{
			if (strcmp(psCurr->pIdent, pIdent) == 0)
			{
				*ppsSym = psCurr;
				return TRUE;
			}
		}
		for(psCurr = psGlobalArrays; psCurr != NULL; psCurr = psCurr->psNext)
		{
			if (strcmp(psCurr->pIdent, pIdent) == 0)
			{
				*ppsSym = psCurr;
				return TRUE;
			}
		}
	}

	/* Failed to find the variable */
	*ppsSym = NULL;

	//debug(LOG_SCRIPT, "END scriptLookUpVariable");
	return FALSE;
}


/* Add a new trigger symbol */
BOOL scriptAddTrigger(STRING *pIdent, TRIGGER_DECL *psDecl, UDWORD line)
{
	TRIGGER_SYMBOL		*psTrigger, *psCurr, *psPrev;

	// Allocate the trigger
	psTrigger = MALLOC(sizeof(TRIGGER_SYMBOL));
	if (!psTrigger)
	{
		scr_error("Out of memory");
		return FALSE;
	}
	psTrigger->pIdent = MALLOC(strlen(pIdent) + 1);
	if (!psTrigger->pIdent)
	{
		scr_error("Out of memory");
		return FALSE;
	}
	strcpy(psTrigger->pIdent, pIdent);
	if (psDecl->size > 0)
	{
		psTrigger->pCode = MALLOC(psDecl->size);
		if (!psTrigger->pCode)
		{
			scr_error("Out of memory");
			return FALSE;
		}
		memcpy(psTrigger->pCode, psDecl->pCode, psDecl->size);
	}
	else
	{
		psTrigger->pCode = NULL;
	}
	psTrigger->size = psDecl->size;
	psTrigger->type = psDecl->type;
	psTrigger->time = psDecl->time;
	psTrigger->index = numTriggers++;
	psTrigger->psNext = NULL;

	// Add debug info
	if (genDebugInfo)
	{
		psTrigger->psDebug = MALLOC(sizeof(SCRIPT_DEBUG));
		psTrigger->psDebug[0].offset = 0;
		psTrigger->psDebug[0].line = line;
		psTrigger->debugEntries = 1;
	}
	else
	{
		psTrigger->debugEntries = 0;
		psTrigger->psDebug = NULL;
	}


	// Store the trigger
	psPrev = NULL;
	for(psCurr = psTriggers; psCurr; psCurr = psCurr->psNext)
	{
		psPrev = psCurr;
	}
	if (psPrev)
	{
		psPrev->psNext = psTrigger;
	}
	else
	{
		psTriggers = psTrigger;
	}

	return TRUE;
}


/* Lookup a trigger symbol */
BOOL scriptLookUpTrigger(STRING *pIdent, TRIGGER_SYMBOL **ppsTrigger)
{
	TRIGGER_SYMBOL	*psCurr;

	//debug(LOG_SCRIPT, "scriptLookUpTrigger");

	for(psCurr = psTriggers; psCurr; psCurr=psCurr->psNext)
	{
		if (strcmp(pIdent, psCurr->pIdent) == 0)
		{
			//debug(LOG_SCRIPT, "scriptLookUpTrigger: found");
			*ppsTrigger = psCurr;
			return TRUE;
		}
	}

	//debug(LOG_SCRIPT, "END scriptLookUpTrigger");

	return FALSE;
}


/* Lookup a callback trigger symbol */
BOOL scriptLookUpCallback(STRING *pIdent, CALLBACK_SYMBOL **ppsCallback)
{
	CALLBACK_SYMBOL		*psCurr;

	//debug(LOG_SCRIPT, "scriptLookUpCallback");

	if (!asScrCallbackTab)
	{
		return FALSE;
	}

	for(psCurr = asScrCallbackTab; psCurr->type != 0; psCurr += 1)
	{
		if (strcmp(pIdent, psCurr->pIdent) == 0)
		{
			//debug(LOG_SCRIPT, "scriptLookUpCallback: found");
			*ppsCallback = psCurr;
			return TRUE;
		}
	}

	//debug(LOG_SCRIPT, "END scriptLookUpCallback: found");
	return FALSE;
}

/* Add a new event symbol */
BOOL scriptDeclareEvent(STRING *pIdent, EVENT_SYMBOL **ppsEvent, SDWORD numArgs)
{
	EVENT_SYMBOL		*psEvent, *psCurr, *psPrev;

	// Allocate the event
	psEvent = MALLOC(sizeof(EVENT_SYMBOL));
	if (!psEvent)
	{
		scr_error("Out of memory");
		return FALSE;
	}
	psEvent->pIdent = MALLOC(strlen(pIdent) + 1);
	if (!psEvent->pIdent)
	{
		scr_error("Out of memory");
		return FALSE;
	}
	strcpy(psEvent->pIdent, pIdent);
	psEvent->pCode = NULL;
	psEvent->size = 0;
	psEvent->psDebug = NULL;
	psEvent->debugEntries = 0;
	psEvent->index = numEvents++;
	psEvent->psNext = NULL;

	/* remember how many params this event has */
	psEvent->numParams = numArgs;
	psEvent->bFunction = FALSE;
	psEvent->bDeclared = FALSE;
	psEvent->retType = VAL_VOID;	/* functions can return a value */

	// Add the event to the list
	psPrev = NULL;
	for(psCurr = psEvents; psCurr; psCurr = psCurr->psNext)
	{
		psPrev = psCurr;
	}
	if (psPrev)
	{
		psPrev->psNext = psEvent;
	}
	else
	{
		psEvents = psEvent;
	}

	*ppsEvent = psEvent;

	return TRUE;
}

// Add the code to a defined event
BOOL scriptDefineEvent(EVENT_SYMBOL *psEvent, CODE_BLOCK *psCode, SDWORD trigger)
{
	VAR_SYMBOL		*psCurr, *psNext;


	if(psCode->size == 0)
		debug(LOG_ERROR, "Event '%s' is empty, please add atleast 1 statement", psEvent->pIdent);

	// events with arguments can't have a trigger assigned
	if(psEvent->numParams > 0 && trigger >= 0)
		debug(LOG_ERROR, "Events with parameters can't have a trigger assigned, event: '%s' ", psEvent->pIdent);

	// Store the event code
	psEvent->pCode = MALLOC(psCode->size);
	if (!psEvent->pCode)
	{
		scr_error("Out of memory");
		return FALSE;
	}

	memcpy(psEvent->pCode, psCode->pCode, psCode->size);
	psEvent->size = psCode->size;
	psEvent->trigger = trigger;



	// Add debug info
	if (genDebugInfo)
	{
		psEvent->psDebug = MALLOC(sizeof(SCRIPT_DEBUG) * psCode->debugEntries);

		if (!psEvent->psDebug)
		{

			scr_error("Out of memory");
			return FALSE;
		}

		memcpy(psEvent->psDebug, psCode->psDebug,
			sizeof(SCRIPT_DEBUG) * psCode->debugEntries);
		psEvent->debugEntries = psCode->debugEntries;
	}
	else
	{
		psEvent->debugEntries = 0;
		psEvent->psDebug = NULL;
	}

	//debug(LOG_SCRIPT, "before define event");

	/* store local vars */
	if(psEvent->index >= maxEventsLocalVars)
		debug(LOG_ERROR, "scriptDefineEvent - psEvent->index >= maxEventsLocalVars");

	return TRUE;
}

/* Lookup an event symbol */
BOOL scriptLookUpEvent(STRING *pIdent, EVENT_SYMBOL **ppsEvent)
{
	EVENT_SYMBOL	*psCurr;
	//debug(LOG_SCRIPT, "scriptLookUpEvent");

	for(psCurr = psEvents; psCurr; psCurr=psCurr->psNext)
	{
		if (strcmp(pIdent, psCurr->pIdent) == 0)
		{
			//debug(LOG_SCRIPT, "scriptLookUpEvent:found");
			*ppsEvent = psCurr;
			return TRUE;
		}
	}
	//debug(LOG_SCRIPT, "END scriptLookUpEvent");
	return FALSE;
}


/* Look up a constant variable symbol */
BOOL scriptLookUpConstant(STRING *pIdent, CONST_SYMBOL **ppsSym)
{
	CONST_SYMBOL	*psCurr;

	//debug(LOG_SCRIPT, "scriptLookUpConstant");

	/* Scan the Constant list */
	if (asScrConstantTab)
	{
		for(psCurr = asScrConstantTab; psCurr->type != VAL_VOID; psCurr++)
		{
			if (strcmp(psCurr->pIdent, pIdent) == 0)
			{
				*ppsSym = psCurr;
				return TRUE;
			}
		}
	}

	//debug(LOG_SCRIPT, "END scriptLookUpConstant");

	return FALSE;
}


/* Look up a function symbol */
BOOL scriptLookUpFunction(STRING *pIdent, FUNC_SYMBOL **ppsSym)
{
	UDWORD i;
	FUNC_SYMBOL	*psCurr;

	//debug(LOG_SCRIPT, "scriptLookUpFunction");

	/* See if the function is defined as an instinct function */
	if (asScrInstinctTab)
	{
		for(i = 0; asScrInstinctTab[i].pFunc != NULL; i++)
		{
			if (strcmp(asScrInstinctTab[i].pIdent, pIdent) == 0)
			{
				*ppsSym = asScrInstinctTab + i;
				return TRUE;
			}
		}
	}

	/* See if the function is defined as a script function */
	for(psCurr = psFunctions; psCurr != NULL; psCurr = psCurr->psNext)
	{
		if (strcmp(psCurr->pIdent, pIdent) == 0)
		{
			*ppsSym = psCurr;
			return TRUE;
		}
	}

	/* Failed to find the indentifier */
	*ppsSym = NULL;

	//debug(LOG_SCRIPT, "END scriptLookUpFunction");
	return FALSE;
}

/* Look up a function symbol defined in script */
BOOL scriptLookUpCustomFunction(STRING *pIdent, EVENT_SYMBOL **ppsSym)
{
	UDWORD i;
	EVENT_SYMBOL	*psCurr;

	//debug(LOG_SCRIPT, "scriptLookUpCustomFunction");

	/* See if the function is defined as a script function */
	for(psCurr = psEvents; psCurr; psCurr = psCurr->psNext)
	{
		if(psCurr->bFunction)	/* event defined as function */
		{
			if (strcmp(psCurr->pIdent, pIdent) == 0)
			{
				//debug(LOG_SCRIPT, "scriptLookUpCustomFunction: %s is a custom function", pIdent);
				*ppsSym = psCurr;
				return TRUE;
			}
		}
	}

	/* Failed to find the indentifier */
	*ppsSym = NULL;

	//debug(LOG_SCRIPT, "END scriptLookUpCustomFunction");
	return FALSE;
}


/* Set the type table */
void scriptSetTypeTab(TYPE_SYMBOL *psTypeTab)
{
#ifdef DEBUG
	SDWORD			i;
	INTERP_TYPE		type;

	for(i=0, type=VAL_USERTYPESTART; psTypeTab[i].typeID != 0; i++)
	{
		ASSERT( psTypeTab[i].typeID == type,
			"scriptSetTypeTab: ID's must be >= VAL_USERTYPESTART and sequential" );
		type += 1;
	}
#endif

	asScrTypeTab = psTypeTab;
}


/* Set the function table */
void scriptSetFuncTab(FUNC_SYMBOL *psFuncTab)
{
	asScrInstinctTab = psFuncTab;
}

/* Set the object variable table */
void scriptSetObjectTab(VAR_SYMBOL *psObjTab)
{
	asScrObjectVarTab = psObjTab;
}

/* Set the external variable table */
void scriptSetExternalTab(VAR_SYMBOL *psExtTab)
{
	asScrExternalTab = psExtTab;
}

/* Set the constant table */
void scriptSetConstTab(CONST_SYMBOL *psConstTab)
{
	asScrConstantTab = psConstTab;
}

/* Set the callback table */
void scriptSetCallbackTab(CALLBACK_SYMBOL *psCallTab)
{
#ifdef DEBUG
	SDWORD			i;
	TRIGGER_TYPE	type;

	for(i=0, type=TR_CALLBACKSTART; psCallTab[i].type != 0; i++)
	{
		ASSERT( psCallTab[i].type == type,
			"scriptSetCallbackTab: ID's must be >= VAL_CALLBACKSTART and sequential" );
		type += 1;
	}
#endif

	asScrCallbackTab = psCallTab;
}
