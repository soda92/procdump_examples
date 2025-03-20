all: build

.PHONY: build

build:
	cmake --preset windows
	cmake --build build
	gen_lsp_tdm

debug:
	cmake --preset windows-debug
	cmake --build build-debug
	gen_lsp_tdm

clean:
	pwsh -nop -c "rm -r -Force build"
