use serde::Deserialize;
use reqwest::{self,Url};

#[derive(Deserialize, Debug)]
pub struct Me {
    pub id: String,
    pub login: String,
    pub name: String
}

#[repr(C)]
pub struct CMe {
    pub id: *const libc::c_char,
    pub login: *const libc::c_char,
    pub name: *const libc::c_char,
}

#[derive(Deserialize, Debug)]
pub struct Collaboration {
    pub vcs_type: String,
    pub name: String,
    pub avatar_url: String,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CCollaboration {
    pub vcs_type: *const libc::c_char,
    pub name: *const libc::c_char,
    pub avatar_url: *const libc::c_char,
}

#[derive(Deserialize, Debug)]
pub struct Project {
    pub vcs_url: String,
    pub following: bool,
    pub username: String,
    pub reponame: String,
    pub default_branch: String
}

#[repr(C)]
pub struct CProject {
    pub vcs_url: *const libc::c_char,
    pub following: bool,
    pub username: *const libc::c_char,
    pub reponame: *const libc::c_char,
    pub default_branch: *const libc::c_char
}

pub struct Api {
    pub base_url: Url,
    pub api_key: String,
}

impl Api {
    pub fn new(base_url: String, api_key: String) -> Api {
        let url = Url::parse(&base_url).unwrap();
        Api {  base_url: url, api_key, }
    }

    pub fn me(&self) -> Result<Me, Box<dyn std::error::Error>> {
        let request_url = format!("{base}/v2/me", base=self.base_url);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(request_url)
            .header("Circle-Token", &self.api_key)
            .send()?;

        let me: Me = response.json()?;
        Ok(me)
    }

    pub fn collaborations(&self) -> Result<Vec<Collaboration>, Box<dyn std::error::Error>> {
        let request_url = format!("{base}/v2/me/collaborations", base=self.base_url);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(request_url)
            .header("Circle-Token", &self.api_key)
            .send()?;

        let collaborations: Vec<Collaboration> = response.json()?;
        Ok(collaborations)
    }

    pub fn projects(&self) -> Result<Vec<Project>, Box<dyn std::error::Error>> {
        let request_url = format!("{base}/v1.1/projects", base=self.base_url);

        let client = reqwest::blocking::Client::new();
        let response = client
            .get(request_url)
            .header("Circle-Token", &self.api_key)
            .send()?;

        let projects: Vec<Project> = response.json()?;

        Ok(projects)
    }
}
