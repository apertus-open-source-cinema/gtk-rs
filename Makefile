ERROR_COLOR=\x1b[31;01m
OK_COLOR=\x1b[32;01m
NO_COLOR=\x1b[0m

LIB_GTK_GLUE = 	libgtk_glue
SRC = 			gtk_glue/gtk_glue.c
INC = 			$(shell pkg-config --cflags gtk+-3.0)
LIBS = 			$(shell pkg-config --libs gtk+-3.0)
GCC = 			gcc
RUSTC = 		rustc
RGTK_LIB_RS = 	lib.rs
LIB_RGTK =		librgtk.so
LIB_PATH = 		lib/
TEST_SRC =		test/main.rs
TEST = 			gtk_test
BIN_PATH = 		bin/
OS =			$(shell uname)

all: 				rgtk

glue: 				$(LIB_GTK_GLUE)

$(LIB_GTK_GLUE):	$(SRC)
					@echo "$(OK_COLOR) Creating lib/ folder $(NO_COLOR)"
					mkdir -p $(LIB_PATH)
					@echo "$(OK_COLOR) Building lib gtk_glue $(NO_COLOR)"
ifeq ($(OS), Darwin)
					$(GCC) $(INC) $(LIBS) -dynamiclib -o  $(LIB_PATH)$(LIB_GTK_GLUE).dylib $(SRC)
else
					$(GCC) $(INC) $(LIBS) -fPIC -c $(SRC)
					gcc -shared gtk_glue.o -o $(LIB_PATH)$(LIB_GTK_GLUE).so
endif

rgtk:				$(LIB_RGTK)

$(LIB_RGTK):		$(RGTK_LIB_RS)
					mkdir -p $(LIB_PATH)
					@echo "$(OK_COLOR) Building librgtk $(NO_COLOR)"
					$(RUSTC) -L $(LIB_PATH) $(RGTK_LIB_RS) --out-dir=$(LIB_PATH)

test:				$(TEST)

$(TEST):			$(TEST_SRC)
					@echo "$(OK_COLOR) Creating bin/ folder $(NO_COLOR)"
					mkdir -p $(BIN_PATH)
					@echo "$(OK_COLOR) Building test $(NO_COLOR)"
					$(RUSTC) -L $(LIB_PATH) $(TEST_SRC) -o $(BIN_PATH)$(TEST)

full:				glue rgtk test

clean:
					rm -rf $(LIB_PATH)

fclean:				clean
					rm -rf $(BIN_PATH)


re: 				clean all
