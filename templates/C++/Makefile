SOURCES = $(shell find ./src -name "*.cpp")
HEADERS = $(shell find ./src -name "*.hpp")
MAIN = main.cpp

all:
	@g++ -std=c++17 $(MAIN) $(SOURCES) -o bin
run:
	@./bin
test:
	@run-cli test
zip: clean
	zip -r submission $(MAIN) $(SOURCES) $(HEADERS) Makefile
clean:
	rm -f submission.zip bin