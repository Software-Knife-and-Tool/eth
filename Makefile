#
# eth makefile
#
.PHONY: release debug run world commit config clean

release:
	@cargo build --release

debug:
	@cargo build

run:
	@cargo run

world: release debug run commit clean

config:
	@cp src/config/* $(HOME)/.config/eth

commit:
	@cargo fmt
	@echo ";;; rust tests"
	@cargo -q test | sed -e '/^$$/d'
	@echo ";;; clippy tests"
	@cargo clippy

clean:
	@rm -rf target
	@cargo update
	@rm -f TAGS
