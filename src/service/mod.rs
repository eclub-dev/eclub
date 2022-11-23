#[allow(missing_docs)]
pub mod user;
#[allow(missing_docs)]
pub mod email;
#[allow(missing_docs)]
pub mod tag;
#[allow(missing_docs)]
pub mod category;

pub use self::user::UserService;
pub use self::email::EmailService;
pub use self::tag::TagService;
pub use self::category::CategoryService;
