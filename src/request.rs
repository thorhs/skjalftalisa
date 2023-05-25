use chrono::prelude::*;
use geo::Polygon;
use serde::Serialize;

#[derive(Serialize)]
pub struct SkjalftalisaRequest {
    area: Vec<Vec<f64>>,
    depth_min: i64,
    depth_max: i64,
    size_max: i64,
    size_min: i64,
    start_time: String,
    end_time: String,
    event_type: Vec<String>,
    fields: Vec<String>,
    magnitude_preference: Vec<String>,
    originating_system: Vec<String>,
}

impl SkjalftalisaRequest {
    pub fn new(polygon_area: Polygon) -> SkjalftalisaRequest {
        let area: Vec<Vec<f64>> = polygon_area
            .exterior()
            .points()
            .map(|p| vec![p.x(), p.y()])
            .collect();

        SkjalftalisaRequest {
            area,
            ..Default::default()
        }
    }

    pub fn with_size(mut self, min: i64, max: i64) -> Self {
        self.size_min = min;
        self.size_max = max;

        self
    }

    pub fn with_depth(mut self, min: i64, max: i64) -> Self {
        self.depth_min = min;
        self.depth_max = max;

        self
    }

    /*
    pub fn with_event_types(&mut self, event_type: Vec<&str>) -> &Self {
        self.event_type = event_type;

        self
    }
    */

    pub fn with_time(mut self, start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        self.start_time = start.format("%Y-%m-%d %H:%M:%S").to_string();
        self.end_time = end.format("%Y-%m-%d %H:%M:%S").to_string();

        self
    }
}

impl Default for SkjalftalisaRequest {
    fn default() -> SkjalftalisaRequest {
        SkjalftalisaRequest {
            area: vec![],
            depth_min: 0,
            depth_max: 25,
            size_min: 0,
            size_max: 11,
            start_time: "1970-01-01 00:00:00".to_owned(),
            end_time: "2030-12-31 23:59:59".to_owned(),
            event_type: vec!["qu".to_owned()],
            fields: vec![
                "time".to_owned(),
                "lat".to_owned(),
                "long".to_owned(),
                "depth".to_owned(),
                "magnitude".to_owned(),
                "magnitude_type".to_owned(),
                "originating_system".to_owned(),
            ],
            magnitude_preference: vec!["Mlw".to_owned(), "Autmag".to_owned()],
            originating_system: vec!["SIL picks".to_owned()],
        }
    }
}
