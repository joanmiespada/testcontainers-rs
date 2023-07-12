use crate::{core::WaitFor, Image};
use std::collections::HashMap;

const NAME: &str = "localstack/localstack-pro";
const TAG: &str = "latest";

#[derive(Debug)]
pub struct LocalStackPro {
    env_vars: HashMap<String, String>,
    volumes: HashMap<String, String>,
    tag: String,
    ports: Vec<u16>
}


impl Image for LocalStackPro {
    type Args = ();

    fn name(&self) -> String {
        NAME.to_owned()
    }

    fn tag(&self) -> String {
        self.tag.to_owned()
    }

    fn ready_conditions(&self) -> Vec<WaitFor> {
        vec![WaitFor::message_on_stdout(format!("Ready."))]
    }

    fn env_vars(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.env_vars.iter())
    }

    fn expose_ports(&self) -> Vec<u16> {
       self.ports.clone() 
    }
    fn volumes(&self) -> Box<dyn Iterator<Item = (&String, &String)> + '_> {
        Box::new(self.volumes.iter())
    }
}


#[derive(Debug)]
pub struct LocalStackProBuilder {
    env_vars: HashMap<String, String>,
    tag: String,
    volumes: HashMap<String, String>,
    ports: Vec<u16>
}

impl LocalStackProBuilder {
    pub fn new(
    ) -> LocalStackProBuilder {
        LocalStackProBuilder {
            env_vars: HashMap::new(),
            tag: "".to_string(),
            volumes: HashMap::new(),
            ports: vec![]
        }
    }

    pub fn defaults(api_key:String) -> LocalStackProBuilder {
        let mut env_vars = HashMap::new();
        env_vars.insert("DEBUG".to_owned(), "1".to_owned());
        env_vars.insert(
            "DOCKER_HOST".to_owned(),
            "unix:///var/run/docker.sock".to_owned(),
        );
        env_vars.insert("LOCALSTACK_API_KEY".to_owned(), api_key);

        let mut volumes = HashMap::new();
        volumes.insert(
            "/var/run/docker.sock".to_owned(),
            "/var/run/docker.sock".to_owned(),
        );
        let mut ports: Vec<u16> = vec![]; //  [u16;3] = [4566,53,443];
        ports.push(4566);
        ports.push(53);
        ports.push(443);
        for i in 4510..4560 {
            ports.push(i);
        }
        LocalStackProBuilder {
            env_vars,
            tag: TAG.to_owned(),
            volumes,
            ports
        }

    }

    pub fn set_env_vars(&mut self, env_vars: HashMap<String, String>) -> &mut LocalStackProBuilder {
        self.env_vars = env_vars;
        self
    }
    
    pub fn set_volumes(&mut self, volumes: HashMap<String, String>) -> &mut LocalStackProBuilder {
        self.volumes = volumes;
        self
    }
    pub fn set_tag(&mut self, tag: String) -> &mut LocalStackProBuilder {
        self.tag = tag;
        self
    }
    pub fn set_ports(&mut self, ports: Vec<u16>) -> &mut LocalStackProBuilder {
        self.ports = ports;
        self
    }

    pub fn build(&self) -> LocalStackPro {
        LocalStackPro {
            env_vars: self.env_vars.clone(),
            volumes: self.volumes.clone(),
            tag: self.tag.clone(),
            ports: self.ports.clone()
        }
    }
}
