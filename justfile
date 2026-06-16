default: build

# -- Docker --
build:
	docker rmi rust-axum-image:latest 2>/dev/null || true
	docker build -t rust-axum-image ./services/axum

build-site:
	docker rmi servers-website:latest 2>/dev/null || true
	docker build -t servers-website -f ui/Dockerfile ui

run:
	docker compose up -d
	@echo Website: http://localhost:8080/

stop:
	docker compose down -v

# -- Development --
rbuild:
	cargo build $(cargo metadata --format-version 1 | jq -r '.workspace_members as $wm | .packages[] | select([.id] | inside($wm)) | .name' | sed 's/^/-p /' | tr '\n' ' ')
