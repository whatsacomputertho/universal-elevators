//Import standard/external libraries
use rand::rngs::StdRng;
use elevate_lib::controller::ElevatorController;

/// # `ElevatorGame` struct
///
/// The `ElevatorGame` is the main Universal Elevators game object.
pub struct ElevatorGame<T: ElevatorController> {
    pub controller: T,
    pub counter: i32,
    rng: StdRng
}

//Implement the ElevatorGame interface
impl<T: ElevatorController> ElevatorGame<T> {
    /// Initialize a new ElevatorGame given an `ElevatorController`
    /// implementation and a `StdRng` (from the rand library).
    ///
    /// ## Example
    ///
    /// ```
    /// let controller_rng = rand::StdRng::new();
    /// let my_rng = rand::StdRng::new();
    /// let my_building: Building = Building::fron(
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
            counter: 0_i32,
            rng: rng
        }
    }

    /// Update the game state, for now this just increments the
    /// counter.
    pub fn update_game_state(&mut self) {
        self.counter += 1_i32;
    }

    /// Get the game state, for now this just returns a string with
    /// the counter state.
    pub fn get_game_state(&self) -> String {
        format!("{{ 'counter' : {} }}", self.counter)
    }
}