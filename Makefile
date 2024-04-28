
build:
	docker build . -f deployments/Dockerfile -t 192.168.10.102:32000/wachasit:latest

push:
	docker push 192.168.10.102:32000/wachasit:latest

