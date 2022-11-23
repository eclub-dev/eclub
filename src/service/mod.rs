#[allow(missing_docs)]
pub mod category;
#[allow(missing_docs)]
pub mod email;
#[allow(missing_docs)]
pub mod follow;
#[allow(missing_docs)]
pub mod profile;
#[allow(missing_docs)]
pub mod tag;
#[allow(missing_docs)]
pub mod user;

pub use self::category::CategoryService;
pub use self::email::EmailService;
pub use self::follow::FollowService;
pub use self::tag::TagService;
pub use self::user::UserService;
