use chrono::NaiveDate;
use gray_matter::engine::YAML;
use gray_matter::Matter;
use leptos::*;
use pulldown_cmark::{html, Options, Parser};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::{
    fs::{self, DirEntry},
    path::Path,
};

use crate::modules::utils::render_jp_or_chn;

#[server(Getcases, "/api")]
pub async fn get_cases() -> Result<HashMap<CaseType, Vec<Case>>, ServerFnError> {
    let mut case_paths = HashMap::new();
    case_paths.insert(CaseType::Exhibition, CaseType::Exhibition.to_route());
    case_paths.insert(CaseType::Venue, CaseType::Venue.to_route());
    case_paths.insert(CaseType::Translation, CaseType::Translation.to_route());
    case_paths.insert(CaseType::Instrument, CaseType::Instrument.to_route());
    case_paths.insert(CaseType::Other, CaseType::Other.to_route());

    let mut all_cases = HashMap::new();

    for (case_type, path) in case_paths {
        let cases = process_cases("frontend/".to_owned() + &path);
        all_cases.insert(case_type, cases);
    }
    let mut all = all_cases.values().flatten().cloned().collect::<Vec<Case>>();
    sort_cases(&mut all);
    all_cases.insert(CaseType::All, all);
    Ok(all_cases)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CaseMetaData {
    pub image: String,
    pub title: String,
    pub date: String,
    pub description: String,
    pub tag: String,
}

impl CaseMetaData {
    pub fn create_href(&self) -> String {
        self.title.replace(' ', "-").to_lowercase()
    }
}

pub type CaseContent = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Case {
    pub case_metadata: CaseMetaData,
    pub case_content: CaseContent,
}

impl Case {
    pub fn new(case_metadata: CaseMetaData, case_content: CaseContent) -> Self {
        Self {
            case_metadata,
            case_content,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum CaseType {
    All,
    Exhibition,
    Venue,
    Translation,
    Instrument,
    Other,
}

impl Display for CaseType {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            CaseType::Exhibition => write!(f, "exhibitions"),
            CaseType::Venue => write!(f, "venues"),
            CaseType::Translation => write!(f, "translations"),
            CaseType::Instrument => write!(f, "instruments"),
            CaseType::All => write!(f, "all"),
            CaseType::Other => write!(f, "others"),
        }
    }
}

impl Iterator for CaseType {
    type Item = Self;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            CaseType::All => {
                *self = CaseType::Exhibition;
                Some(CaseType::All)
            }
            CaseType::Exhibition => {
                *self = CaseType::Venue;
                Some(CaseType::Exhibition)
            }
            CaseType::Venue => {
                *self = CaseType::Translation;
                Some(CaseType::Venue)
            }
            CaseType::Translation => {
                *self = CaseType::Instrument;
                Some(CaseType::Translation)
            }
            CaseType::Instrument => {
                *self = CaseType::Other;
                Some(CaseType::Instrument)
            }
            CaseType::Other => None,
        }
    }
}

impl CaseType {
    pub fn new() -> Self {
        CaseType::All
    }

    pub fn from(s: &str) -> Self {
        match s {
            "exhibition" => CaseType::Exhibition,
            "venue" => CaseType::Venue,
            "translation" => CaseType::Translation,
            "instrument" => CaseType::Instrument,
            "other" => CaseType::Other,
            _ => CaseType::All,
        }
    }

    pub fn to_tag_name(&self, cx: Scope) -> &'static str {
        match self {
            CaseType::Exhibition => render_jp_or_chn(cx, "展覧会の計画", "展覽規劃"),
            CaseType::Venue => render_jp_or_chn(cx, "会場の貸し出し", "場地租借"),
            CaseType::Instrument => render_jp_or_chn(cx, "楽器の貸し出し", "樂器租借"),
            CaseType::Translation => render_jp_or_chn(cx, "講演の翻訳", "演講翻譯"),
            CaseType::All => render_jp_or_chn(cx, "すべて", "全部"),
            CaseType::Other => render_jp_or_chn(cx, "その他", "其他"),
        }
    }

    pub fn to_route(&self) -> String {
        format!("/cases/{}", self.to_string())
    }
}

pub fn get_cases_file<P: AsRef<Path>>(path: P) -> Vec<DirEntry> {
    fs::read_dir(path)
        .unwrap()
        .flatten()
        .filter(|entry| match entry.file_type() {
            Ok(file_type) => file_type.is_file() && entry.path().extension() == Some("md".as_ref()),
            Err(_) => false,
        })
        .collect()
}

pub fn read_case_content(entry: DirEntry) -> Option<String> {
    fs::read_to_string(entry.path()).ok()
}

pub fn parse_case_content(content: &str) -> Option<Case> {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    let matter = Matter::<YAML>::new();

    let case_data = matter
        .parse_with_struct::<CaseMetaData>(content)
        .expect("Unable to parse md frontmatter");
    let case_metadata = case_data.data;
    let content = case_data.content;
    let parser = Parser::new_ext(&content, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    Some(Case::new(case_metadata, html_output))
}

pub fn sort_cases(cases: &mut [Case]) {
    cases.sort_by(|a, b| {
        let a_date = NaiveDate::parse_from_str(&a.case_metadata.date, "%Y-%m-%d").unwrap();
        let b_date = NaiveDate::parse_from_str(&b.case_metadata.date, "%Y-%m-%d").unwrap();
        //reverse sorting
        a_date.cmp(&b_date)
    });
}

pub fn process_cases<P: AsRef<Path>>(path: P) -> Vec<Case> {
    let cases_text = get_cases_file(path);
    let mut cases = Vec::new();

    for entry in cases_text {
        if let Some(content) = read_case_content(entry) {
            if let Some(case) = parse_case_content(&content) {
                cases.push(case);
            }
        }
    }

    sort_cases(&mut cases);

    cases
}
