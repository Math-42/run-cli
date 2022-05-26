SOURCES = $(shell find ./src -name "*.c")
HEADERS = $(shell find ./src -name "*.h")
MAIN = main.cpp

all:
	gcc $(SOURCES) -o bin
run:
	@./bin
test:
	@run-cli test
zip: clean
	zip -r submission $(MAIN) $(SOURCES) $(HEADERS) Makefile
clean:
	rm -f bin submission.zip