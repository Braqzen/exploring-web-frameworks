default: build-servers

# -- Docker --
build-servers: build-axum

build-axum:
	docker rmi servers-axum:latest 2>/dev/null || true
	docker build -t servers-axum -f services/axum/Dockerfile .

build-generator:
	docker rmi servers-generator:latest 2>/dev/null || true
	docker build -t servers-generator -f generator/Dockerfile .

run:
	docker compose up -d
	@echo Grafana: http://localhost:3000/dashboards

stop:
	docker compose down -v
