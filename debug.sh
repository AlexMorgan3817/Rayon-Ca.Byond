cargo b
cp target/i686-unknown-linux-gnu/debug/librayon_ca_byond.so test/lib.so
cd test
DreamMaker -DLINUX -DVERBOSE test.dme
DreamDaemon test.dmb -trusted
