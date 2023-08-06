use crate::modules::lang::Lang;

pub struct Services {
    jp: [&'static str; 5],
    ch: [&'static str; 5],
}

impl Services {
    pub const fn new() -> Self {
        Self {
            ch: ["展覽規劃", "場地租借", "演講翻譯", "樂器租借", "其他"],
            jp: [
                "展覧会の計画",
                "会場の貸し出し",
                "講演の翻訳",
                "楽器の貸し出し",
                "その他",
            ],
        }
    }

    pub fn get_service(&self, lang: Lang) -> Vec<String> {
        let target = match lang {
            Lang::CHN => self.ch,
            Lang::JP => self.jp,
        };
        target.iter().map(|s| s.to_string()).collect()
    }
}

pub const SERVICES: Services = Services::new();
