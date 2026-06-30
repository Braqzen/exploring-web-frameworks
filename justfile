default: build-servers

# -- Docker --
build-servers: build-axum build-actix build-warp build-rocket build-poem build-salvo

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

build-generator:
	docker rmi servers-generator:latest 2>/dev/null || true
	docker buildx bake -f docker/build.hcl generator

run:
	docker compose up -d
	@echo Grafana: http://localhost:3000/dashboards

stop:
	docker compose down

clean:
	docker compose down -v