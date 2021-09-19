use serde::Deserialize;
use reqwest::{self,Url};

#[derive(Deserialize, Debug)]
pub struct Me {
    pub id: String,
    pub login: String,
    pub name: String
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub login: String,
    pub avatar_url: String,
}

#[repr(C)]
pub struct CUser {
    pub login: *const libc::c_char,
    pub avatar_url: *const libc::c_char,
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

#[derive(Deserialize, Debug)]
pub struct Pipeline {
    pub id: String,
    pub project_slug: String,
    #[serde(default)]
    pub updated_at: String,
    pub number: i64,
    pub state: String,
    pub created_at: String,
    pub trigger: Trigger,
    pub vcs: VCS,
}

#[repr(C)]
pub struct CPipeline {
    pub id: *const libc::c_char,
    pub project_slug: *const libc::c_char,
    pub updated_at: *const libc::c_char,
    pub number: *const i64,
    pub state: *const libc::c_char,
    pub created_at: *const libc::c_char,
    pub trigger: CTrigger,
    pub vcs: CVCS,
}

#[derive(Deserialize, Debug)]
pub struct Trigger {
    #[serde(rename = "type")] 
    pub ttype: String,
    pub received_at: String,
    pub actor: User,
}

#[derive(Deserialize, Debug)]
pub struct VCS {
    pub provider_name: String,
    pub target_repository_url: String,
    #[serde(default)]
    pub branch: String,
    #[serde(default)]
    pub review_id: String,
    #[serde(default)]
    pub review_url: String,
    pub revision: String,
    #[serde(default)]
    pub tag: String,
    #[serde(default)]
    pub commit: Commit,
    pub origin_repository_url: String,
}

#[repr(C)]
pub struct CTrigger {
    pub ttype: *const libc::c_char,
    pub received_at: *const libc::c_char,
    pub actor: CUser,
}

#[repr(C)]
pub struct CVCS {
    pub provider_name: *const libc::c_char,
    pub target_repository_url: *const libc::c_char,
    pub branch: *const libc::c_char,
    pub review_id: *const libc::c_char,
    pub review_url: *const libc::c_char,
    pub revision: *const libc::c_char,
    pub tag: *const libc::c_char,
    pub commit: CCommit,
    pub origin_repository_url: *const libc::c_char,
}

#[derive(Deserialize, Debug, Default)]
pub struct Commit {
    pub subject: String,
    pub body: String,
}

#[repr(C)]
pub struct CCommit {
    pub subject: *const libc::c_char,
    pub body: *const libc::c_char,
}

pub struct Api {
    pub base_url: Url,
    pub api_key: String,
}

#[derive(Deserialize, Debug)]
struct PipelineResponse {
    next_page_token: Option<String>,
    items: Vec<Pipeline>
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

    pub fn pipelines(&self, slug: String) -> Result<Vec<Pipeline>, Box<dyn std::error::Error>> {
        let request_url = format!("{base}/v2/project/{project_slug}/pipeline",
            base=self.base_url, project_slug=slug);
        let client = reqwest::blocking::Client::new();
        let response = client
            .get(request_url)
            .header("Circle-Token", &self.api_key)
            .send()?;

        let pipeline_response: PipelineResponse = response.json()?;
        Ok(pipeline_response.items)
    }
}
