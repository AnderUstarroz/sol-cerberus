.PHONY: dependencies

dependencies:
	@if ! [ -d "deps/mpl-token-metadata" ]; then \
		git submodule add --force https://github.com/metaplex-foundation/mpl-token-metadata deps/mpl-token-metadata; \
	fi
	@echo "installing npm packages"
	yarn
	@echo "installing submodules"
	git submodule update --recursive --init
	@echo "building token-metadata program"
	cd deps/mpl-token-metadata/programs/token-metadata/program && cargo build-bpf && cd ../../../../
