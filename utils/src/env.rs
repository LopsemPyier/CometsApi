use lazy_static::lazy_static;

lazy_static! {
    pub static ref PASSWORD_SECRET_KEY: String = std::env::var("PASSWORD_SECRET_KEY").expect("Can't read PASSWORD_SECRET_KEY");
    pub static ref JWT_SECRET_KEY: String = std::env::var("JWT_SECRET_KEY").expect("Can't read JWT_SECRET_KEY");

}