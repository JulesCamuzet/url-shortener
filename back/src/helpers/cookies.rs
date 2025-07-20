use axum::http::HeaderMap;

pub fn get_cookie(headers: &HeaderMap, cookie_name: &str) -> Option<String> {
    let cookies = match headers.get("cookie") {
        None => return None,
        Some(cookies) => cookies
    };
    
    let str_cookies = match cookies.to_str() {
        Err(_) => return None,
        Ok(result) => result
    };
    
    let splitted_cookies = str_cookies.split(';');
    
    for cookie in splitted_cookies {
        let cookie_parts: Vec<&str> = cookie.split("=").collect();
       
        if cookie_parts.len() >= 2 && cookie_parts[0] == cookie_name {
            return Some(cookie_parts[1].to_string());
        }
    }
    
    return None;
}
