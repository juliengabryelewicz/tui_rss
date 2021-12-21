use std::error::Error;
use std::fs;
use std::io::BufReader;
use reqwest;
use rss::Channel;
use serde::{Deserialize, Serialize};
use webbrowser;


#[derive(Serialize, Deserialize, Clone)]
pub struct Rss {
    pub name: String,
    link: String
}

pub enum State {
    Init,
    Initialized {
        rss_selected: u8,
        news_selected: u8,
        list_rss:Vec<Rss>,
        list_news:Channel
    },
}

const DB_PATH: &str = "./data/db.json";

impl State {
    pub fn initialized() -> Self {
        let rss_selected = 0;
        let news_selected = 0;
        let list_rss = Self::read_rss_list();
        let list_news = Channel::default();
        Self::Initialized {
            rss_selected,
            news_selected,
            list_rss,
            list_news,
        }
    }

    fn read_rss_list() -> Vec<Rss> {
        let db_content = fs::read_to_string(DB_PATH).unwrap();
        let parsed: Vec<Rss> = serde_json::from_str(&db_content).unwrap();
        parsed
    }

    pub fn up_rss(&mut self) {
        if let Self::Initialized { rss_selected, list_rss, .. } = self {
            if *rss_selected != list_rss.len() as u8 - 1 {
                *rss_selected = *rss_selected+1;
            }
        }
    }

    pub fn down_rss(&mut self) {
        if let Self::Initialized { rss_selected, .. } = self {
            if *rss_selected!=0 {
                *rss_selected = *rss_selected-1;
            }
        }
    }

    pub fn up_news(&mut self) {
        if let Self::Initialized { news_selected, list_news, .. } = self {
            let item_count = list_news.items().len();
            if *news_selected != item_count as u8 - 1{
                *news_selected = *news_selected+1;
            }
        }
    }

    pub fn initialize_news(&mut self) {
        if let Self::Initialized { news_selected, .. } = self {
            *news_selected = 0;
        }
    }

    pub fn down_news(&mut self) {
        if let Self::Initialized { news_selected, .. } = self {
            if *news_selected!=0 {
                *news_selected = *news_selected-1;
            }
        }
    }

    pub fn open_browser(&mut self) {
        if let Self::Initialized { news_selected, list_news, .. } = self {
            let item = list_news.items().get(*news_selected as usize).unwrap();
            let _link_news = if let Some(link_news) = item.link() {
                webbrowser::open(link_news);
            };
        }
    }

    pub fn list_rss(&self) -> Option<&Vec<Rss>> {
        if let Self::Initialized { list_rss, .. } = self {
            Some(list_rss)
        } else {
            None
        }
    }

    pub fn rss_selected(&self) -> Option<u8> {
        if let Self::Initialized { rss_selected, .. } = self {
            Some(*rss_selected)
        } else {
            Some(0)
        }
    }

    pub fn news_selected(&self) -> Option<u8> {
        if let Self::Initialized { news_selected, .. } = self {
            Some(*news_selected)
        } else {
            Some(0)
        }
    }

    pub fn list_news(&self) -> Option<&Channel> {
        if let Self::Initialized { list_news, .. } = self {
            Some(list_news)
        } else {
            None
        }
    }

    pub fn read_news_from_rss(&mut self)  {
        if let Self::Initialized { rss_selected, list_rss, list_news, ..} = self {
            let news = Self::get_news(&list_rss[*rss_selected as usize].link);
            *list_news  = match news{
                Ok(v) => { v },
                Err(_e) => { Channel::default() }  
            }
        }
    }

    fn get_news(url_newspaper:&str) -> Result<Channel, Box<dyn Error>> {
        let content = reqwest::blocking::get(url_newspaper)?;
        let channel = Channel::read_from(BufReader::new(content)).unwrap();
        Ok(channel)
    }

}

impl Default for State {
    fn default() -> Self {
        Self::Init
    }
}
