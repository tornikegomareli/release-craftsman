prog := releasecraftsman
version ?= latest

debug ?=
ifdef debug
  release :=
  target := debug
  extension := debug
else
  release := --release
  target := release
  extension :=
endif

os := $(shell uname -s | tr '[:upper:]' '[:lower:]')

# need to change this in future
arch := x86_64

tar_name := $(prog)-$(version)-$(arch)-$(os).tar.gz

$(info debug is $(debug))
$(info version is $(version))
$(info OS is $(os))
$(info Arch is $(arch))

build:
	cargo build $(release)

install:
	cp target/$(target)/$(prog) /usr/local/bin/

package: build
	cd target/$(target) && tar -czf $(tar_name) $(prog)

checksum: package
	shasum -a 256 target/$(target)/$(tar_name)

all: build install package checksum

help:
	@echo "usage: make $(prog) [debug=1] [version=x.y.z]"
