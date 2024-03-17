/// # `ElevatorGameUpgrades` struct
///
/// The `ElevatorGameUpgrades` struct stores each elevator game
/// upgrade and provides an interface through which one may interact
/// with the game's upgrades.
pub struct ElevatorGameUpgrades {
    pub collect_tips: CollectTipsUpgrade,
    pub append_floor: AppendFloorUpgrade,
    pub append_elevator: AppendElevatorUpgrade,
    pub add_floor_capacity: AddFloorCapacityUpgrade,
    pub add_elevator_capacity: AddElevatorCapacityUpgrade
}

impl ElevatorGameUpgrades {
    /// Initialize the `ElevatorGameUpgrades` struct
    pub fn new() -> ElevatorGameUpgrades {
        ElevatorGameUpgrades {
            collect_tips: CollectTipsUpgrade::new(),
            append_floor: AppendFloorUpgrade::new(10_f64, 1.5_f64),
            append_elevator: AppendElevatorUpgrade::new(100_f64, 1.9_f64),
            add_floor_capacity: AddFloorCapacityUpgrade::new(10_f64, 1.1_f64),
            add_elevator_capacity: AddElevatorCapacityUpgrade::new(10_f64, 1.1_f64)
        }
    }
}

/// # `ElevatorGameUpgrade` trait
///
/// The `ElevatorGameUpgrade` trait specifies the interface through
/// which the game interacts with different types of upgrades.
pub trait ElevatorGameUpgrade {
    fn get_cost(&self) -> f64;

    fn is_enough(&self, money: f64) -> bool;

    fn get_max_buys(&self) -> usize;

    fn get_name(&self) -> &str;

    fn get_description(&self) -> &str;

    fn buy(&mut self) -> f64;
}

/// # `CollectTipsUpgrade` struct
///
/// The `CollectTipsUpgrade` struct is an upgrade for collecting tips
/// from the player's building.  It is constantly available throughout
/// the game.
pub struct CollectTipsUpgrade {
    name: String,
    description: String
}

impl CollectTipsUpgrade {
    /// Initialize an `CollectTipsUpgrade` struct
    pub fn new() -> CollectTipsUpgrade {
        //Set the name of the upgrade and the description
        let name = "Collect Tips".to_string();
        let description = "Collect the tips accumulated by your building".to_string();

        //Initialize and return the CollectTipsUpgrade
        CollectTipsUpgrade {
            name: name,
            description: description
        }
    }
}

impl ElevatorGameUpgrade for CollectTipsUpgrade {
    /// Get the cost of the upgrade
    fn get_cost(&self) -> f64 {
        0.0_f64
    }

    /// Check if the given amount is less than the cost of the upgrade
    fn is_enough(&self, money: f64) -> bool {
        true
    }

    /// Get the maximum number of floors one can buy
    fn get_max_buys(&self) -> usize {
        usize::MAX
    }

    /// Get the name of the upgrade
    fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the description of the upgrade
    fn get_description(&self) -> &str {
        &self.description
    }

    /// Update the upgrade properties after buying
    fn buy(&mut self) -> f64 {
        0.0_f64
    }
}

/// # `AppendFloorUpgrade` struct
///
/// The `AppendFloorUpgrade` struct is an upgrade for adding a floor
/// to the player's building.  It is constantly available throughout
/// the game.
pub struct AppendFloorUpgrade {
    base_cost: f64,
    base_coef: f64,
    num_buys: usize,
    max_buys: usize,
    name: String,
    description: String
}

impl AppendFloorUpgrade {
    /// Initialize an `AppendFloorUpgrade` struct
    pub fn new(base_cost: f64, base_coef: f64) -> AppendFloorUpgrade {
        //Set the name of the upgrade and the description
        let name = "Add Floor".to_string();
        let description = "Adds a new floor to your building".to_string();

        //Initialize and return the AppendFloorUpgrade
        AppendFloorUpgrade {
            base_cost: base_cost,
            base_coef: base_coef,
            num_buys: 0_usize,
            max_buys: usize::MAX,
            name: name,
            description: description
        }
    }
}

impl ElevatorGameUpgrade for AppendFloorUpgrade {
    /// Get the cost of the upgrade
    fn get_cost(&self) -> f64 {
        self.base_cost + f64::powf(self.base_coef, self.num_buys as f64)
    }

    /// Check if the given amount is less than the cost of the upgrade
    fn is_enough(&self, money: f64) -> bool {
        money >= self.base_cost + f64::powf(self.base_coef, self.num_buys as f64)
    }

    /// Get the maximum number of floors one can buy
    fn get_max_buys(&self) -> usize {
        self.max_buys
    }

    /// Get the name of the upgrade
    fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the description of the upgrade
    fn get_description(&self) -> &str {
        &self.description
    }

    /// Update the upgrade properties after buying
    fn buy(&mut self) -> f64 {
        //Make sure the upgrade can be purchased
        if self.num_buys > self.max_buys {
            panic!("Cannot buy upgrade: {}", self.name);
        }

        //Calculate the cost before incrementing the num buys
        let cost: f64 = self.base_cost + f64::powf(self.base_coef, self.num_buys as f64);

        //If it can be purchased, then update the number of buys
        self.num_buys += 1;

        //Return the cost
        cost
    }
}

/// # `AppendElevatorUpgrade` struct
///
/// The `AppendElevatorUpgrade` struct is an upgrade for adding an elevator
/// to the player's building.  It is constantly available throughout
/// the game.
pub struct AppendElevatorUpgrade {
    base_cost: f64,
    base_coef: f64,
    num_buys: usize,
    max_buys: usize,
    name: String,
    description: String
}

impl AppendElevatorUpgrade {
    /// Initialize an `AppendElevatorUpgrade` struct
    pub fn new(base_cost: f64, base_coef: f64) -> AppendElevatorUpgrade {
        //Set the name of the upgrade and the description
        let name = "Add Elevator".to_string();
        let description = "Adds a new elevator to your building".to_string();

        //Initialize and return the AppendElevatorUpgrade
        AppendElevatorUpgrade {
            base_cost: base_cost,
            base_coef: base_coef,
            num_buys: 0_usize,
            max_buys: usize::MAX,
            name: name,
            description: description
        }
    }
}

impl ElevatorGameUpgrade for AppendElevatorUpgrade {
    /// Get the cost of the upgrade
    fn get_cost(&self) -> f64 {
        self.base_cost + f64::powf(self.base_coef, self.num_buys as f64)
    }

    /// Check if the given amount is less than the cost of the upgrade
    fn is_enough(&self, money: f64) -> bool {
        money >= self.base_cost + f64::powf(self.base_coef, self.num_buys as f64)
    }

    /// Get the maximum number of floors one can buy
    fn get_max_buys(&self) -> usize {
        self.max_buys
    }

    /// Get the name of the upgrade
    fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the description of the upgrade
    fn get_description(&self) -> &str {
        &self.description
    }

    /// Update the upgrade properties after buying
    fn buy(&mut self) -> f64 {
        //Make sure the upgrade can be purchased
        if self.num_buys > self.max_buys {
            panic!("Cannot buy upgrade: {}", self.name);
        }

        //Calculate the cost before incrementing the num buys
        let cost: f64 = self.base_cost + f64::powf(self.base_coef, self.num_buys as f64);

        //If it can be purchased, then update the number of buys
        self.num_buys += 1;

        //Return the cost
        cost
    }
}

/// # `AddFloorCapacityUpgrade` struct
///
/// The `AddFloorCapacityUpgrade` struct is an upgrade for adding capacity
/// to the floors of the player's buildings.  It is constantly available
/// throughout the game.
pub struct AddFloorCapacityUpgrade {
    base_cost: f64,
    base_coef: f64,
    num_buys: usize,
    max_buys: usize,
    name: String,
    description: String
}

impl AddFloorCapacityUpgrade {
    /// Initialize an `AddFloorCapacityUpgrade` struct
    pub fn new(base_cost: f64, base_coef: f64) -> AddFloorCapacityUpgrade {
        //Set the name of the upgrade and the description
        let name = "Add Floor Capacity".to_string();
        let description = "Adds more capacity to your floors".to_string();

        //Initialize and return the AddFloorCapacityUpgrade
        AddFloorCapacityUpgrade {
            base_cost: base_cost,
            base_coef: base_coef,
            num_buys: 0_usize,
            max_buys: usize::MAX,
            name: name,
            description: description
        }
    }
}

impl ElevatorGameUpgrade for AddFloorCapacityUpgrade {
    /// Get the cost of the upgrade
    fn get_cost(&self) -> f64 {
        self.base_cost + f64::powf(self.base_coef, self.num_buys as f64)
    }

    /// Check if the given amount is less than the cost of the upgrade
    fn is_enough(&self, money: f64) -> bool {
        money >= self.base_cost + f64::powf(self.base_coef, self.num_buys as f64)
    }

    /// Get the maximum number of floors one can buy
    fn get_max_buys(&self) -> usize {
        self.max_buys
    }

    /// Get the name of the upgrade
    fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the description of the upgrade
    fn get_description(&self) -> &str {
        &self.description
    }

    /// Update the upgrade properties after buying
    fn buy(&mut self) -> f64 {
        //Make sure the upgrade can be purchased
        if self.num_buys > self.max_buys {
            panic!("Cannot buy upgrade: {}", self.name);
        }

        //Calculate the cost before incrementing the num buys
        let cost: f64 = self.base_cost + f64::powf(self.base_coef, self.num_buys as f64);

        //If it can be purchased, then update the number of buys
        self.num_buys += 1;

        //Return the cost
        cost
    }
}

/// # `AddElevatorCapacityUpgrade` struct
///
/// The `AddElevatorCapacityUpgrade` struct is an upgrade for adding capacity
/// to the floors of the player's buildings.  It is constantly available
/// throughout the game.
pub struct AddElevatorCapacityUpgrade {
    base_cost: f64,
    base_coef: f64,
    num_buys: usize,
    max_buys: usize,
    name: String,
    description: String
}

impl AddElevatorCapacityUpgrade {
    /// Initialize an `AddElevatorCapacityUpgrade` struct
    pub fn new(base_cost: f64, base_coef: f64) -> AddElevatorCapacityUpgrade {
        //Set the name of the upgrade and the description
        let name = "Add Elevator Capacity".to_string();
        let description = "Adds more capacity to your elevators".to_string();

        //Initialize and return the AddElevatorCapacityUpgrade
        AddElevatorCapacityUpgrade {
            base_cost: base_cost,
            base_coef: base_coef,
            num_buys: 0_usize,
            max_buys: usize::MAX,
            name: name,
            description: description
        }
    }
}

impl ElevatorGameUpgrade for AddElevatorCapacityUpgrade {
    /// Get the cost of the upgrade
    fn get_cost(&self) -> f64 {
        self.base_cost + f64::powf(self.base_coef, self.num_buys as f64)
    }

    /// Check if the given amount is less than the cost of the upgrade
    fn is_enough(&self, money: f64) -> bool {
        money >= self.base_cost + f64::powf(self.base_coef, self.num_buys as f64)
    }

    /// Get the maximum number of floors one can buy
    fn get_max_buys(&self) -> usize {
        self.max_buys
    }

    /// Get the name of the upgrade
    fn get_name(&self) -> &str {
        &self.name
    }

    /// Get the description of the upgrade
    fn get_description(&self) -> &str {
        &self.description
    }

    /// Update the upgrade properties after buying
    fn buy(&mut self) -> f64 {
        //Make sure the upgrade can be purchased
        if self.num_buys > self.max_buys {
            panic!("Cannot buy upgrade: {}", self.name);
        }

        //Calculate the cost before incrementing the num buys
        let cost: f64 = self.base_cost + f64::powf(self.base_coef, self.num_buys as f64);

        //If it can be purchased, then update the number of buys
        self.num_buys += 1;

        //Return the cost
        cost
    }
}