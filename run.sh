make clean
.venv/bin/intercept-build make -j4
~/.cargo/bin/c2rust transpile -b main --fail-on-error compile_commands.json -- $INCLUDES
