use wasm_bindgen::prelude::*;

use anyhow::Error;
use serde_derive::{ Serialize,Deserialize};
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
pub struct Reading{
    x: String,
    y: f32
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct WateringChartData {
    sensor1_soil_humidity: Vec<Reading>,
    sensor1_watering: Vec<Reading>,
    sensor2_soil_humidity: Vec<Reading>,
    sensor2_watering: Vec<Reading>,
}

impl WateringChartData {
    fn from_dashboard(dashboard: &common::WateringDashboard) -> anyhow::Result<WateringChartData> {
        let mut sensor1_soil_humidity: Vec<Reading> = vec!();
        let mut sensor2_soil_humidity: Vec<Reading> = vec!();
        for reading in &dashboard.sensor_readings {
            match &reading.sensor[..] {
                "1" => sensor1_soil_humidity.push(Reading{x: reading.time.format("%Y-%m-%d %H:%M:%S").to_string(), y: reading.value}),
                "2" => sensor2_soil_humidity.push(Reading{x: reading.time.format("%Y-%m-%d %H:%M:%S").to_string(), y: reading.value}),
                _ => (),
            }
        }
        

        let mut sensor1_watering: Vec<Reading> = vec!();
        let mut sensor2_watering: Vec<Reading> = vec!();
        for reading in &dashboard.waterings {
            match &reading.sensor[..] {
                "1" => sensor1_watering.push(Reading{x: reading.time.format("%Y-%m-%d %H:%M:%S").to_string(), y: reading.duration_seconds as f32}),
                "2" => sensor2_watering.push(Reading{x: reading.time.format("%Y-%m-%d %H:%M:%S").to_string(), y: reading.duration_seconds as f32}),
                _ => (),
            }
        }

        let mut chart = WateringChartData::default();
        chart.sensor1_soil_humidity = sensor1_soil_humidity;
        chart.sensor2_soil_humidity = sensor2_soil_humidity;
        chart.sensor1_watering = sensor1_watering;
        chart.sensor2_watering = sensor2_watering;
        Ok(
            chart
        )
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

            let range = format!("{} - {}", value.from.format("%Y-%m-%d %H:%M:%S"), value.to.format("%Y-%m-%d %H:%M:%S"));
            html! {
                <p>{range}</p>
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
                    vec!(Msg::FetchReady(data))
                } else {
                    log::info!("fetch failed");
                    vec!() // FIXME: Handle this error accordingly..
                }
            },
        );


        let request = Request::get("http://127.0.0.1:8081/sensors/readings").body(Nothing).unwrap();
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
            },
            Msg::FetchReady(response) => {
                self.fetching = false;
                self.data = response.map(|data| data).ok();
                true
            },


        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <button onclick=self.link.callback(|_| Msg::FetchData)>
                    { "Fetch Data" }
                </button>

                { self.view_data() }

                <div style="width:75%;">
                    <canvas id="myChart"></canvas>
                </div>
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<Model>();
}