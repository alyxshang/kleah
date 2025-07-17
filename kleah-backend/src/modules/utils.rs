use url::Url;
use super::err::KleahErr;
use std::collections::HashMap;

pub fn parse_query(
    url_str: &String
) -> Result<HashMap<String, String>, KleahErr>{
    let mut res: HashMap<String, String> = HashMap::new();
    let parsed = match Url::parse(url_str){
        Ok(parsed) => parsed,
        Err(e) => return Err::<HashMap<String,String>, KleahErr>(
            KleahErr::new(&e.to_string())
        ) 
    };
    let mut counter: usize = 0;
    while let Some(item) = parsed.query(){
        res.insert(format!("{:?}", counter), item.to_string());
        counter += 1;
    }
    Ok(res)
}
