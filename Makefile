.PHONY: all clean configs

all:
	cargo build --all --release

configs:
	cargo build --bin genconfig --release
	for f in 1 4 8 16 32 ; do \
		N=$$(( 2*$$f + 1 )) ; \
		mkdir -p testdata/n$$N-f$$f ; \
		./target/release/genconfig -n $$N -d 5000 --base_port 4000 --target testdata/n$$N-f$$f ;\
	done
	@mkdir -p testdata/test
	@./target/release/genconfig -n 7 -d 50 --base_port 4000 --target testdata/test
	@mkdir -p testdata/test-local
	@./target/release/genconfig -n 4 -d 50 --base_port 4000 --target testdata/test-local
	@mkdir -p testdata/n4-f1
	@./target/release/genconfig -n 4 -d 5000 --base_port 4000 --target testdata/n4-f1