# Document:
# https://gist.github.com/prongbang/7c8a17a493fcabecc01ac4a949194243

NAMESPACE ?= itnt-production

# make create_secret
create_secret:
	kubectl create secret docker-registry mysecretname --docker-server=https://ghcr.io --docker-username=mygithubusername --docker-password=mygithubreadtoken --docker-email=mygithubemail -n $(NAMESPACE)

# make deploy_deployment
deploy_deployment:
	kubectl apply -f deployment.yaml -n $(NAMESPACE)

# make deploy_service
deploy_service:
	kubectl apply -f service.yaml -n $(NAMESPACE)

# make deploy_all
deploy_all:
	kubectl apply -f deployment.yaml -n $(NAMESPACE)
	kubectl apply -f service.yaml -n $(NAMESPACE)

# make rollout_status
rollout_status:
	kubectl rollout status deploy/inteniquetic -n $(NAMESPACE)

# make logs
logs:
	kubectl logs deploy/inteniquetic -n $(NAMESPACE) --tail=200

# make get_service (EXTERNAL-IP)
#NAME    TYPE           CLUSTER-IP      EXTERNAL-IP      PORT(S)        AGE
#nginx   LoadBalancer   10.152.183.27   192.168.10.110   80:32662/TCP   18s
get_service:
	kubectl get service -n $(NAMESPACE)
