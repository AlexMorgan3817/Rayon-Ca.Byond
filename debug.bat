@echo off

cargo build --target i686-pc-windows-msvc
copy target\\i686-pc-windows-msvc\\debug\\rayon_ca_byond.dll test\\lib.dll

cd test
	dm -DVERBOSE test.dme
	dd test.dmb -trusted
	cd ..
