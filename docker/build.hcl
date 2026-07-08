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
    service = ["express", "fastify", "hono", "koa"]
  }
  name = service
  context    = "."
  dockerfile = "docker/Dockerfile.typescript-services"
  tags       = ["servers-${service}:latest"]
  args       = { SERVICE = service }
}
