setup:
	brew install krakend
	brew install wrk

build-server:
	cd server && cargo build --release

run-server:
	./server/target/release/server

start-service:
	cd krakend && make run
	cd zolly && make run

load_test_direct:
	wrk -t12 -c100 -d60s http://127.0.0.1:8000/v1/hello

load_test_zolly:
	wrk -t12 -c100 -d60s http://127.0.0.1:8070/v1/hello

load_test_krakend:
	wrk -t12 -c100 -d60s http://127.0.0.1:8090/v1/hello