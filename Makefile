.PHONY: dependencies

dependencies:
	@if ! [ -d "deps/metaplex" ]; then \
		git submodule add --force https://github.com/metaplex-foundation/metaplex-program-library deps/metaplex-program-library; \
	fi
	@echo "installing npm packages"
	yarn
	@echo "installing submodules"
	git submodule update --recursive --init
	@echo "building token-metadata program"
	cd deps/metaplex-program-library/token-metadata/program && cargo build-bpf && cd ../../../../
