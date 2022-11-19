use serde::{Deserialize, Serialize};
use serde_json::{self, json, Value};

#[derive(Debug)]
pub enum Blocks {
    MoveSteps(MoveSteps),
    TurnRight(TurnRight),
    TurnLeft(TurnLeft),
}


// motion blocks
#[derive(Debug)]
pub struct MoveSteps {
    id: String,
    next: String,
    parent: String,
    // block inputs
    steps: f64,
}

impl MoveSteps {
    pub fn new(json_data: (&String, &Value)) -> Self {
        let id = json_data.0;
        let b = json_data.1.as_object().unwrap();
        // check if b["next"] is not null
        let mut next = "null";
        if !b["next"].is_null() {
            next = b["next"].as_str().unwrap();
        }
        // check if b["paarent"] is not null
        let mut parent = "null";
        if !b["parent"].is_null() {
            parent = b["parent"].as_str().unwrap();
        }
        
        // this next part should be different for each block as these are the block's unique inputs
        // b["inputs"] looks like: Object {"STEPS": Array [Number(1), Array [Number(4), String("10")]]}
        // in the 0th array, 1 means that there is a shadow so we need to get the number
        // in the 1st array, 4 means that the input type is a number or in our case a f64

        let mut steps = 0.0;
        if b["inputs"]["STEPS"][0].as_u64().unwrap() == 1 {
            steps = b["inputs"]["STEPS"][1][1].as_str().unwrap().parse().unwrap();
        }
        
        MoveSteps {
            id: id.to_string(),
            next: next.to_string(),
            parent: parent.to_string(),
            steps,
        }
    }
}

#[derive(Debug)]
pub struct TurnRight {
    id: String,
    next: String,
    parent: String,
    // block inputs
    degrees: f64,
}

impl TurnRight {
    pub fn new(json_data: (&String, &Value)) -> Self {
        let id = json_data.0;
        let b = json_data.1.as_object().unwrap();
        // check if b["next"] is not null
        let mut next = "null";
        if !b["next"].is_null() {
            next = b["next"].as_str().unwrap();
        }
        // check if b["paarent"] is not null
        let mut parent = "null";
        if !b["parent"].is_null() {
            parent = b["parent"].as_str().unwrap();
        }
        
        // this next part should be different for each block as these are the block's unique inputs
        // b["inputs"] looks like: Object {"STEPS": Array [Number(1), Array [Number(4), String("10")]]}
        // in the 0th array, 1 means that there is a shadow so we need to get the number
        // in the 1st array, 4 means that the input type is a number or in our case a f64

        let mut degrees = 0.0;
        if b["inputs"]["DEGREES"][0].as_u64().unwrap() == 1 {
            degrees = b["inputs"]["DEGREES"][1][1].as_str().unwrap().parse().unwrap();
        }
        
        TurnRight  {
            id: id.to_string(),
            next: next.to_string(),
            parent: parent.to_string(),
            degrees,
        }
    }
}

#[derive(Debug)]
pub struct TurnLeft {
    id: String,
    next: String,
    parent: String,
    // block inputs
    degrees: f64,
}

impl TurnLeft {
    pub fn new(json_data: (&String, &Value)) -> Self {
        let id = json_data.0;
        let b = json_data.1.as_object().unwrap();
        // check if b["next"] is not null
        let mut next = "null";
        if !b["next"].is_null() {
            next = b["next"].as_str().unwrap();
        }
        // check if b["paarent"] is not null
        let mut parent = "null";
        if !b["parent"].is_null() {
            parent = b["parent"].as_str().unwrap();
        }
        
        // this next part should be different for each block as these are the block's unique inputs
        // b["inputs"] looks like: Object {"STEPS": Array [Number(1), Array [Number(4), String("10")]]}
        // in the 0th array, 1 means that there is a shadow so we need to get the number
        // in the 1st array, 4 means that the input type is a number or in our case a f64

        let mut degrees = 0.0;
        if b["inputs"]["DEGREES"][0].as_u64().unwrap() == 1 {
            degrees = b["inputs"]["DEGREES"][1][1].as_str().unwrap().parse().unwrap();
        }
        
        TurnLeft  {
            id: id.to_string(),
            next: next.to_string(),
            parent: parent.to_string(),
            degrees,
        }
    }
}