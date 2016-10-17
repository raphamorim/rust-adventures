rustc --crate-type=lib src/docker.rs -o src/libdocker.rlib
rustc ./src/main.rs --extern docker=./src/libdocker.rlib -o docker \
    && ./docker && rm docker
