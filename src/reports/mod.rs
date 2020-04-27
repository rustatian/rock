use crate::profile::Profile;
use crate::Options;

mod graph;

pub struct Report {
    prof: Profile,
    total: i64,
    options: Options,
    format_value: Box<dyn Fn(i64, Options) -> String>,
}

impl Report {
    fn generate_raw_report(&self, p: Profile, opts: Options) -> Self {
        Report {
            prof: p,
            total: 1,
            options: opts,
            format_value: Box::new(|num: i64, o: Options| {
                let a = o.profile_path;
                a
            }),
        }
    }

    fn aa(self) {
        let ab = (self.format_value)(1, self.options);
    }
}
