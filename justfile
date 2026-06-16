default: build-servers

# -- Docker --
build-servers: build-axum

build-axum:
	docker rmi servers-axum:latest 2>/dev/null || true
	docker build -t servers-axum ./services/axum

build-generator:
	docker rmi servers-generator:latest 2>/dev/null || true
	docker build -t servers-generator ./generator

build-ui:
	docker rmi servers-ui:latest 2>/dev/null || true
	docker build -t servers-ui -f ui/Dockerfile ui

run:
	docker compose up -d
	@echo Website: http://localhost:8080/

stop:
	docker compose down -v
