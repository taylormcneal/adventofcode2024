CC = gcc
CFLAGS = -Wall -Wextra -std=c99

# Directories
COMMON_DIR = ../../common

# Sources
SOURCES = main.c $(COMMON_DIR)/utils.c
OBJECTS = $(SOURCES:.c=.o)
EXEC = solution

# Targets
all: $(EXEC)

$(EXEC): $(OBJECTS)
	$(CC) $(CFLAGS) -o $(EXEC) $(OBJECTS)

.c.o:
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm -f $(OBJECTS) $(EXEC)
