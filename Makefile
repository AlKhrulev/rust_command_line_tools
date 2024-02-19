.PHONY: link clean info

EXECUTABLE= ./target/release/timeit

link:
	ln -s $(EXECUTABLE) ./timeit

info:
	ls -lh $(EXECUTABLE)

clean:
	cargo clean && rm -v ./timeit

