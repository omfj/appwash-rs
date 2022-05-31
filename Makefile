prog :=appwash

install:
	cp target/release/appwash-cli /usr/local/bin/$(prog)

build:
	cargo build --release

clean:
	rm -rf /usr/local/bin/appwash

all: build install

