mod register;
use serde_with::{
    serde::{Deserialize, Serialize},
    serde_as,
};
use std::{collections::HashMap, path::Path};
const SHORTCUT_DB: &str = "autobot_shortcuts.json";

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
struct ShortcutMapValue {
    file: Option<String>,
    map: Option<Box<ShortcutMap>>,
}

impl ShortcutMapValue {
    fn insert_key(&mut self, key: &String) {
        if self.map.is_none() {
            let mut m = ShortcutMap::new();
            m.0.insert(
                String::clone(key),
                ShortcutMapValue {
                    file: None,
                    map: None,
                },
            );
            self.map = Some(Box::new(m));
            return;
        }
        self.map.as_mut().unwrap().get_or_create_key_map_mut(key);
    }
}

fn insert_shortcut<'a>(
    shortcut_map_value: &mut ShortcutMapValue,
    mut keys: impl Iterator<Item = &'a mut &'a str> + 'a,
    file: &str,
) -> Result<(), String> {
    let first = keys.next();

    if first.is_none() {
        //TODO: remove to string
        shortcut_map_value.file = Some(file.to_string());
        return Ok(());
    };

    let first = first.unwrap();
    //TODO: remove to string

    shortcut_map_value.insert_key(&first.to_string());

    let shortcut_map = shortcut_map_value.map.as_mut().unwrap().as_mut();

    insert_shortcut(
        shortcut_map.0.get_mut(&first.to_string()).unwrap(),
        keys,
        file,
    )
}

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ShortcutMap(HashMap<String, ShortcutMapValue>);
impl ShortcutMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn save_shortcut<'a>(
        &'a mut self,
        keys: &'a mut Vec<&'a str>,
        file: &str,
    ) -> Result<(), String> {
        //TODO : check shortcut file validity exists?
        let mut keys_it = keys.iter_mut();
        let first = keys_it.next();

        if first.is_none() {
            return Err(format!("Can not save empty shortcut"));
        };
        let first = first.unwrap();
        insert_shortcut(self.get_or_create_key_map_mut(first), keys_it, file)?;
        println!("{:?}", self);
        let res = serde_json::to_string_pretty(&self).unwrap();
        println!("json : {}", res);

        match std::fs::write(SHORTCUT_DB, res) {
            Err(err) => {
                return Err(format!("Err occured while saving : {}", err));
            }
            Ok(_) => (),
        }

        Ok(())
    }
    fn get_or_create_key_map_mut(&mut self, key: &str) -> &mut ShortcutMapValue {
        if !self.0.contains_key(key) {
            self.0.insert(
                key.to_string(),
                ShortcutMapValue {
                    file: None,
                    map: None,
                },
            );
        }
        self.0.get_mut(key).unwrap()
    }
}

pub fn read_shortcuts() -> ShortcutMap {
    let shortcut_db_path = Path::new(SHORTCUT_DB);

    if !std::path::Path::exists(&shortcut_db_path) {
        return ShortcutMap::new();
    }

    let file = std::fs::read_to_string(shortcut_db_path)
        .expect(&format!("Error opening {}", shortcut_db_path.display(),));

    let shortcut_map = serde_json::from_str::<ShortcutMap>(&file).expect(&format!(
        "Couldnt deserealize {}",
        shortcut_db_path.display()
    ));
    shortcut_map
}
