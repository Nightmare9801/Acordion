pub(crate) struct GameTree {
    positions: Vec<Vec<f64>>,
    rewards: Vec<i8>,
}

impl GameTree {
    
    /// The function creates a new instance of a GameTree struct with given positions and rewards.
    /// 
    /// Arguments:
    /// 
    /// * `positions`: The `positions` parameter is a vector of vectors of type `f64`, representing the
    /// positions in the game tree. Each inner vector contains the coordinates of a position in the game
    /// tree.
    /// * `rewards`: The `rewards` parameter in the `new` function is a vector of integers representing
    /// the rewards associated with each position in the game tree.
    /// 
    /// Returns:
    /// 
    /// A new instance of the `GameTree` struct with the provided `positions` and `rewards` vectors is
    /// being returned.
    pub fn new(positions: Vec<Vec<f64>>, rewards: Vec<i8>) -> Self {
        GameTree { positions, rewards }
    }

    /// The function `add_position` adds a new position to a mutable vector of positions.
    /// 
    /// Arguments:
    /// 
    /// * `new_position`: A vector of floating-point numbers representing the new position to be added
    /// to the existing positions.
    pub fn add_position(&mut self, new_position: Vec<f64>) {
        self.positions.push(new_position);
    }

    /// The function `add_reward` adds a new reward to a list of rewards.
    /// 
    /// Arguments:
    /// 
    /// * `new_reward`: The `new_reward` parameter is of type `i8`, which is a signed 8-bit integer in
    /// Rust.
    pub fn add_reward(&mut self, new_reward: i8) {
        self.rewards.push(new_reward);
    }

    /// The function `get_rewards` returns a cloned vector of rewards.
    /// 
    /// Returns:
    /// 
    /// A vector of signed 8-bit integers (Vec<i8>) containing the rewards.
    pub fn get_rewards(&self) -> Vec<i8> {
        return self.rewards.clone();
    }

    /// The function `get_positions` returns a clone of the positions stored in a vector of vectors of
    /// f64 values.
    /// 
    /// Returns:
    /// 
    /// A vector of vectors of type f64 containing positions is being returned.
    pub fn get_positions(&self) -> Vec<Vec<f64>> {
        return self.positions.clone();
    }
}