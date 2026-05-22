mod app;
mod kind;
mod spec;

pub use app::AppError;
pub use kind::ErrorKind;
pub use spec::{ErrorSpec, ErrorSpecExt, OptionAppExt};

pub type AppResult<T> = Result<T, AppError>;
