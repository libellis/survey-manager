.PHONY: build

build: 
	docker-compose -f deploy/docker-compose.yml build

up:
	docker-compose -f deploy/docker-compose.yml up -d

down:
	docker-compose -f deploy/docker-compose.yml down

clean:
	@echo 'stopping docker containers'
	@docker stop `docker ps -aq`
	@echo 'removing all node containers'
	@docker ps -a | awk '{ print $$1,$$2 }' | grep libellis | awk '{ print $$1 }' | xargs -I {} docker rm {}
