use serde::{Deserialize, Serialize};
use strum::Display;


#[derive(Debug, Display, Deserialize, Serialize)]
pub enum GeometryType {
    #[strum(to_string = "polyline")]
    Polyline,

    #[strum(to_string = "polyline6")]
    Polyline6,

    #[strum(to_string = "geojson")]
    GeoJson,
}

#[derive(Debug, Display, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Annotation {
    #[strum(to_string = "duration")]
    True,

    #[strum(to_string = "distance")]
    False,

    #[strum(to_string = "nodes")]
    Nodes,

    #[strum(to_string = "distance")]
    Distance,

    #[strum(to_string = "duration")]
    Duration,

    #[strum(to_string = "datasources")]
    DataSources,

    #[strum(to_string = "weight")]
    Weight,

    #[strum(to_string = "speed")]
    Speed,
}

#[derive(Debug, Display, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Overview {
    #[default]
    #[strum(to_string = "simplified")]
    Simplified,

    #[strum(to_string = "full")]
    Full,

    #[strum(to_string = "false")]
    False,
}

#[derive(Debug, Display, Deserialize, Serialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum Gaps {
    #[default]
    Split,
    Ignore,
}

#[derive(Debug, Display, Deserialize, Serialize, Default)]
pub enum Source {
    #[strum(to_string = "any")]
    #[default]
    Any,
    #[strum(to_string = "first")]
    First,
}

#[derive(Debug, Display, Deserialize, Serialize)]
#[allow(unused)]
pub enum Service {
    /// Finds the fastest route between coordinates in the supplied order.
    #[strum(to_string = "route")]
    Route {
        alternatives: Option<u32>,
        steps: bool,
        annotations: Option<Vec<Annotation>>,
        geometries: Option<GeometryType>,
        overview: Option<Overview>,
        continue_straight: bool,
        waypoints: Option<Vec<u32>>,
    },

    /// Snaps a coordinate to the street network and returns the nearest n matches.
    #[strum(to_string = "nearest")]
    Nearest {
        number: Option<u32>,
    },

    /// Computes the duration of the fastest route between all pairs of supplied coordinates.
    /// Returns the durations or distances or both between the coordinate pairs.
    /// Note that the distances are not the shortest distance between two coordinates, but rather the distances of the fastest routes.
    /// Duration is in seconds and distances is in meters.
    #[strum(to_string = "table")]
    Table {
        sources: Option<Vec<u32>>,
        destinations: Option<Vec<u32>>,
        fallback_speed: Option<f64>,
    },

    /// Map matching matches/snaps given GPS points to the road network in the most plausible way. Please note the request might result multiple sub-traces. Large jumps in the timestamps (> 60s) or improbable transitions lead to trace splits if a complete matching could not be found. The algorithm might not be able to match all points. Outliers are removed if they can not be matched successfully.
    #[strum(to_string = "match")]
    Match {
        steps: bool,
        geometries: Option<GeometryType>,
        annotations: Option<Vec<Annotation>>,

        #[serde(default)]
        overview: Option<Overview>,
        timestamps: Option<Vec<u32>>,

        radiuses: Option<Vec<f32>>,

        #[serde(default)]
        gaps: Option<String>,

        #[serde(default)]
        tidy: bool,

        /// indices of the input coordinates
        waypoints: Option<Vec<u32>>,
    },

    /// The trip plugin solves the Traveling Salesman Problem using a greedy heuristic (farthest-insertion algorithm) for 10 or more waypoints and uses brute force for less than 10 waypoints. The returned path does not have to be the fastest path. As TSP is NP-hard it only returns an approximation. Note that all input coordinates have to be connected for the trip service to work.
    #[strum(to_string = "trip")]
    Trip {
        roundtrip: bool,
        source: Source,
        annotations: Option<Vec<Annotation>>,

        geometries: Option<GeometryType>,

        #[serde(default)]
        overview: Option<Overview>,
    },

    /// TODO: Implement this service
    /// This service generates Mapbox Vector Tiles that can be viewed with a vector-tile capable slippy-map viewer. The tiles contain road geometries and metadata that can be used to examine the routing graph. The tiles are generated directly from the data in-memory, so are in sync with actual routing results, and let you examine which roads are actually routable, and what weights they have applied.
    #[strum(to_string = "tile")]
    Tile {

    },
}

#[derive(Debug, Display, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Profile {
    #[strum(to_string = "driving")]
    Car,
    #[strum(to_string = "bike")]
    Bike,
    #[strum(to_string = "foot")]
    Foot,
}

#[derive(Debug, Display, Deserialize, Serialize, Default)]
pub enum Format {
    #[strum(to_string = "json")]
    #[default]
    Json,
    #[strum(to_string = "flatbuffers")]
    FlatBuffers
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Request {
    pub service: Service,
    pub version: Option<String>,
    pub profile: Profile,
    
    // TODO: implement polyline
    pub coordinates: Vec<(f64, f64)>,
    pub format: Format,
}

impl Request {
    pub fn new(service: Service, version: Option<String>, profile: Profile, coordinates: Vec<(f64, f64)>, format: Format) -> Self {
        Request {
            service,
            version,
            profile,
            coordinates,
            format,
        }
    }
}

