from tornado.web import HTTPError

from routes.handlers.base import BaseHandler


class NotFoundHandler(BaseHandler):
    async def prepare(self) -> None:
        await super().prepare()
        raise HTTPError(404)
