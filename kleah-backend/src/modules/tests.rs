use super::utils::parse_query;
use super::foreign::WebFingerLink;
use super::foreign::get_stream_url;
use super::foreign::WebFingerResponse;
use super::foreign::get_actor_info_url;
use super::foreign::get_actor_inbox_url;
use super::foreign::get_actor_apub_info;
use super::foreign::get_actor_outbox_url;

#[tokio::test]
pub async fn test_apub_stream(){
    let wfl_one = WebFingerLink{
        rel: String::from("http://webfinger.net/rel/profile-page"),
        content_type: Some(String::from("text/html")),
        href: Some(String::from("https://sakurajima.moe/@alyxshang")),
        template: None
    };
    let wfl_two = WebFingerLink{
        rel: String::from("self"),
        content_type: Some(String::from("application/activity+json")),
        href: Some(String::from("https://sakurajima.moe/users/alyxshang")),
        template: None,
    };
    let wfl_three = WebFingerLink{
        rel: String::from("http://ostatus.org/schema/1.0/subscribe"),
        content_type: None,
        href: None,
        template: Some(
            String::from("https://sakurajima.moe/authorize_interaction?uri={uri}")
        )
    };
    let wfl_four = WebFingerLink{
        rel: String::from("http://webfinger.net/rel/avatar"),
        content_type: Some(String::from("image/jpeg")),
        href: Some(String::from("https://media.sakurajima.moe/accounts/avatars/114/855/931/775/786/556/original/b1275b68ae315a29.jpeg")),
        template: None
    };
    let mut wfl_vec: Vec<WebFingerLink> = Vec::new();
    wfl_vec.push(wfl_one);
    wfl_vec.push(wfl_two);
    wfl_vec.push(wfl_three);
    wfl_vec.push(wfl_four);
    let mut alias_vec: Vec<String> = Vec::new();
    alias_vec.push(String::from("https://sakurajima.moe/@alyxshang"));
    alias_vec.push(String::from("https://sakurajima.moe/users/alyxshang"));
    let wfr: WebFingerResponse = WebFingerResponse{
        subject: String::from("acct:alyxshang@sakurajima.moe"),
        aliases: alias_vec,
        links: wfl_vec
    };
    match get_stream_url(
        &String::from("https://sakurajima.moe"), 
        &String::from("alyxshang")
    ).await {
        Ok(wf_resp) => assert_eq!(wf_resp, wfr),
        Err(e) => println!("{}", &e.to_string())
    };
}

#[tokio::test]
pub async fn test_actor_res_url(){
    match get_stream_url(
        &String::from("https://sakurajima.moe"), 
        &String::from("alyxshang")
    ).await {
        Ok(wfr) => {
            match get_actor_info_url(&wfr){
                Ok(my_str) => assert_eq!(&my_str, "https://sakurajima.moe/users/alyxshang"),
                Err(e) => println!("{}", &e.to_string())
            }
        },
        Err(e) => println!("{}", &e.to_string())
    };
}

#[tokio::test]
pub async fn test_actor_info_url(){
    match get_stream_url(
        &String::from("https://sakurajima.moe"), 
        &String::from("alyxshang")
    ).await {
        Ok(wfr) => {
            match get_actor_info_url(&wfr){
                Ok(my_str) => {
                    match get_actor_apub_info(&my_str).await {
                        Ok(res) => assert_eq!(res.id, String::from("https://sakurajima.moe/users/alyxshang")),
                        Err(e) => println!("{}", &e.to_string())
                    }
                },
                Err(e) => println!("{}", &e.to_string())
            }
        },
        Err(e) => println!("{}", &e.to_string())
    };
}

#[tokio::test]
pub async fn test_actor_inbox_url(){
    match get_stream_url(
        &String::from("https://sakurajima.moe"), 
        &String::from("alyxshang")
    ).await {
        Ok(wfr) => {
            match get_actor_info_url(&wfr){
                Ok(my_str) => {
                    match get_actor_apub_info(&my_str).await {
                        Ok(res) => assert_eq!(
                            get_actor_inbox_url(&res), 
                            String::from("https://sakurajima.moe/users/alyxshang/inbox")
                        ),
                        Err(e) => println!("{}", &e.to_string())
                    }
                },
                Err(e) => println!("{}", &e.to_string())
            }
        },
        Err(e) => println!("{}", &e.to_string())
    };
}

#[tokio::test]
pub async fn test_actor_outbox_url(){
    match get_stream_url(
        &String::from("https://sakurajima.moe"), 
        &String::from("alyxshang")
    ).await {
        Ok(wfr) => {
            match get_actor_info_url(&wfr){
                Ok(my_str) => {
                    match get_actor_apub_info(&my_str).await {
                        Ok(res) => assert_eq!(
                            get_actor_outbox_url(&res), 
                            String::from("https://sakurajima.moe/users/alyxshang/outbox")
                        ),
                        Err(e) => println!("{}", &e.to_string())
                    }
                },
                Err(e) => println!("{}", &e.to_string())
            }
        },
        Err(e) => println!("{}", &e.to_string())
    };
}

/*#[test]
pub fn test_url_parser(){
    let my_str: &str = "https://sakurajima.moe/users/alyxshang/statuses/114856349622127231/replies?only_other_accounts=true&page=true";
    match parse_query(&my_str.to_string()){
        Ok(res) => {
            assert_eq!(true, true);
            println!("{:?}", res);
        },
        Err(e) => println!("{}", &e.to_string())
    };

}*/
