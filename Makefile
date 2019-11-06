CLIPPY_ARGS = --all-targets --all-features -- -D clippy::wildcard_dependencies -D clippy::cargo_common_metadata -D warnings
COVERAGE_PACKAGES = nb2
COVERAGE_EXCLUDES = macros/*

.PHONY: build clean fmt lint test watch watch-bench watch-test

build:
	@cargo build

build-rel:
	@cargo build --release

clean:
	@cargo clean

coverage:
	@cargo tarpaulin -l -p $(COVERAGE_PACKAGES) --exclude-files $(COVERAGE_EXCLUDES) --out Xml

fmt:
	@cargo fmt

lint:
	@cargo clippy $(CLIPPY_ARGS)

test:
	@cargo test

watch:
ifdef WATCH
	@cargo watch --poll -x build -w $(WATCH)
else
	@cargo watch --poll -x build --all
endif

watch-bench:
ifdef WATCH
	@cargo watch --poll -x bench -w $(WATCH)
else
	@cargo watch --poll -x bench --all
endif

watch-test:
ifdef WATCH
	@cargo watch --poll -x test -w $(WATCH)
else
	@cargo watch --poll -x test --all
endif


