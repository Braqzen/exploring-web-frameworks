target "generator" {
  context    = "."
  dockerfile = "docker/Dockerfile.generator"
  tags       = ["servers-generator:latest"]
}

target "rust-services" {
  matrix = {
    service = ["actix", "axum", "poem", "rocket", "salvo", "warp"]
  }
  name = service
  context    = "."
  dockerfile = "docker/Dockerfile.rust-services"
  tags       = ["servers-${service}:latest"]
  args       = { SERVICE = service }
}

target "typescript-services" {
  matrix = {
    service = ["express", "fastify", "hono", "koa", "elysia"]
  }
  name = service
  context    = "."
  dockerfile = "docker/Dockerfile.typescript-services"
  tags       = ["servers-${service}:latest"]
  args       = { SERVICE = service }
}

target "python-services" {
  matrix = {
    service = ["flask", "fastapi", "sanic", "quart", "django", "tornado", "starlette"]
  }
  name = service
  context    = "."
  dockerfile = "docker/Dockerfile.python-services"
  tags       = ["servers-${service}:latest"]
  args       = { SERVICE = service }
}

target "go-services" {
  matrix = {
    service = ["gin", "chi", "fiber", "echo"]
  }
  name = service
  context    = "."
  dockerfile = "docker/Dockerfile.go-services"
  tags       = ["servers-${service}:latest"]
  args       = { SERVICE = service }
}
