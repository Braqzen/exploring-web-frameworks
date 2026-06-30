# servers

Grafana: http://localhost:3000/dashboards

TODO:

- update all rust docker files to use the same version
- error handling for bad requests / rejections
- insert random error rate into generator (configurable)
  - unknown operations e.g. filter
  - wrong method (HEAD, etc.)
  - oversize body
  - bad UUID
  - malformed JSON
- insert random error rate into each api (configurable)
  - apis may just fail randomly so perhaps just return an error, sleep or something
- consider injecting langtype into telemetry instead of just service name
- Consider APIs
  - Python: fastapi, django, flask, starlette, sanic, quart, tornado
  - Go : gin, chi, echo, fiber, std/net
  - Ts : express, fastify, koa, hono, nestjs
  - Zig : zap, http.zig
  - C++ : drogon, crow, boost-beast, pistache, Restinio, CppCMS
  - Odin: ?
- Lookup
  - Python: PyPi
  - Ts: pnpm
  - Go: pkg.go.dev
  - Zig: Github
  - C++: vcpkg
