#make clean
#.venv/bin/intercept-build make -j4
#rm -r ./rust/
#~/.cargo/bin/c2rust transpile -b main --emit-no-std -r --translate-const-macros -o rust --fail-on-error compile_commands.json -- $INCLUDES

make -j4
~/.cargo/bin/c2rust transpile -b main --overwrite-existing -o rust --fail-on-error compile_commands.json -- $INCLUDES
cd rust
cargo build
