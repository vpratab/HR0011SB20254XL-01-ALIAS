.PHONY: build test evidence summary package

build:
	./build.sh

test:
	. "${HOME}/.cargo/env" && cargo test --workspace

evidence:
	./scripts/run_scenarios.sh

summary:
	./scripts/summarize_evidence.py .

package:
	./scripts/prepare_package.sh
