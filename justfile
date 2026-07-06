# If you run "just" it will default to building everything
default: build-generator build-apis

# Shorthand to build all the APIs
build-apis: build-rust-apis build-typescript-apis

# Shorthand to build the APIs per language
build-rust-apis: build-axum build-actix build-warp build-rocket build-poem build-salvo
build-typescript-apis: build-express build-fastify

build-generator:
	docker rmi servers-generator:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl generator

# Rust APIs
build-axum:
	docker rmi servers-axum:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl axum

build-actix:
	docker rmi servers-actix:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl actix

build-warp:
	docker rmi servers-warp:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl warp

build-rocket:
	docker rmi servers-rocket:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl rocket

build-poem:
	docker rmi servers-poem:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl poem

build-salvo:
	docker rmi servers-salvo:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl salvo

# TypeScript APIs
build-express:
	docker rmi servers-express:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl express

build-fastify:
	docker rmi servers-fastify:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl fastify

# Docker Compose Commands
run:
	docker compose -f docker/docker-compose.yml up -d
	@echo Grafana: http://localhost:3000/dashboards

stop:
	docker compose -f docker/docker-compose.yml down

clean:
	docker compose -f docker/docker-compose.yml down -v
