EXTENSION = ""
SOURCES = $(shell find ./t -name "*.$(EXTENSION)")
MAIN =  

all:

run:

test:
	@run-cli test
zip: clean
	zip -r submission $(MAIN) $(SOURCES) Makefile
clean:
	rm -f submission.zip