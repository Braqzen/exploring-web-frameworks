// Re-export as 1 module

import routes/handlers/get
import routes/handlers/post

pub const get_handler = get.get_handler

pub const post_handler = post.post_handler
