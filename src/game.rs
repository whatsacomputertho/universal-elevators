//Import standard/external libraries
use rand::rngs::StdRng;
use json::object;
use elevate_lib::controller::ElevatorController;
use elevate_lib::elevators::Elevators;
use elevate_lib::floors::Floors;
use elevate_lib::people::People;
use elevate_lib::building::Building;

/// # `ElevatorGame` struct
///
/// The `ElevatorGame` is the main Universal Elevators game object.
pub struct ElevatorGame<T: ElevatorController> {
    pub controller: T,
    rng: StdRng,
    time_steps: i32
}

//Implement the ElevatorGame interface
impl<T: ElevatorController> ElevatorGame<T> {
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
    /// let my_game: ElevatorGame<RandomController> = ElevatorGame::from(
    ///     my_controller,
    ///     my_rng
    /// );
    /// ```
    pub fn from(controller: T, rng: StdRng) -> ElevatorGame<T> {
        //Initialize the game
        ElevatorGame {
            controller: controller,
            rng: rng,
            time_steps: 0_i32
        }
    }

    /// Update the game state, for now this just increments the
    /// counter.
    pub fn update_game_state(&mut self) {
        //Make updates to the building prior to updating its elevators
        {
            //Mutably borrow the controller's building
            let building: &mut Building = self.controller.get_building_mut();

            //Generate people arriving and leaving
            building.gen_people_arriving(&mut self.rng);
            building.gen_people_leaving(&mut self.rng);

            //Move people on and off the elevators and out of the building
            building.flush_first_floor();
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
        //Initialize a game state string
        let mut game_state = object!{
            floors: [],
            elevators: [],
            avg_energy_spent: 0.0_f64,
            avg_wait_time: 0.0_f64
        };

        //Append the floor state for each floor
        let building: &Building = self.controller.get_building();
        for floor in building.floors.iter() {
            //Add the floor state to the game state object
            let _ = game_state["floors"].push(
                object!{
                    num_people: floor.get_num_people(),
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
                    floor_on: elevator.floor_on
                }
            );
        }

        //Update the average energy spent and average wait time
        //in the game state object
        game_state["avg_energy_spent"] = building.avg_energy.into();
        game_state["avg_wait_time"] = building.avg_wait_time.into();

        //Serialize and return the game state JSON string
        json::stringify(game_state)
    }
}