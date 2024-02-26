use json;

/// # `ElevatorGameInput` struct
///
/// The `ElevatorGameInput` struct contains information on the
/// input provided by the user via the front-end during a time
/// step.  It is used to update the game state based on user
/// input.
pub struct ElevatorGameInput {
    pub collect_tips: bool,
    pub append_floor: bool,
    pub append_elevator: bool
}

//Implement the ElevatorGameInput interface
impl ElevatorGameInput {
    /// Initialize an `ElevatorGameInput` struct explicitly
    pub fn new(collect_tips: bool, append_floor: bool, append_elevator: bool) -> ElevatorGameInput {
        ElevatorGameInput {
            collect_tips: collect_tips,
            append_floor: append_floor,
            append_elevator: append_elevator
        }
    }

    /// Initialize an `ElevatorGameInput` struct given a JSON
    /// serialized string containing an input object
    pub fn from_json(input: String) -> ElevatorGameInput {
        let input_object = json::parse(&input).unwrap();
        ElevatorGameInput {
            collect_tips: input_object["collect_tips"].as_bool().unwrap(),
            append_floor: input_object["append_floor"].as_bool().unwrap(),
            append_elevator: input_object["append_elevator"].as_bool().unwrap()
        }
    }
}