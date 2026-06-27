default: build-servers

# -- Docker --
build-servers: build-axum build-actix build-warp

build-axum:
	docker rmi servers-axum:latest 2>/dev/null || true
	docker build -t servers-axum -f services/rust/axum/Dockerfile .

build-actix:
	docker rmi servers-actix:latest 2>/dev/null || true
	docker build -t servers-actix -f services/rust/actix/Dockerfile .

build-warp:
	docker rmi servers-warp:latest 2>/dev/null || true
	docker build -t servers-warp -f services/rust/warp/Dockerfile .

build-generator:
	docker rmi servers-generator:latest 2>/dev/null || true
	docker build -t servers-generator -f generator/Dockerfile .

run:
	docker compose up -d
	@echo Grafana: http://localhost:3000/dashboards

stop:
	docker compose down -v
