noinst_LIBRARIES = libwidget.a
AM_CPPFLAGS = -I$(top_srcdir)
libwidget_a_SOURCES = \
	bar.c \
	button.c \
	editbox.c \
	form.c \
	label.c \
	scrap.c \
	slider.c \
	tip.c \
	widget.c \
	bar.h \
	button.h \
	editbox.h \
	form.h \
	label.h \
	scrap.h \
	slider.h \
	tip.h \
	widgbase.h \
	widget.h \
	widgint.h

libwidget_a_libs = ../framework/libframework.a \
		../$(RENDERER)/lib$(RENDERER).a \
		../ivis_common/libivis_common.a
libwidget_a_DEPENDENCIES = $(libwidget_a_libs)
libwidget_a_LIBADD = $(libwidget_a_libs)
