// stock_option.rs

use std::f64::consts::E;

/// Represents a stock option with its associated parameters.
pub struct StockOption {
    /// The initial stock price.
    pub s0: f64,
    /// The strike price of the option.
    pub k: f64,
    /// The risk-free interest rate.
    pub r: f64,
    /// The time to expiration of the option (in years).
    pub t: f64,
    /// The number of time steps in the binomial tree.
    pub n: usize,
    /// The stock price tree.
    pub sts: Vec<Vec<f64>>,
    /// The probability of an up move in the binomial tree.
    pub pu: f64,
    /// The probability of a down move in the binomial tree.
    pub pd: f64,
    /// The continuous dividend yield of the underlying asset.
    pub div: f64,
    /// The volatility of the underlying asset.
    pub sigma: f64,
    /// A boolean indicating whether the option is a call (true) or a put (false).
    pub is_call: bool,
    /// A boolean indicating whether the option is European-style (true) or American-style (false).
    pub is_european: bool,
}

impl StockOption {
    /// Creates a new `StockOption` instance with the given parameters.
    ///
    /// # Arguments
    ///
    /// * `s0` - The initial stock price.
    /// * `k` - The strike price of the option.
    /// * `r` - The risk-free interest rate.
    /// * `t` - The time to expiration of the option (in years).
    /// * `n` - The number of time steps in the binomial tree.
    /// * `pu` - The probability of an up move in the binomial tree.
    /// * `pd` - The probability of a down move in the binomial tree.
    /// * `div` - The continuous dividend yield of the underlying asset.
    /// * `sigma` - The volatility of the underlying asset.
    /// * `is_put` - A boolean indicating whether the option is a put (true) or a call (false).
    /// * `is_am` - A boolean indicating whether the option is American-style (true) or European-style (false).
    ///
    /// # Returns
    ///
    /// A new `StockOption` instance with the provided parameters.
    pub fn new(
        s0: f64,
        k: f64,
        r: f64,
        t: f64,
        n: usize,
        pu: f64,
        pd: f64,
        div: f64,
        sigma: f64,
        is_put: bool,
        is_am: bool,
    ) -> Self {
        StockOption {
            s0,
            k,
            r,
            t,
            n: n.max(1),
            sts: Vec::new(),
            pu,
            pd,
            div,
            sigma,
            is_call: !is_put,
            is_european: !is_am,
        }
    }

    /// Calculates the time step size (Δt) of the binomial tree.
    ///
    /// # Returns
    ///
    /// The time step size (Δt) of the binomial tree.
    pub fn dt(&self) -> f64 {
        self.t / self.n as f64
    }

    /// Calculates the discount factor for each time step.
    ///
    /// The discount factor is calculated using the risk-free interest rate and the dividend yield.
    ///
    /// # Returns
    ///
    /// The discount factor for each time step.
    pub fn df(&self) -> f64 {
        E.powf(-1.0 * (self.r - self.div) * self.dt())
    }
}