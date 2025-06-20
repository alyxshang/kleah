/*
Kleah by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the "get"
/// function to fetch
/// an HTML page.
use reqwest::get;

/// Importing the "Response"
/// structure for explicit 
/// typing.
use reqwest::Response;

/// Importing this crate's
/// structure for catching
/// and handling errors.
use super::err::KleahErr;

/// This function attempts
/// to fetch the HTML page 
/// stored at the link
/// supplied. If this 
/// operation fails,
/// an error is returned.
pub async fn fetch_page(
    link: &str
) -> Result<String, KleahErr>{
    let resp: Response = match get(link).await{
        Ok(resp) => resp,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    let text: String = match resp.text().await {
        Ok(text) => text,
        Err(e) => return Err::<String, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(text)
}

/// This function attempts
/// verify whether the 
/// normal Fediverse link
/// element is present
/// in the HTML code 
/// supplied. If this 
/// operation fails,
/// an error is returned.
pub fn verify_link_in_page(
    host: &str, 
    user: &str, 
    html: &str
) -> bool {
    let mut result: bool = false;
    let link_elem: String = format!(
        "<link rel=\"me\" href=\"{}/@{}\"/>", 
        host, 
        user
    );
    if html.contains(&link_elem){
        result = true;
    }
    else {}
    result
}

/// This function attempts
/// to check whether a Fediverse
/// verification link is present
/// in a link the user has supplied.
pub async fn verify_link(
    link: &str,
    host: &str,
    user: &str
) -> Result<bool, KleahErr>{
    let html: String = match fetch_page(link).await {
        Ok(html) => html,
        Err(e) => return Err::<bool, KleahErr>(KleahErr::new(&e.to_string()))
    };
    Ok(verify_link_in_page(host, user, &html))
}