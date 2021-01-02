use wasm_bindgen::prelude::*;

use anyhow::Error;
use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};
use yew::format::{Json, Nothing};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

extern crate wasm_bindgen;

mod bindings;

pub enum Msg {
    FetchData,
    FetchReady(Result<common::WateringDashboard, Error>),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlantData {
    name: String,
    soil_humidity_readings: Vec<Reading>,
    watering_readings: Vec<Reading>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Reading {
    x: String,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WateringChartData {
    from: DateTime<Local>,
    to: DateTime<Local>,
    sensor1: PlantData,
    sensor2: PlantData,
}

impl WateringChartData {
    fn from_dashboard(dashboard: &common::WateringDashboard) -> anyhow::Result<WateringChartData> {
        let mut sensor1_name: Option<String> = None;
        let mut sensor2_name = None;

        let mut sensor1_soil_humidity: Vec<Reading> = vec![];
        let mut sensor2_soil_humidity: Vec<Reading> = vec![];
        for reading in &dashboard.sensor_readings {
            match &reading.sensor[..] {
                "1" => {
                    if let None = sensor1_name {
                        sensor1_name = Some(reading.name.clone());
                    }
                    sensor1_soil_humidity.push(Reading {
                        x: reading.time.to_rfc3339(),
                        y: reading.value,
                    })
                }
                "2" => {
                    if let None = sensor2_name {
                        sensor2_name = Some(reading.name.clone());
                    }
                    sensor2_soil_humidity.push(Reading {
                        x: reading.time.to_rfc3339(),
                        y: reading.value,
                    })
                }
                _ => (),
            }
        }

        let mut sensor1_watering: Vec<Reading> = vec![];
        let mut sensor2_watering: Vec<Reading> = vec![];
        for reading in &dashboard.waterings {
            match &reading.sensor[..] {
                "1" => sensor1_watering.push(Reading {
                    x: reading.time.to_rfc3339(),
                    y: reading.duration_seconds as f32,
                }),
                "2" => sensor2_watering.push(Reading {
                    x: reading.time.to_rfc3339(),
                    y: reading.duration_seconds as f32,
                }),
                _ => (),
            }
        }

        Ok(WateringChartData {
            from: dashboard.from,
            to: dashboard.to,
            sensor1: PlantData {
                name: sensor1_name.unwrap(),
                soil_humidity_readings: sensor1_soil_humidity,
                watering_readings: sensor1_watering,
            },
            sensor2: PlantData {
                name: sensor2_name.unwrap(),
                soil_humidity_readings: sensor2_soil_humidity,
                watering_readings: sensor2_watering,
            },
        })
    }
}

pub struct Model {
    fetching: bool,
    data: Option<common::WateringDashboard>,
    ft: Option<FetchTask>,

    link: ComponentLink<Model>,
}

impl Model {
    fn view_data(&self) -> Html {
        if let Some(value) = &self.data {
            let data = WateringChartData::from_dashboard(value).unwrap();
            bindings::load_dashboard(JsValue::from_serde(&data).unwrap());
            html! {
                <p></p>
            }
        } else {
            html! {
                <p>{ "Data hasn't fetched yet." }</p>
            }
        }
    }

    fn fetch_json(&mut self) -> yew::services::fetch::FetchTask {
        let callback = self.link.batch_callback(
            move |response: Response<Json<Result<common::WateringDashboard, Error>>>| {
                let (meta, Json(data)) = response.into_parts();
                println!("META: {:?}, {:?}", meta, data);
                if meta.status.is_success() {
                    log::info!("fetch ready {:?}", data);
                    vec![Msg::FetchReady(data)]
                } else {
                    log::info!("fetch failed");
                    vec![] // FIXME: Handle this error accordingly..
                }
            },
        );

        // let url = "/sensors/readings";
        // todo if trunk serve is running i should set it to this value
        let url = "http://192.168.0.100:8080/sensors/readings";
        let request = Request::get(url).body(Nothing).unwrap();
        FetchService::fetch(request, callback).unwrap()
    }
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut s = Self {
            fetching: false,
            data: None,
            ft: None,
            link,
        };
        s.update(Msg::FetchData);
        s
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::FetchData => {
                self.fetching = true;
                log::info!("fetching");
                let task = self.fetch_json();
                self.ft = Some(task);
                true
            }
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data).ok();
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                { self.view_data() }

                <div style="width:100%;">
                    <canvas id="myChart"></canvas>
                </div>
                <button onclick=self.link.callback(|_| Msg::FetchData)>
                { "Refresh data" }
                </button>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}
