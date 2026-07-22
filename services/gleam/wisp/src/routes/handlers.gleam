// Re-export as 1 module

import routes/handlers/delete
import routes/handlers/get
import routes/handlers/patch
import routes/handlers/post
import routes/handlers/put

pub const get_handler = get.get_handler

pub const post_handler = post.post_handler

pub const patch_handler = patch.patch_handler

pub const put_handler = put.put_handler

pub const delete_handler = delete.delete_handler
