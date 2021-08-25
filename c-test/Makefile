build: main.c ../target/debug/libcircleci.so
	gcc -g -o build/circleci main.c -L../target/debug -lcircleci

gui: gui.c ../target/debug/libcircleci.so
	gcc -g -o build/circleci-gui gui.c -L../target/debug -lcircleci `pkg-config --cflags --libs gtk4`
