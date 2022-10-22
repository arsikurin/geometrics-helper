use std::process::exit;

use confy::ConfyError;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

use crate::types;

pub async fn obtain_token(login: String, password: String) -> Result<types::POSTLoginResp, Box<dyn std::error::Error>> {
    let body = reqwest::Client::new()
        .post("http://localhost:1323/api/v1/login")
        .header(CONTENT_TYPE, "application/json")
        .json(&types::POSTLoginReq {
            login,
            password,
        })
        .send()
        .await?
        .json()
        .await?;

    Ok(body)
}

pub fn get_token_from_conf(cfg: &Result<types::MyConfig, ConfyError>) -> &String {
    match cfg {
        Ok(types::MyConfig { token }) => {
            if token == "" {
                println!("Not authorized. Run auth");
                exit(0);
            } else {
                return token;
            }
        }
        Err(e) => {
            println!("ERROR: {:?}", e);
            exit(0);
        }
    }
}

pub async fn put_problem(token: &String, name: String, description: String, solutionBase64: String) -> Result<types::ProblemResp, Box<dyn std::error::Error>> {
    let body = reqwest::Client::new()
        .put("http://localhost:1323/api/v1/problems")
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, token)
        .json(&types::PUTProblemReq {
            name,
            description,
            solutionBase64,
        })
        .send()
        .await?
        .json()
        .await?;

    Ok(body)
}

pub async fn patch_problem(token: &String, id: u16, name: Option<String>, description: Option<String>, solutionBase64: Option<String>) -> Result<types::ProblemResp, Box<dyn std::error::Error>> {
    let body = reqwest::Client::new()
        .patch(format!("http://localhost:1323/api/v1/problems/{}", id))
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, token)
        .json(&types::PATCHProblemReq {
            name,
            description,
            solutionBase64,
        })
        .send()
        .await?
        .json()
        .await?;

    Ok(body)
}

pub async fn delete_problem(token: &String, id: u16) -> Result<types::ProblemResp, Box<dyn std::error::Error>> {
    let body = reqwest::Client::new()
        .delete(format!("http://localhost:1323/api/v1/problems/{}", id))
        .header(CONTENT_TYPE, "application/json")
        .header(AUTHORIZATION, token)
        .send()
        .await?
        .json()
        .await?;

    Ok(body)
}