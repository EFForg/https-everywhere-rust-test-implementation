use https_everywhere_lib_core::{updater::{UpdateChannels, Updater}, RuleSets, Rewriter, Storage, Settings};
use std::collections::HashMap;
use std::fs;
use std::thread;
use std::sync::{Arc, Mutex};

#[derive(Default)]
pub struct WorkingTempStorage {
    ints: HashMap<String, usize>,
    bools: HashMap<String, bool>,
    strings: HashMap<String, String>,
}

impl Storage for WorkingTempStorage {
    fn get_int(&self, key: String) -> Option<usize> {
        match self.ints.get(&key) {
            Some(value) => Some(*value),
            None => None
        }
    }

    fn get_bool(&self, key: String) -> Option<bool> {
        match self.bools.get(&key) {
            Some(value) => Some(*value),
            None => None
        }
    }

    fn get_string(&self, key: String) -> Option<String> {
        match self.strings.get(&key) {
            Some(value) => Some(value.clone()),
            None => None
        }
    }

    fn set_int(&mut self, key: String, value: usize) {
        self.ints.insert(key, value);
    }

    fn set_bool(&mut self, key: String, value: bool) {
        self.bools.insert(key, value);
    }

    fn set_string(&mut self, key: String, value: String) {
        self.strings.insert(key, value);
    }
}

fn main() {
    simple_logger::init().unwrap();
    let s = Arc::new(Mutex::new(WorkingTempStorage::default()));
    let s2 = Arc::clone(&s);
    let s3 = Arc::clone(&s);

    let mut settings = Settings::new(s3);
    settings.set_ease_mode_enabled(true);

    let mut rs = RuleSets::new();
    let rulesets_string = fs::read_to_string("lib-core/tests/mock_rulesets.json").unwrap();
    rs.add_all_from_json_string(&rulesets_string, true, &HashMap::new(), &None);

    let update_channels_string = fs::read_to_string("lib-core/tests/update_channels.json").unwrap();
    let ucs = UpdateChannels::from(&update_channels_string[..]);

    let rs_threadsafe = Arc::new(Mutex::new(rs));
    let rs_threadsafe2 = Arc::clone(&rs_threadsafe);

    let rw = Rewriter::new(rs_threadsafe2, s2);
    let ra = rw.rewrite_url(&"http://1.usa.gov/".to_string());
    println!("{:?}", ra);

    let t = thread::spawn(move || {
        let mut updater = Updater::new(rs_threadsafe, &ucs, s, None, 15);
        updater.apply_stored_rulesets();
        updater.perform_check();
    });

    t.join().unwrap();

    let ra = rw.rewrite_url(&"http://1.usa.gov/".to_string());
    println!("{:?}", ra);
}
