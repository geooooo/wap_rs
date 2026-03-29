.DEFAULT_GOAL := dev

dev:
	trunk serve

build:
	trunk build --release

clean:
	trunk clean