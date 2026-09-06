use alloc::string::{String, ToString};
use alloc::vec::Vec;
use alloc::collections::BTreeMap;
use spin::Mutex;
use lazy_static::lazy_static;

pub enum Node {
    File(Vec<u8>),
    Dir(BTreeMap<String, Node>),
}

pub struct Vfs {
    root: Node,
}

impl Vfs {
    pub fn new() -> Self {
        Vfs { root: Node::Dir(BTreeMap::new()) }
    }

    pub fn resolve(cwd: &str, path: &str) -> String {
        let mut parts: Vec<String> = if path.starts_with('/') {
            Vec::new()
        } else {
            cwd.split('/').filter(|s| !s.is_empty()).map(|s| s.to_string()).collect()
        };

        for part in path.split('/') {
            match part {
                "" | "." => {}
                ".." => { parts.pop(); }
                p => parts.push(p.to_string()),
            }
        }

        let mut result = String::from("/");
        result.push_str(&parts.join("/"));
        result
    }

    fn split(path: &str) -> Vec<&str> {
        path.split('/').filter(|s| !s.is_empty()).collect()
    }

    fn get_dir_mut<'a>(node: &'a mut Node, parts: &[&str]) -> Result<&'a mut Node, &'static str> {
        if parts.is_empty() {
            return Ok(node);
        }
        match node {
            Node::Dir(map) => {
                let next = map.get_mut(parts[0]).ok_or("path not found")?;
                Vfs::get_dir_mut(next, &parts[1..])
            }
            Node::File(_) => Err("not a directory"),
        }
    }

    fn get_node<'a>(node: &'a Node, parts: &[&str]) -> Result<&'a Node, &'static str> {
        if parts.is_empty() {
            return Ok(node);
        }
        match node {
            Node::Dir(map) => {
                let next = map.get(parts[0]).ok_or("path not found")?;
                Vfs::get_node(next, &parts[1..])
            }
            Node::File(_) => Err("path not found"),
        }
    }

    pub fn mkdir(&mut self, path: &str) -> Result<(), &'static str> {
        let parts = Vfs::split(path);
        if parts.is_empty() {
            return Ok(());
        }
        let (name, dirs) = parts.split_last().unwrap();
        let dir = Vfs::get_dir_mut(&mut self.root, dirs)?;
        match dir {
            Node::Dir(map) => {
                map.entry(name.to_string()).or_insert_with(|| Node::Dir(BTreeMap::new()));
                Ok(())
            }
            Node::File(_) => Err("not a directory"),
        }
    }

    pub fn touch(&mut self, path: &str) -> Result<(), &'static str> {
        self.write(path, Vec::new())
    }

    pub fn write(&mut self, path: &str, content: Vec<u8>) -> Result<(), &'static str> {
        let parts = Vfs::split(path);
        if parts.is_empty() {
            return Err("invalid path");
        }
        let (name, dirs) = parts.split_last().unwrap();
        let dir = Vfs::get_dir_mut(&mut self.root, dirs)?;
        match dir {
            Node::Dir(map) => {
                map.insert(name.to_string(), Node::File(content));
                Ok(())
            }
            Node::File(_) => Err("not a directory"),
        }
    }

    pub fn read(&self, path: &str) -> Result<Vec<u8>, &'static str> {
        let parts = Vfs::split(path);
        match Vfs::get_node(&self.root, &parts)? {
            Node::File(data) => Ok(data.clone()),
            Node::Dir(_) => Err("is a directory"),
        }
    }

    pub fn ls(&self, path: &str) -> Result<Vec<String>, &'static str> {
        let parts = Vfs::split(path);
        match Vfs::get_node(&self.root, &parts)? {
            Node::Dir(map) => Ok(map.keys().cloned().collect()),
            Node::File(_) => Err("not a directory"),
        }
    }

    pub fn is_dir(&self, path: &str) -> bool {
        let parts = Vfs::split(path);
        match Vfs::get_node(&self.root, &parts) {
            Ok(Node::Dir(_)) => true,
            _ => false,
        }
    }

    pub fn remove(&mut self, path: &str) -> Result<(), &'static str> {
        let parts = Vfs::split(path);
        if parts.is_empty() {
            return Err("cannot remove root directory");
        }
        let (name, dirs) = parts.split_last().unwrap();
        let dir = Vfs::get_dir_mut(&mut self.root, dirs)?;
        match dir {
            Node::Dir(map) => {
                map.remove(*name).ok_or("not found")?;
                Ok(())
            }
            Node::File(_) => Err("not a directory"),
        }
    }
}

lazy_static! {
    pub static ref VFS: Mutex<Vfs> = Mutex::new(Vfs::new());
}
