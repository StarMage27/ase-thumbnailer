bin_name = ase-thumbnailer
thumb_name = aseprite.thumbnailer
xml_name = igara-aseprite.xml
release_path = target/release
bin_path = $(HOME)/.local/bin

default_target: build

build:
	cargo build

run:
	cargo run

build-release:
	cargo build --release

run-release:
	cargo run --release

bloat:
	cargo bloat --release --crates

install: build-release
	cp $(release_path)/$(bin_name) $(bin_path)/
	chmod +x $(bin_path)/$(bin_name)
	cp $(thumb_name) $(XDG_DATA_HOME)/thumbnailers/
	xdg-mime install --mode user $(xml_name)
