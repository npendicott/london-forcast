// use chrono::NaiveDate;
use serde::{Serialize, Deserialize, Deserializer, Serializer};

// Custom reading parser
// Or, All this for a trim?
fn parse_energy_reading<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error> 
    where D: Deserializer<'de> 
{
    // But hey, we probs need to parse anyway, right?
    let raw_val = String::deserialize(deserializer)?;
    if raw_val == "Null" {
        return Ok(None);
    }

    return Ok(Some(raw_val.trim().parse().unwrap()))
}


//London
#[derive(Serialize, Debug, Deserialize, PartialEq)]
pub struct HHourlyReading {
    #[serde(rename = "LCLid")]
    lcl_id: String,

    #[serde(rename = "tstp")]
    timestamp: String, // DT

    // TODO: Do this right
    #[serde(rename = "energy(kWh/hh)", deserialize_with = "parse_energy_reading")]
    pub energy_kwh_hh: Option<f64>,
}

impl HHourlyReading {
    pub fn from_line(_line: &str) -> HHourlyReading {
        unimplemented!()
    }

    // pub fn to_line() -> Vec<String, String, Option<f64>> {
    //     unimplemented!()
    // }

    pub fn print(&self) {
        println!("MAC: {}", self.lcl_id);
        println!("Timestamp: {}", self.timestamp);
        // println!("Mean: {}", self.energy_kwh_hh);
        match &self.energy_kwh_hh {
            Some(results) => println!("Mean: {}", results),
            None => println!("Mean: NULL")
        }
        println!();
    }
}

// #[derive(Debug, Deserialize, PartialEq)]
// pub struct DailyReading {
//     #[serde(rename = "LCLid")]
//     lcl_id: String,

//     #[serde(rename = "day")]
//     day: String, // DT

//     #[serde(rename = "energy_median")]
//     energy_median: f64,

//     #[serde(rename = "energy_mean")]
//     energy_mean: f64,
    
//     #[serde(rename = "energy_max")]
//     energy_max: f64,

//     #[serde(rename = "energy_count")]
//     energy_count: i64,

//     #[serde(rename = "energy_std")]
//     energy_std: Option<f64>,

//     #[serde(rename = "energy_sum")]
//     energy_sum: f64,

//     #[serde(rename = "energy_min")]
//     energy_min: f64,
// }

// impl DailyReading {
//     pub fn from_line(line: &str) -> DailyReading {
//         unimplemented!()
//     }

//     pub fn print(&self) {
//         println!("MAC: {}", self.lcl_id);
//         println!("Day: {}", self.day);
//         println!("Mean: {}", self.energy_mean);
//         println!();
//     }
// }

// // Test
// #[derive(Debug, Deserialize, PartialEq)]
// pub struct Stuff {
//     #[serde(rename = "Latitude")]
//     latitude: f64,

//     #[serde(rename = "Longitude")]
//     longitude: f64,

//     #[serde(rename = "Population")]
//     population: Option<u64>,

//     #[serde(rename = "City")]
//     city: String,

//     #[serde(rename = "State")]
//     state: String,
// }

// impl Stuff {
//     pub fn from_line(line: &str) -> Stuff {
//         unimplemented!()
//     }

//     pub fn print(&self) {
//         println!("{0}, {1}", self.city, self.state);
//         println!("lat: {}", self.latitude);
//         println!("lng: {}", self.longitude);
//         println!();
//     }
// }


