use lib_tm::tm::Sysinfo as TmSysinfo;

/// State type that can be used in actix context
pub type WebState<'a> = actix_web::web::Data<&'a State>;

impl State {
    pub fn new_state() -> Self {
        State {
            sysinfo: Sysinfo {
                tm_sysinfo: TmSysinfo::new_all(),
            },
        }
    }
}

/// Root state type shared between all api
pub struct State {
    pub sysinfo: Sysinfo,
}

pub struct Sysinfo {
    pub tm_sysinfo: TmSysinfo,
}
