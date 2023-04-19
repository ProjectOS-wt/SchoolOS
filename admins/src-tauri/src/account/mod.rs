use serde::{Deserialize, Serialize};
use tauri::Runtime;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountDetails {
    pub token: String,
    pub cached_name: Option<String>,
    pub cached_avatar_url: Option<String>,
}

impl AccountDetails {
    pub fn new(token: String) -> Self {
        Self {
            token,
            cached_name: None,
            cached_avatar_url: None,
        }
    }
}

#[tauri::command]
pub async fn login<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>) {
	// Create a new window
    // Use Oauth2 to get the token
    // Save the token in the storage

	//Get oauth url from the oauth2 server using request
	
	/* let client = reqwest::Client::new(); */

	/* let oauthurl_res = match client
        .get("")
        .send()
        .await{
			Ok(r) => r,
			Err(e) => {
				println!("Error getting URL: {}", e);
				return;
			}
		};

        let oauth_url = match oauthurl_res
            .text()
            .await{
				Ok(r) => r,
				Err(e) => {
					println!("Error getting URL: {}", e);
					return;
				}
			};
	
	println!("Oauth URL: {}", oauth_url); */

    let window = match tauri::WindowBuilder::new(
        &app,
        "oauth2signin",
        tauri::WindowUrl::External("oauthurl".parse().unwrap()),
    )
    .build(){
		Ok(w) => w,
		Err(e) => {
			println!("Error creating window: {}", e);
			return;
		}
	};

}

