use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SkjalftalisaResponseData {
    depth: Vec<f64>,
    lat: Vec<f64>,
    long: Vec<f64>,
    magnitude: Vec<f64>,
    magnitude_type: Vec<String>,
    originating_system: Vec<String>,
    time: Vec<i64>,
}

#[derive(Deserialize, Debug)]
pub struct SkjalftalisaResponse {
    pub data: SkjalftalisaResponseData,
}

impl SkjalftalisaResponseData {
    pub fn get(&self, index: usize) -> Option<Quake> {
        Some(Quake {
            depth: *self.depth.get(index)?,
            lat: *self.lat.get(index)?,
            long: *self.long.get(index)?,
            magnitude: *self.magnitude.get(index)?,
            magnitude_type: self.magnitude_type.get(index)?.to_owned(),
            originating_system: self.originating_system.get(index)?.to_owned(),
            time: *self.time.get(index)?,
        })
    }

    pub fn iter(&self) -> SkjalftalisaResponseIter {
        SkjalftalisaResponseIter {
            data: self,
            index: 0,
        }
    }
}

pub struct SkjalftalisaResponseIter<'a> {
    data: &'a SkjalftalisaResponseData,
    index: usize,
}

impl<'a> Iterator for SkjalftalisaResponseIter<'a> {
    type Item = Quake;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.index;
        self.index = current + 1;
        Some(Quake {
            depth: *self.data.depth.get(current)?,
            lat: *self.data.lat.get(current)?,
            long: *self.data.long.get(current)?,
            magnitude: *self.data.magnitude.get(current)?,
            magnitude_type: self.data.magnitude_type.get(current)?.to_owned(),
            originating_system: self.data.originating_system.get(current)?.to_owned(),
            time: *self.data.time.get(current)?,
        })
    }
}

pub struct SkjalftalisaResponseDataTimeIter<'a> {
    data: &'a SkjalftalisaResponseData,
    index: usize,
    start_time: i64,
    end_time: i64,
}

impl<'a> Iterator for SkjalftalisaResponseDataTimeIter<'a> {
    type Item = Quake;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let current = self.index;
            self.index = current + 1;
            let time = self.data.time.get(current)?;
            if *time >= self.start_time && *time <= self.end_time {
                return Some(Quake {
                    depth: *self.data.depth.get(current)?,
                    lat: *self.data.lat.get(current)?,
                    long: *self.data.long.get(current)?,
                    magnitude: *self.data.magnitude.get(current)?,
                    magnitude_type: self.data.magnitude_type.get(current)?.to_owned(),
                    originating_system: self.data.originating_system.get(current)?.to_owned(),
                    time: *self.data.time.get(current)?,
                });
            }
        }
    }
}

#[derive(Default, Debug)]
pub struct Quake {
    pub depth: f64,
    pub lat: f64,
    pub long: f64,
    pub magnitude: f64,
    pub magnitude_type: String,
    pub originating_system: String,
    pub time: i64,
}

#[cfg(test)]
mod tests {
    static TEST_RESPONSE_JSON: &str = r#"{ "data": {
    "depth": [ 8.39, 8.314, 8.067 ],
    "lat": [ 66.13165, 66.13033, 66.12598 ],
    "long": [ -17.80339, -17.80419, -17.81501 ],
    "magnitude": [ 0.65, 1.11, 0.18 ],
    "magnitude_type": [ "mlw", "mlw", "mlw" ],
    "originating_system": [ "SIL picks", "SIL picks", "SIL picks" ],
    "time": [ 1684432911, 1684433664, 1684434229 ]
  }}"#;

    #[test]
    fn test_parse_data() {
        let data: super::SkjalftalisaResponse = serde_json::from_str(TEST_RESPONSE_JSON).unwrap();

        assert_eq!(data.data.depth.len(), 3);
    }

    #[test]
    fn test_get_item() {
        let data: super::SkjalftalisaResponse = serde_json::from_str(TEST_RESPONSE_JSON).unwrap();

        let quake = data.data.get(1).unwrap();

        assert_eq!(quake.depth, 8.314);
        assert_eq!(quake.lat, 66.13033);
        assert_eq!(quake.long, -17.80419);
        assert_eq!(quake.magnitude, 1.11);
        assert_eq!(quake.magnitude_type, "mlw");
        assert_eq!(quake.originating_system, "SIL picks");
        assert_eq!(quake.time, 1684433664);
    }

    #[test]
    fn test_get_item_oob() {
        let data: super::SkjalftalisaResponse = serde_json::from_str(TEST_RESPONSE_JSON).unwrap();

        assert!(data.data.get(4).is_none());
    }
    #[test]
    fn test_iter() {
        let data: super::SkjalftalisaResponse = serde_json::from_str(TEST_RESPONSE_JSON).unwrap();

        let iter = data.data.iter();

        let output: Vec<super::Quake> = iter.collect();

        assert_eq!(output.len(), 3);
    }
    // "time": [ 1684432911, 1684433664, 1684434229 ]
}
