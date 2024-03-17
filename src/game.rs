//Import standard/external libraries
use rand::rngs::StdRng;
use json::object;
use elevate_lib::controller::ElevatorController;
use elevate_lib::elevators::Elevators;
use elevate_lib::floors::Floors;
use elevate_lib::people::People;
use elevate_lib::building::Building;

//Input source libraries
use crate::input::ElevatorGameInput;
use crate::upgrade::{ElevatorGameUpgrade, ElevatorGameUpgrades};

/// # `ElevatorGame` struct
///
/// The `ElevatorGame` is the main Universal Elevators game object.
pub struct ElevatorGame {
    controller: Box<dyn ElevatorController + Send>,
    upgrades: ElevatorGameUpgrades,
    tips: f64,
    rng: StdRng,
    time_steps: i32
}

//Implement the ElevatorGame interface
impl ElevatorGame {
    /// Initialize a new ElevatorGame given an `ElevatorController`
    /// implementation and a `StdRng` (from the rand library).
    ///
    /// ## Example
    ///
    /// ```
    /// let controller_rng = rand::StdRng::from_seed(rand::thread_rng().gen());
    /// let my_rng = rand::StdRng::from_seed(rand::thread_rng().gen());
    /// let my_building: Building = Building::from(
    ///     4_usize,
    ///     2_usize,
    ///     0.5_f64,
    ///     5.0_f64,
    ///     2.5_f64,
    ///     0.5_f64
    /// );
    /// let my_controller: RandomController = RandomController::from(
    ///     my_building,
    ///     controller_rng
    /// );
    /// let my_game: ElevatorGame = ElevatorGame::from(
    ///     Box::new(my_controller),
    ///     my_rng
    /// );
    /// ```
    pub fn from(controller: Box<dyn ElevatorController + Send>, upgrades: ElevatorGameUpgrades, rng: StdRng) -> ElevatorGame {
        //Initialize the game
        ElevatorGame {
            controller: controller,
            upgrades: upgrades,
            tips: 0.0_f64,
            rng: rng,
            time_steps: 0_i32
        }
    }

    /// Update the game state, for now this just increments the
    /// counter.
    pub fn update_game_state(&mut self, input: ElevatorGameInput) {
        //Make updates to the building prior to updating its elevators
        {
            //Mutably borrow the controller's building
            let building: &mut Building = self.controller.get_building_mut();

            //If the player collected tips, then collect the tips from the
            //building
            if input.collect_tips {
                self.tips += building.collect_tips();
            }

            //If the player added a floor or elevator, then add the floor
            //and/or elevator to the building
            if input.append_floor && self.upgrades.append_floor.is_enough(self.tips) {
                let cost: f64 = self.upgrades.append_floor.buy();
                self.tips -= cost;
                let capacity: usize = building.floors[0].capacity;
                building.append_floor(capacity);
            }
            if input.append_elevator && self.upgrades.append_elevator.is_enough(self.tips) {
                let cost: f64 = self.upgrades.append_elevator.buy();
                self.tips -= cost;
                let capacity: usize = building.elevators[0].capacity;
                let energy_up: f64 = building.elevators[0].energy_up;
                let energy_down: f64 = building.elevators[0].energy_down;
                let energy_coef: f64 = building.elevators[0].energy_coef;
                building.append_elevator(capacity, energy_up, energy_down, energy_coef);
            }

            //If the player added capacity to their floors or elevators,
            //then update their capacities
            if input.add_floor_capacity && self.upgrades.add_floor_capacity.is_enough(self.tips) {
                let cost: f64 = self.upgrades.add_floor_capacity.buy();
                self.tips -= cost;
                let current_capacity: usize = building.floors[0].capacity;
                building.floors.update_capacities(current_capacity + 100);
            }
            if input.add_elevator_capacity && self.upgrades.add_elevator_capacity.is_enough(self.tips) {
                let cost: f64 = self.upgrades.add_elevator_capacity.buy();
                self.tips -= cost;
                let current_capacity: usize = building.elevators[0].capacity;
                building.elevators.update_capacities(current_capacity + 10);
            }

            //Generate people arriving and leaving
            building.gen_people_arriving(&mut self.rng);
            building.gen_people_leaving(&mut self.rng);

            //Move people on and off the elevators and out of the building
            building.flush_and_update_tips(&mut self.rng);
            building.exchange_people_on_elevator();
        }

        //Update the building's elevators
        self.controller.update_elevators();

        //Make updates to the building after updading its elevators
        {
            //Mutably borrow the controller's building
            let building: &mut Building = self.controller.get_building_mut();

            //Increment the wait times, update average energy, update dest probabilities
            let energy_spent: f64 = building.elevators.get_energy_spent();
            building.increment_wait_times();
            building.update_average_energy(self.time_steps, energy_spent);
            building.update_dest_probabilities();
        }

        //Increment the time step counter
        self.time_steps += 1_i32;
    }

    /// Get the game state, for now this just returns a string with
    /// the counter state.
    pub fn get_game_state(&mut self) -> String {
        //Borrow the controller's building
        let building: &Building = self.controller.get_building();

        //Initialize a game state string
        let mut game_state = object!{
            floors: [],
            elevators: [],
            upgrades: {
                append_floor: {
                    name: self.upgrades.append_floor.get_name(),
                    description: self.upgrades.append_floor.get_description(),
                    cost: self.upgrades.append_floor.get_cost()
                },
                append_elevator: {
                    name: self.upgrades.append_elevator.get_name(),
                    description: self.upgrades.append_elevator.get_description(),
                    cost: self.upgrades.append_elevator.get_cost()
                }
            },
            avg_energy_spent: building.avg_energy,
            avg_wait_time: building.avg_wait_time,
            building_tips: building.tot_tips,
            collected_tips: self.tips
        };

        //Append the floor state for each floor
        for floor in building.floors.iter() {
            //Add the floor state to the game state object
            let _ = game_state["floors"].push(
                object!{
                    num_people: floor.get_num_people(),
                    capacity: floor.capacity,
                    are_people_waiting: floor.are_people_waiting()
                }
            );
        }

        //Append the elevator state for each floor 
        for elevator in building.elevators.iter() {
            //Add the elevator state to the game state object
            let _ = game_state["elevators"].push(
                object!{
                    num_people: elevator.get_num_people(),
                    capacity: elevator.capacity,
                    floor_on: elevator.floor_on
                }
            );
        }

        //Serialize and return the game state JSON string
        json::stringify(game_state)
    }
}