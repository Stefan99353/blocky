build:
	meson build
	ninja -C build

build-dev:
	meson build -Dprofile=development
	ninja -C build

install:
	ninja -C build install

clean:
	rm -rf target build