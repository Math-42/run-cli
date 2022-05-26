SOURCES = $(shell find ./src -name "*.py")
MAIN = main.py

all:
	echo "A python project didn't need to be compiled"
run:
	@python3 main.py
test:
	@run-cli test
zip: clean
	zip -r submission $(MAIN) $(SOURCES) Makefile
clean:
	rm -f submission.zip bin