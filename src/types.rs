use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct MyConfig {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct POSTLoginResp {
    pub code: u16,
    pub status: String,
    pub message: Option<String>,
    pub token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct POSTLoginReq {
    pub login: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PUTProblemReq {
    pub name: String,
    pub description: String,
    pub solutionBase64: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct PATCHProblemReq {
    pub name: Option<String>,
    pub description: Option<String>,
    pub solutionBase64: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ProblemResp {
    pub code: u16,
    pub status: String,
    pub message: Option<String>,
    pub problem_id: Option<u16>,
}