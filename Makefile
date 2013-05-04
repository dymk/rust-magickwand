ifeq ($(OS),Windows_NT)
	RUSTC=rustc.exe
else
	RUSTC=rustc
endif

all: magick_wand
test: clean test_magick_wand
	./build/test_magick_wand
run: test

magick_wand: src/magick_wand.rs
	$(RUSTC)   src/magick_wand.rs -o build/magick_wand

test_magick_wand: src/magick_wand.rs
	$(RUSTC) --test src/magick_wand.rs -o build/test_magick_wand

clean:
	rm -f build/*.dll
	rm -f build/*.exe
	rm -f test/written/*
