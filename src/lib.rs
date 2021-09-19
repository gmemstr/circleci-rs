mod api_v2;

use std::ffi::CString;
use std::ffi::CStr;
use libc::c_int;
use crate::api_v2::CTrigger;
use crate::api_v2::CVCS;
use crate::api_v2::{Collaboration, Me, Api, CCollaboration, CProject, CMe, CPipeline, CUser, CCommit};
use std::os::raw::c_char;
use std::{ptr, mem};

#[no_mangle]
pub extern "C" fn circleci_api(base_url: *const c_char, api_key: *const c_char) -> *mut Api {
    let burl = unsafe { CStr::from_ptr(base_url) };
    let akey = unsafe { CStr::from_ptr(api_key) };

    let url = match burl.to_str() {
        Ok(s) => s,
        Err(_) => panic!("Couldn't load")
    };
    let key = match akey.to_str() {
        Ok(s) => s,
        Err(_) => panic!("Couldn't load")
    };
    Box::into_raw(Box::new(api_v2::Api::new(url.to_string(), key.to_string())))
}

#[no_mangle]
pub unsafe extern "C" fn circleci_api_me(api: *const api_v2::Api) -> *mut CMe {
    let api = &*api;
    let res = match api.me() {
        Ok(r) => r,
        Err(_) => {
            let def_id = "none".to_string();
            let def_login = "none".to_string();
            let def_name = "none".to_string();
            Me{id: def_id, login: def_login, name: def_name}
        },
    };

    let id = CString::new(res.id).expect("Err: CString::new()").into_raw();
    let login = CString::new(res.login).expect("Err: CString::new()").into_raw();
    let name = CString::new(res.name).expect("Err: CString::new()").into_raw();

    Box::into_raw(Box::new(CMe{id, login, name}))
}

#[no_mangle]
pub unsafe extern "C" fn circleci_api_collaborations(api: *const api_v2::Api, outlen: *mut c_int) -> *mut CCollaboration {
    let api = &*api;
    let mut res = match api.collaborations() {
        Ok(r) => r,
        Err(_) => Vec::new()
    };

    if res.len() == 0 {
        ptr::write(outlen, 0 as c_int);
        return Vec::new().as_mut_ptr()
    }

    let mut ccolabs: Vec<CCollaboration> = res.drain(1..).map(|x| {
        let vcs_type = CString::new(x.vcs_type).expect("Err: CString::new()").into_raw();
        let name = CString::new(x.name).expect("Err: CString::new()").into_raw();
        let avatar_url = CString::new(x.avatar_url).expect("Err: CString::new()").into_raw();
        CCollaboration{vcs_type, name, avatar_url}
    }).collect();

    ccolabs.shrink_to_fit();
    assert!(ccolabs.len() == ccolabs.capacity());

    let len = ccolabs.len();
    let ptr = ccolabs.as_mut_ptr();
    mem::forget(ccolabs);
    ptr::write(outlen, len as c_int);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn circleci_api_projects(api: *const api_v2::Api, outlen: *mut c_int) -> *mut CProject {
    let api = &*api;
    let mut res = match api.projects() {
        Ok(r) => r,
        Err(_) => Vec::new(),
    };

    if res.len() == 0 {
        ptr::write(outlen, 0 as c_int);
        return Vec::new().as_mut_ptr()
    }

    let mut cprojects: Vec<CProject> = res.drain(1..).map(|x| {
        let vcs_url = CString::new(x.vcs_url).expect("Err: CString::new()").into_raw();
        let username = CString::new(x.username).expect("Err: CString::new()").into_raw();
        let reponame = CString::new(x.reponame).expect("Err: CString::new()").into_raw();
        let default_branch = CString::new(x.default_branch).expect("Err: CString::new()").into_raw();
        let vcs_type = CString::new(x.vcs_type).expect("Err: CString::new()").into_raw();
        CProject{vcs_url, following: x.following, username, reponame, default_branch, vcs_type}
    }).collect();

    cprojects.shrink_to_fit();
    assert!(cprojects.len() == cprojects.capacity());

    let len = cprojects.len();
    let ptr = cprojects.as_mut_ptr();
    mem::forget(cprojects);
    ptr::write(outlen, len as c_int);
    ptr
}

#[no_mangle]
pub unsafe extern "C" fn circleci_api_project_pipelines(api: *const api_v2::Api, slug: *const libc::c_char, outlen: *mut c_int) -> *mut CPipeline {
    let api = &*api;
    let rslug = CStr::from_ptr(slug).to_str().unwrap();
    let rrslug = String::from(rslug);
    let mut res = match api.pipelines(rrslug) {
        Ok(r) => r,
        Err(_) => Vec::new()
    };

    if res.len() == 0 {
        ptr::write(outlen, 0 as c_int);
        return Vec::new().as_mut_ptr()
    }

    let mut cpipelines: Vec<CPipeline> = res.drain(1..).map(|x| {
        let id = CString::new(x.id).expect("Err: CString::new()").into_raw();
        let project_slug = CString::new(x.project_slug).expect("Err: CString::new()").into_raw();
        let updated_at = CString::new(x.updated_at).expect("Err: CString::new()").into_raw();
        let number = x.number;
        let state = CString::new(x.state).expect("Err: CString::new()").into_raw();
        let created_at = CString::new(x.created_at).expect("Err: CString::new()").into_raw();
        let trigger = CTrigger{
            ttype:  CString::new(x.trigger.ttype).expect("Err: CString::new()").into_raw(),
            received_at: CString::new(x.trigger.received_at).expect("Err: CString::new()").into_raw(),
            actor: CUser {
                login: CString::new(x.trigger.actor.login).expect("Err: CString::new()").into_raw(),
                avatar_url: CString::new(x.trigger.actor.avatar_url).expect("Err: CString::new()").into_raw(),

            }
        };

        let vcs = CVCS {
            provider_name: CString::new(x.vcs.provider_name).expect("Err: CString::new()").into_raw(),
            target_repository_url: CString::new(x.vcs.target_repository_url).expect("Err: CString::new()").into_raw(),
            branch: CString::new(x.vcs.branch).expect("Err: CString::new()").into_raw(),
            review_id: CString::new(x.vcs.review_id).expect("Err: CString::new()").into_raw(),
            review_url: CString::new(x.vcs.review_url).expect("Err: CString::new()").into_raw(),
            revision: CString::new(x.vcs.revision).expect("Err: CString::new()").into_raw(),
            tag: CString::new(x.vcs.tag).expect("Err: CString::new()").into_raw(),
            commit: CCommit {
                subject: CString::new(x.vcs.commit.subject).expect("Err: CString::new()").into_raw(),
                body: CString::new(x.vcs.commit.body).expect("Err: CString::new()").into_raw(),
            },
            origin_repository_url: CString::new(x.vcs.origin_repository_url).expect("Err: CString::new()").into_raw(),
        };

        CPipeline{ id, project_slug, updated_at, number: &number, state, created_at, trigger, vcs }
    }).collect();

    cpipelines.shrink_to_fit();
    assert!(cpipelines.len() == cpipelines.capacity());

    let len = cpipelines.len();
    let ptr = cpipelines.as_mut_ptr();
    mem::forget(cpipelines);
    ptr::write(outlen, len as c_int);
    ptr
}

#[cfg(test)]
mod tests {
    use crate::api_v2::{self, Collaboration, Pipeline, Project};
    use std::env;

    #[test]
    fn me() {
        let api_key = match env::var("CIRCLE_TOKEN") {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let v2 = api_v2::Api::new( 
            "https://circleci.com/api".to_string(), 
            api_key, 
        );
        match v2.me() {
            Ok(me) => {
                assert_eq!(me.login, "gmemstr".to_string())
            },
            Err(_) => {
                println!("Did not get expected result from endpoint!"); assert!(false)
            }
        };
    }

    #[test]
    fn collaborations() {
        let api_key = match env::var("CIRCLE_TOKEN") {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let v2 = api_v2::Api::new( 
            "https://circleci.com/api".to_string(), 
            api_key, 
        );
        let collabs: Vec<Collaboration> = match v2.collaborations() {
            Ok(c) => c,
            Err(_) => {
                println!("Did not get expected result from endpoint!"); 
                panic!("");
            }
        };
        let mut has = false;
        for c in &collabs {
            if c.name == "gmemstr" { has = true; }
        }

        assert_eq!(has, true)
    }

    #[test]
    fn projects() {
        let api_key = match env::var("CIRCLE_TOKEN") {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let api = api_v2::Api::new(
            "https://circleci.com/api".to_string(),
            api_key,
        );
        let projs: Vec<Project> = match api.projects() {
            Ok(c) => c,
            Err(_) => {
                println!("Did not get expected result from endpoint!");
                panic!("");
            }
        };
        let mut has = false;
        for p in &projs {
            if p.reponame == "blog.gabrielsimmer.com" { has = true; }
        }

        assert_eq!(has, true)
    }

    #[test]
    fn pipelines() {
        let api_key = match env::var("CIRCLE_TOKEN") {
            Ok(val) => val,
            Err(_) => "".to_string(),
        };

        let api = api_v2::Api::new(
            "https://circleci.com/api".to_string(),
            api_key,
        );
        let pipelines: Vec<Pipeline> = match api.pipelines("gh/gmemstr/circleci-rs".to_string()) {
            Ok(c) => c,
            Err(e) => {
                println!("Did not get expected result from endpoint!");
                panic!("{}", e);
            }
        };

        assert_ne!(pipelines.len(), 0)
    }
}
