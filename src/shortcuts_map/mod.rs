use serde_with::{
    serde::{Deserialize, Serialize},
    serde_as,
};
use std::{collections::HashMap, iter::Peekable, ops::Deref, path::Path};

use crate::runtime::keyboard::KeyModes;
const SHORTCUT_DB: &str = "autobot_shortcuts.json";
//TODO : simply the shortcut map
#[derive(Deserialize, Serialize, Debug)]
pub struct ShortcutFile(pub String, pub bool);

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ShortcutMapValue {
    pub file: Option<ShortcutFile>,
    pub map: Option<Box<ShortcutMap>>,
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

#[serde_as]
#[derive(Deserialize, Serialize, Debug)]
pub struct ShortcutMap(pub HashMap<String, ShortcutMapValue>);
impl ShortcutMap {
    pub fn new() -> Self {
        Self(HashMap::new())
    }
    pub fn save_shortcut<'a>(
        &'a mut self,
        keys_modes: &'a mut Vec<KeyModes>,
        last_key: char,
        file: ShortcutFile,
    ) -> Result<(), String> {
        // sort key modes so that they can be parsed in the same order as well as two keys are presses at the same time
        // how to decide which one should be stored in the map first?
        keys_modes.sort();
        let mut keys: Vec<&str> = keys_modes.iter().map(|v| v.into()).collect();
        let keybind = &last_key.to_string();
        keys.push(&keybind);

        //TODO : check shortcut file validity exists?
        let mut keys_it = keys.iter_mut();
        let first = keys_it.next();

        if first.is_none() {
            return Err(format!("Can not save empty shortcut"));
        };
        let first = first.unwrap();
        ShortcutMap::insert_shortcut_recursive(
            self.get_or_create_key_map_mut(first),
            keys_it,
            file,
        )?;
        let res = serde_json::to_string_pretty(&self).unwrap();

        match std::fs::write(SHORTCUT_DB, res) {
            Err(err) => {
                return Err(format!("Err occured while saving : {}", err));
            }
            Ok(_) => (),
        }

        Ok(())
    }
    pub fn get_file_from_key_iter<'a, T: Iterator<Item = &'a str>>(
        &'a self,
        mut keys: Peekable<T>,
    ) -> Option<&ShortcutFile> {
        let key = keys.next()?;
        let map_val = self.0.get(key)?;
        let has_next = keys.peek();
        if has_next.is_none() {
            return map_val.file.as_ref();
        }
        map_val.map.as_ref()?.get_file_from_key_iter(keys)
    }
    fn insert_shortcut_recursive<'a>(
        shortcut_map_value: &mut ShortcutMapValue,
        mut keys: impl Iterator<Item = &'a mut &'a str> + 'a,
        file: ShortcutFile,
    ) -> Result<(), String> {
        let first = keys.next();

        if first.is_none() {
            //TODO: remove to string

            shortcut_map_value.file = Some(file);
            return Ok(());
        };

        let first = first.unwrap();
        //TODO: remove to string

        shortcut_map_value.insert_key(&first.to_string());

        let shortcut_map = shortcut_map_value.map.as_mut().unwrap().as_mut();

        Self::insert_shortcut_recursive(
            shortcut_map.0.get_mut(&first.to_string()).unwrap(),
            keys,
            file,
        )
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

impl Deref for ShortcutMap {
    type Target = HashMap<String, ShortcutMapValue>;
    fn deref(&self) -> &Self::Target {
        &self.0
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
