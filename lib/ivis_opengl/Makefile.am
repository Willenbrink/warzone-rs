noinst_LIBRARIES = libivis_opengl.a
AM_CPPFLAGS = -I$(top_srcdir)
libivis_opengl_a_SOURCES = \
	bspimd.c \
	ivi.c \
	pieblitfunc.c \
	piedraw.c \
	piefunc.c \
	piematrix.c \
	piemode.c \
	piepalette.c \
	piestate.c \
	pietexture.c \
	rendfunc.c \
	rendmode.c \
	screen.c \
	tex.c \
	textdraw.c \
	ivis02.h \
	piematrix.h \
	pietexture.h \
	screen.h

libivis_opengl_a_libs = ../framework/libframework.a ../ivis_common/libivis_common.a
libivis_opengl_a_DEPENDENCIES = $(libivis_opengl_a_libs)
libivis_opengl_a_LIBADD = $(libivis_opengl_a_libs)
