from fastapi import FastAPI

from routes.handlers import (
    post_handler,
    get_handler,
    delete_handler,
    put_handler,
    patch_handler,
)


def register_routes(app: FastAPI):
    app.post("/", status_code=201)(post_handler)
    app.get("/{id}", status_code=200)(get_handler)
    app.delete("/{id}", status_code=204)(delete_handler)
    app.put("/{id}", status_code=200)(put_handler)
    app.patch("/{id}", status_code=200)(patch_handler)
