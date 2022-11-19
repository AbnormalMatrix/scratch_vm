use serde::{Deserialize, Serialize};
use serde_json::{self, json};
mod blocks;

#[derive(Debug)]
struct Costume {
    name: String,
    data_format: String,
    image_path: String,
    rotation_center_x: f64,
    rotation_center_y: f64,
}


#[derive(Serialize, Deserialize, Debug)]
struct Variable {
    id: String,
    name: String,
    value: f64,
}

#[derive(Debug)]

struct Target {
    is_stage: bool,
    name: String,
    variables: Vec<Variable>,
    current_costume: u32,
    layer_order: u32,
    costumes: Vec<Costume>,
    blocks: Vec<blocks::Blocks>,


}

fn main() {
    let scratch_file = "testing/project.json";
    // load the file
    let contents = std::fs::read_to_string(scratch_file).unwrap();
    let json_data: serde_json::Value = serde_json::from_str(&contents).unwrap();

    let mut targets: Vec<Target> = Vec::new();

    for target in json_data["targets"].as_array().unwrap() {
        let is_stage = target["isStage"].as_bool().unwrap();
        let name = target["name"].as_str().unwrap();
        
        let mut variables: Vec<Variable> = Vec::new();


        for v in target["variables"].as_object().unwrap() {
            let v_id = v.0;
            
            let data = v.1.as_array().unwrap();
            let v_name = data[0].as_str().unwrap();
            let v_value = data[1].as_f64().unwrap();
            
            let new_variable = Variable {
                id: v_id.to_string(),
                name: v_name.to_string(),
                value: v_value,
            };
            variables.push(new_variable);
            
        }

        let current_costume = target["currentCostume"].as_u64().unwrap();
        let layer_order = target["layerOrder"].as_u64().unwrap();

        let mut costumes: Vec<Costume> = Vec::new();

        for c in target["costumes"].as_array().unwrap() {
            let c_obj = c.as_object().unwrap();
            let c_name = c_obj["name"].as_str().unwrap();
            let c_data_format = c_obj["dataFormat"].as_str().unwrap();
            let c_image_path = c_obj["md5ext"].as_str().unwrap();
            let c_rotation_center_x = c_obj["rotationCenterX"].as_f64().unwrap();
            let c_rotation_center_y = c_obj["rotationCenterY"].as_f64().unwrap();

            let new_costume = Costume {
                name: c_name.to_string(),
                data_format: c_data_format.to_string(),
                image_path: c_image_path.to_string(),
                rotation_center_x: c_rotation_center_x,
                rotation_center_y: c_rotation_center_y,
            };

            costumes.push(new_costume);



        }

        let mut blocks: Vec<blocks::Blocks> = Vec::new();
        for b in target["blocks"].as_object().unwrap() {
            let opcode = b.1.as_object().unwrap()["opcode"].as_str().unwrap();
            match opcode {
                "motion_movesteps" => {
                    let new_block = blocks::Blocks::MoveSteps(blocks::MoveSteps::new(b));
                    blocks.push(new_block);
                },
                "motion_turnright" => {
                    let new_block = blocks::Blocks::TurnRight(blocks::TurnRight::new(b));
                    blocks.push(new_block);
                },
                "motion_turnleft" => {
                    let new_block = blocks::Blocks::TurnLeft(blocks::TurnLeft::new(b));
                    blocks.push(new_block);
                },
                _ => {
                    println!("opcode not found");
                }
            
                
            }
        }

        let new_target = Target {
            is_stage,
            name: name.to_string(),
            variables,
            current_costume: current_costume as u32,
            layer_order: layer_order as u32,
            costumes,
            blocks,
        };
        println!("{:?}", new_target);
        targets.push(new_target);
    }
    

    
    
}
