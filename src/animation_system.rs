use std::path::Path;

use regex::Regex;

#[derive(Debug)]
pub struct AnimationSystem {
    pub name: String,
    pub animations: Vec<Animation>,
    pub length: u32,
    pub current_animation: u32,
}

impl AnimationSystem {
    pub fn load_from(path: impl Into<String>) -> AnimationSystem {
        // check the path exists and whether it is a directory
        let binding = path.into();
        let path = Path::new(&binding);
        if !path.exists() || !path.is_dir() {
            panic!("Path does not exist or is not a directory");
        }

        // get the name of the directory the name is the end of the path
        let name = path.file_name().unwrap().to_str().unwrap();
        let mut length: u32 = 0;
        let mut animations = Vec::new();
        for entry in path.read_dir().unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                continue;
            }
            let file_name = path.file_name().unwrap().to_str().unwrap();
            let file_path = path.parent().unwrap().to_str().unwrap();
            println!("file_path: {file_path}");
            println!("file_name: {file_name}");
            let animation = Animation {
                file_name: file_name.to_string(),
                file_path: file_path.to_string(),
            };
            animations.push(animation);
            length += 1;
        }
        animations.sort_by(|a, b| {
            // use regex to get the number from the file name
            let re = Regex::new(r"\d+").unwrap();
            let a = re
                .find(&a.file_name)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            let b = re
                .find(&b.file_name)
                .unwrap()
                .as_str()
                .parse::<u32>()
                .unwrap();
            a.cmp(&b)
        });

        AnimationSystem {
            name: name.to_string(),
            animations,
            length,
            current_animation: 0,
        }
    }

    pub fn next_animation(&mut self) -> Option<&Animation> {
        if self.current_animation == self.length {
            return None;
        }
        self.current_animation += 1;
        Some(&self.animations[self.current_animation as usize - 1])
    }
}

#[derive(Debug)]
pub struct Animation {
    file_name: String,
    // this field not used yet
    #[allow(dead_code)]
    file_path: String,
}

impl From<&str> for Animation {
    fn from(path: &str) -> Self {
        let file = Path::new(path);
        if !file.exists() {
            panic!("File does not exist");
        }
        let file_name = file.file_name().unwrap().to_str().unwrap();
        let file_path = file.parent().unwrap().to_str().unwrap();

        // println!("file_path: {}", file_path);
        // println!("file_name: {}", file_name);
        Self {
            file_name: file_name.to_string(),
            file_path: file_path.to_string(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_animation_from() {
        let animation = Animation::from("img/Stocking/shime1.png");
        assert_eq!(animation.file_path, "img/Stocking");
        assert_eq!(animation.file_name, "shime1.png");
    }

    #[test]
    fn test_animation_system_load() {
        let animation_system = AnimationSystem::load_from("img/Stocking");
        assert_eq!(animation_system.name, "Stocking");
        assert_eq!(animation_system.length, 43);
        assert_eq!(animation_system.current_animation, 0);
        assert_eq!(animation_system.animations[0].file_name, "shime1.png");
        assert_eq!(animation_system.animations[1].file_name, "shime2.png");
        assert_eq!(animation_system.animations[2].file_name, "shime3.png");
        assert_eq!(animation_system.animations[3].file_name, "shime4.png");
    }

    #[test]
    fn test_animation_system_next_animation() {
        let mut animation_system = AnimationSystem::load_from("img/Stocking");
        let animation = animation_system.next_animation().unwrap();
        assert_eq!(animation.file_name, "shime1.png");
        assert_eq!(animation_system.current_animation, 1);
        let animation = animation_system.next_animation().unwrap();
        assert_eq!(animation.file_name, "shime2.png");
        assert_eq!(animation_system.current_animation, 2);
        let animation = animation_system.next_animation().unwrap();
        assert_eq!(animation.file_name, "shime3.png");
    }
}
