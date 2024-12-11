#![allow(dead_code)]
pub struct PgnHeaders {
    event: Option<String>,
    site: Option<String>,
    date: Option<String>,
    white: Option<String>,
    black: Option<String>,
    result: Option<String>,
}

impl PgnHeaders{
    pub fn new() -> PgnHeaders {
        PgnHeaders {
            event: None,
            site: None,
            date: None,
            white: None,
            black: None,
            result: None,
        }
    }

    pub fn set_event(&mut self, event: String) {
        self.event = Some(event);
    }

    pub fn set_site(&mut self, site: String) {
        self.site = Some(site);
    }

    pub fn set_date(&mut self, date: String) {
        self.date = Some(date);
    }

    pub fn set_white(&mut self, white: String) {
        self.white = Some(white);
    }

    pub fn set_black(&mut self, black: String) {
        self.black = Some(black);
    }

    pub fn set_result(&mut self, result: String) {
        self.result = Some(result);
    }

    pub fn print_headers(&self) {
        println!("Event: {}", self.event.as_deref().unwrap_or(""));
        println!("Site: {}", self.site.as_deref().unwrap_or(""));
        println!("Date: {}", self.date.as_deref().unwrap_or(""));
        println!("White: {}", self.white.as_deref().unwrap_or(""));
        println!("Black: {}", self.black.as_deref().unwrap_or(""));
        println!("Result: {}", self.result.as_deref().unwrap_or(""));
    }
}