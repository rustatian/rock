use std::collections::HashMap;

#[derive(Clone)]
pub struct Server {}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Profiles {
    Heap = 0,
    Goroutine = 1,
    ThreadCreate = 2,
    Block = 3,
    Mutex = 4,
    // TIMED
    Cpu = 5,
    Trace = 6,
}

impl Server {
    /// `duration` in seconds
    pub fn new_timed_profiles(duration: &std::time::Duration) -> Self {
        let mut map: HashMap<Profiles, String> = HashMap::new();
        map.insert(
            Profiles::Cpu,
            format!("/debug/pprof/profile?seconds={}", duration.as_secs()),
        );
        map.insert(
            Profiles::Trace,
            format!("/debug/pprof/trace?seconds={}", duration.as_secs()),
        );

        Server {}
    }
    pub fn new(_name: &str) -> Self {
        let mut map: HashMap<Profiles, &str> = HashMap::new();

        // heap profile, memory
        map.insert(Profiles::Heap, "/debug/pprof/heap");
        map.insert(Profiles::Goroutine, "/debug/pprof/goroutine");
        map.insert(Profiles::ThreadCreate, "/debug/pprof/threadcreate");
        map.insert(Profiles::Block, "/debug/pprof/block");
        map.insert(Profiles::Mutex, "/debug/pprof/mutex");

        Server {}
    }

    // for the future state machine
    // this function will take profile type
    // and return struct with binary data --> Vec<u8> and identifier of what is it that data
}
