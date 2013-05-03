ifeq ($(OS),Windows_NT)
	RUSTC=rustc.exe
else
	RUSTC=rustc
endif

all: magick_wand.rs
test: clean test_magick_wand.rs
	./build/test_magick_wand
run: test

magick_wand.rs:
	$(RUSTC) src/magick_wand.rs -o build/magick_wand.exe

test_magick_wand.rs:
	$(RUSTC) --test src/magick_wand.rs -o build/test_magick_wand.exe

clean:
	rm -f build/*.dll
	rm -f build/*.exe
	rm -f test/written/*
