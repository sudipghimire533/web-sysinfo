/// State type that can be used in actix context
pub type WebState<'a> = actix_web::web::Data<&'a State>;

/// Root state type shared between all api
pub struct State {
    pub debug: bool,
}

impl State {
    pub fn empty_sysinfo(&self) -> lib_tm::tm::Sysinfo {
        lib_tm::tm::new_sysinfo()
    }
}

impl Default for State {
    fn default() -> Self {
        Self { debug: true }
    }
}
