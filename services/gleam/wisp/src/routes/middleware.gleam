// Re-export as 1 module

import routes/middleware/chaos
import routes/middleware/log
import routes/middleware/rescue

pub const log_middleware = log.log_middleware

pub const chaos_middleware = chaos.chaos_middleware

pub const rescue_middleware = rescue.rescue_middleware
