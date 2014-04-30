RUSTC=rustc

SRCS=src/main.rs
NAME=TicTacToe

ifeq ($(OS),Windows_NT)
	NAME:=$(NAME).exe
endif

$(NAME):
	$(RUSTC) -Llib $(SRCS) -o $(NAME)

.PHONY: clean
clean:
	rm -f $(NAME)
