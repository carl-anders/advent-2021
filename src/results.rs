#[derive(Debug, Clone)]
pub struct DayResult<'a> {
    pub day: u8,
    pub part: Option<u8>,
    pub results: [&'a str; 2],
}
impl<'a> DayResult<'a> {
    pub fn get_name(&self) -> String {
        match self.part {
            Some(i) if i >= 2 => format!("{} training {}", self.day, i),
            Some(_) => format!("{} training", self.day),
            None => self.day.to_string(),
        }
    }
    pub fn get_file(&self) -> String {
        format!(
            "input/day{}.txt",
            match self.part {
                Some(i) if i >= 2 => format!("{}t{}", self.day, i),
                Some(_) => format!("{}t", self.day),
                None => self.day.to_string(),
            }
        )
    }
}

pub const RESULTS: [DayResult; 49] = [
    DayResult {
        day: 1,
        part: Some(1),
        results: ["7", "5"],
    },
    DayResult {
        day: 1,
        part: None,
        results: ["1557", "1608"],
    },
    DayResult {
        day: 2,
        part: Some(1),
        results: ["150", "900"],
    },
    DayResult {
        day: 2,
        part: None,
        results: ["1484118", "1463827010"],
    },
    DayResult {
        day: 3,
        part: Some(1),
        results: ["198", "230"],
    },
    DayResult {
        day: 3,
        part: None,
        results: ["3847100", "4105235"],
    },
    DayResult {
        day: 4,
        part: Some(1),
        results: ["4512", "1924"],
    },
    DayResult {
        day: 4,
        part: None,
        results: ["63552", "9020"],
    },
    DayResult {
        day: 5,
        part: Some(1),
        results: ["5", "12"],
    },
    DayResult {
        day: 5,
        part: None,
        results: ["6007", "19349"],
    },
    DayResult {
        day: 6,
        part: Some(1),
        results: ["5934", "26984457539"],
    },
    DayResult {
        day: 6,
        part: None,
        results: ["379114", "1702631502303"],
    },
    DayResult {
        day: 7,
        part: Some(1),
        results: ["37", "168"],
    },
    DayResult {
        day: 7,
        part: None,
        results: ["337833", "96678050"],
    },
    DayResult {
        day: 8,
        part: Some(1),
        results: ["26", "61229"],
    },
    DayResult {
        day: 8,
        part: None,
        results: ["452", "1096964"],
    },
    DayResult {
        day: 9,
        part: Some(1),
        results: ["15", "1134"],
    },
    DayResult {
        day: 9,
        part: None,
        results: ["562", "1076922"],
    },
    DayResult {
        day: 10,
        part: Some(1),
        results: ["26397", "288957"],
    },
    DayResult {
        day: 10,
        part: None,
        results: ["442131", "3646451424"],
    },
    DayResult {
        day: 11,
        part: Some(1),
        results: ["1656", "195"],
    },
    DayResult {
        day: 11,
        part: None,
        results: ["1644", "229"],
    },
    DayResult {
        day: 12,
        part: Some(1),
        results: ["226", "3509"],
    },
    DayResult {
        day: 12,
        part: None,
        results: ["5212", "134862"],
    },
    DayResult {
        day: 13,
        part: Some(1),
        results: [
            "17",
            "
█████
█   █
█   █
█   █
█████",
        ],
    },
    DayResult {
        day: 13,
        part: None,
        results: [
            "729",
            "
███   ██  ████ █    ███  █  █ ████ ███ 
█  █ █  █    █ █    █  █ █  █ █    █  █
█  █ █      █  █    ███  ████ ███  █  █
███  █ ██  █   █    █  █ █  █ █    ███ 
█ █  █  █ █    █    █  █ █  █ █    █   
█  █  ███ ████ ████ ███  █  █ █    █   ",
        ],
    },
    DayResult {
        day: 14,
        part: Some(1),
        results: ["1588", "2188189693529"],
    },
    DayResult {
        day: 14,
        part: None,
        results: ["2891", "4607749009683"],
    },
    DayResult {
        day: 15,
        part: Some(1),
        results: ["40", "315"],
    },
    DayResult {
        day: 15,
        part: None,
        results: ["456", "2831"],
    },
    DayResult {
        day: 16,
        part: Some(1),
        results: [
            "6, 9, 14, 16, 12, 23, 31, 14, 8, 15, 11, 13, 19, 16, 20",
            "2021, 1, 3, 15, 46, 46, 54, 3, 54, 7, 9, 1, 0, 0, 1",
        ],
    },
    DayResult {
        day: 16,
        part: None,
        results: ["947", "660797830937"],
    },
    DayResult {
        day: 17,
        part: Some(1),
        results: ["45", "112"],
    },
    DayResult {
        day: 17,
        part: None,
        results: ["7626", "2032"],
    },
    DayResult {
        day: 18,
        part: Some(1),
        results: ["4140", "3993"],
    },
    DayResult {
        day: 18,
        part: None,
        results: ["4120", "4725"],
    },
    DayResult {
        day: 19,
        part: Some(1),
        results: ["79", "3621"],
    },
    DayResult {
        day: 19,
        part: None,
        results: ["378", "13148"],
    },
    DayResult {
        day: 20,
        part: Some(1),
        results: ["35", "3351"],
    },
    DayResult {
        day: 20,
        part: None,
        results: ["5437", "19340"],
    },
    DayResult {
        day: 21,
        part: Some(1),
        results: ["739785", "444356092776315"],
    },
    DayResult {
        day: 21,
        part: None,
        results: ["926610", "146854918035875"],
    },
    DayResult {
        day: 22,
        part: Some(1),
        results: ["590784", "39769202357779"],
    },
    DayResult {
        day: 22,
        part: None,
        results: ["580012", "1334238660555542"],
    },
    DayResult {
        day: 23,
        part: Some(1),
        results: ["12521", "44169"],
    },
    DayResult {
        day: 23,
        part: None,
        results: ["14415", "41121"],
    },
    DayResult {
        day: 24,
        part: None,
        results: ["91599994399395", "71111591176151"],
    },
    DayResult {
        day: 25,
        part: Some(1),
        results: ["58", "1"],
    },
    DayResult {
        day: 25,
        part: None,
        results: ["300", "1"],
    },
];
