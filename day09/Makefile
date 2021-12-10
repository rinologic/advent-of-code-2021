day = day09
.DEFAULT_GOAL := default
data = data.txt
sample = data_sample.txt

sample:
	cargo build
	@ if [ -f $(sample) ]; then cat $(sample) | ./target/debug/$(day); fi

default:
	cargo build
	@ if [ -f $(data) ]; then cat $(data) | ./target/debug/$(day); fi
