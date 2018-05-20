.PHONY: watch
watch:
	cargo watch -i "*.so" -c -s "cp target/debug/libmyrustpylib.so myrustpylib.so && python main.py" -x build
