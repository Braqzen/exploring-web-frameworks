from django.urls import path
from django.http import HttpResponse

from routes.handlers import (
    post_handler,
    get_handler,
    delete_handler,
    put_handler,
    patch_handler,
    invalid_path_handler,
    internal_error_handler,
    large_payload_handler,
    invalid_method_handler,
)


async def id_handler(request, id: str) -> HttpResponse:
    if request.method == "GET":
        return await get_handler(request, id)
    elif request.method == "DELETE":
        return await delete_handler(request, id)
    elif request.method == "PUT":
        return await put_handler(request, id)
    elif request.method == "PATCH":
        return await patch_handler(request, id)
    return invalid_method_handler(request, None)


async def post_route(request) -> HttpResponse:
    if request.method == "POST":
        return await post_handler(request)
    return invalid_method_handler(request, None)


urlpatterns = [
    path("", post_route),
    path("<str:id>", id_handler),
]

handler404 = invalid_path_handler
handler500 = internal_error_handler
handler413 = large_payload_handler
