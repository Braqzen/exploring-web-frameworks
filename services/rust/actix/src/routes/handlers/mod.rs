mod delete;
mod get;
mod invalid_method;
mod invalid_path;
mod patch;
mod post;
mod put;

pub use delete::delete_handler;
pub use get::get_handler;
pub use invalid_method::invalid_method_handler;
pub use invalid_path::invalid_path_handler;
pub use patch::patch_handler;
pub use post::post_handler;
pub use put::put_handler;
