# make up
up:
	docker-compose up -d

# make down
down:
	docker-compose down

# make restart
restart:
	docker-compose restart

# make build
build:
	docker-compose build

doc build:
	docker build --build-arg DATABASE_URL=surrealdb://db:8000 -t itsuki/rustapp:1.0.0 .

# make rebuild
rebuild:
	docker-compose up --build -d

# make images
images:
	docker images

# make logs db
logs:
	docker-compose logs ${DB_CONTAINER_NAME}

# make ps
ps:
	docker ps

# make psall
ps a:
	docker ps -a

# make prune
prune:
	docker system prune

# make clean 不要なDockerリソースを削除
clean:
	docker system prune -a --volumes

# make volume
volume:
	docker volume ls

# make exec app=service_name 特定のサービスにシェルアクセス
exec:
	docker-compose exec $(app) /bin/bash

# make db
db:
	docker-compose exec ${DB_CONTAINER_NAME} /bin/bash