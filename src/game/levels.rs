pub enum LevelName {
    Introduction,
    Test,
    Woua,
}
use std::collections::HashMap;

lazy_static! {
    pub static ref LEVELS: HashMap<LevelName, Level> = {
        let mut t = HashMap::new();
        t.insert(
            Introduction,
            Level {
                name: "Introduction",
                description: "nice",
            },
        );
        t.insert(
            Test,
            Level {
                name: "Test",
                description: "nice",
            },
        );
        t.insert(
            Woua,
            Level {
                name: "Woua",
                description: "nice",
            },
        );
        t
    };
}

pub struct Level {
    name: &str,
    description: &str,
}
