# Document:
# https://gist.github.com/prongbang/7c8a17a493fcabecc01ac4a949194243

# make create_secret
create_secret:
	kubectl create secret docker-registry mysecretname --docker-server=https://ghcr.io --docker-username=mygithubusername --docker-password=mygithubreadtoken --docker-email=mygithubemail -n wcs-production

# make deploy_deployment
deploy_deployment:
	kubectl create -f deployment.yaml -n wcs-production

# make deploy_service
deploy_service:
	kubectl create -f service.yaml -n wcs-production

# make get_service (EXTERNAL-IP)
#NAME    TYPE           CLUSTER-IP      EXTERNAL-IP      PORT(S)        AGE
#nginx   LoadBalancer   10.152.183.27   192.168.10.110   80:32662/TCP   18s
get_service:
	kubectl get service -n wcs-production