use crate::errors::RockError;
use crate::profile::Profile;
use crate::Options;
use std::collections::HashMap;

mod graph;

pub struct Report<'rep> {
    prof: &'rep Profile,
    total: i64,
    options: &'rep Options,
    //format_value: Box<dyn Fn(i64, Options) -> &'a str>,
}

impl<'rep> Report<'rep> {
    pub fn new(p: &'rep Profile, opts: &'rep Options) -> Self {
        Report {
            prof: p,
            options: opts,
            total: 0,
        }
    }

    fn generate_report(&self) {
        let mut rptr = self.generate_raw_report(self.prof, self.options);
        let g = graph::Graph::new(self.prof);
    }

    fn generate_raw_report(&self, p: &'rep Profile, opts: &'rep Options) -> Self {
        let num_label_units = self.identify_num_label_units(opts, p);


        Report {
            prof: p,
            total: 1,
            options: opts,
            //format_value: Box::new(|_num: i64, o: Options| o.profile_path),
        }
    }

    // identifyNumLabelUnits returns a map of numeric label keys to the units
    // associated with those keys.
    // ui passed here to print errors in interactive mode
    #[inline(always)]
    fn identify_num_label_units(
        &self,
        _opts: &'_ Options,
        profile: &'_ Profile,
    ) -> Result<HashMap<String, String>, RockError> {
        match profile.num_label_units() {
            Ok(res) => Ok(res.0),
            Err(err) => Err(err),
        }
    }

    // fn aa(self) {
    //     let ab = (self.format_value)(1, self.options);
    // }
}
