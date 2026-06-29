mod delete;
mod get;
mod patch;
mod post;
mod put;

pub use delete::delete_handler;
pub use get::get_handler;
pub use patch::patch_handler;
pub use post::post_handler;
pub use put::put_handler;
