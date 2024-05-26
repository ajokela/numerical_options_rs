// binomial_lr_with_greeks.rs

use crate::binomial_lr_option::BinomialLROption;

/// Represents a binomial LR (Leisen-Reimer) option with Greeks calculation.
///
/// This struct extends the `BinomialLROption` to include the calculation of option Greeks,
/// such as delta, gamma, theta, vega, and rho.
pub struct BinomialLRWithGreeks {
    /// The underlying binomial LR option.
    pub lr_option: BinomialLROption,
}

impl BinomialLRWithGreeks {
    /// Creates a new `BinomialLRWithGreeks` instance with the given `BinomialLROption`.
    ///
    /// # Arguments
    ///
    /// * `lr_option` - The binomial LR option to be used for Greeks calculation.
    pub fn new(lr_option: BinomialLROption) -> Self {
        BinomialLRWithGreeks { lr_option }
    }

    /// Generates a new stock price tree based on the binomial LR option parameters.
    ///
    /// This method calculates the stock prices at each node of the binomial tree using
    /// the up and down factors from the binomial LR option.
    fn new_stock_price_tree(&mut self) {
        let u_over_d = self.lr_option.tree.u / self.lr_option.tree.d;
        let d_over_u = self.lr_option.tree.d / self.lr_option.tree.u;

        self.lr_option.tree.option.sts = vec![vec![
            self.lr_option.tree.option.s0 * u_over_d,
            self.lr_option.tree.option.s0,
            self.lr_option.tree.option.s0 * d_over_u,
        ]];

        for _ in 0..self.lr_option.tree.option.n {
            let prev_branches = &self.lr_option.tree.option.sts[self.lr_option.tree.option.sts.len() - 1];
            let mut st = prev_branches
                .iter()
                .map(|&x| x * self.lr_option.tree.u)
                .collect::<Vec<_>>();
            st.push(prev_branches[prev_branches.len() - 1] * self.lr_option.tree.d);
            self.lr_option.tree.option.sts.push(st);
        }
    }

    /// Calculates the option price and Greeks (delta, gamma, theta, vega, rho).
    ///
    /// This method first sets up the binomial LR option parameters and generates the stock price tree.
    /// It then calculates the option payoffs using the `begin_tree_traversal` method from the binomial LR option.
    /// Finally, it computes the option price and various Greeks based on the calculated payoffs and stock prices.
    ///
    /// # Returns
    ///
    /// A tuple containing the following values:
    /// - `option_value`: The calculated option price.
    /// - `delta`: The option's delta (rate of change of option price with respect to the underlying asset price).
    /// - `gamma`: The option's gamma (rate of change of delta with respect to the underlying asset price).
    /// - `theta`: The option's theta (rate of change of option price with respect to time).
    /// - `vega`: The option's vega (sensitivity of option price to changes in volatility).
    /// - `rho`: The option's rho (sensitivity of option price to changes in the risk-free interest rate).
    pub fn price(&mut self) -> (f64, f64, f64, f64, f64, f64) {
        self.lr_option.setup_parameters();
        self.new_stock_price_tree();

        let payoffs = self.lr_option.tree.begin_tree_traversal();
        let option_value = payoffs[payoffs.len() / 2];
        let payoff_up = payoffs[0];
        let payoff_down = payoffs[payoffs.len() - 1];

        let s_up = self.lr_option.tree.option.sts[0][0];
        let s_down = self.lr_option.tree.option.sts[0][2];

        let ds_up = s_up - self.lr_option.tree.option.s0;
        let ds_down = self.lr_option.tree.option.s0 - s_down;
        let ds = s_up - s_down;
        let dv = payoff_up - payoff_down;

        // Calculate delta as the change in option value divided by the change in stock price
        let delta = dv / ds;

        // Calculate gamma as the change in delta divided by the change in stock price
        let gamma = ((payoff_up - option_value) / ds_up - (option_value - payoff_down) / ds_down)
            / ((self.lr_option.tree.option.s0 + s_up) / 2.0 - (self.lr_option.tree.option.s0 + s_down) / 2.0);

        let dt = 0.0001; // Small perturbation in time
        let original_t = self.lr_option.tree.option.t;
        self.lr_option.tree.option.t -= dt;
        self.lr_option.setup_parameters();
        let payoffs_theta = self.lr_option.tree.begin_tree_traversal();
        let option_value_theta = payoffs_theta[payoffs_theta.len() / 2];
        
        // Calculate theta as the negative of the change in option value divided by the change in time
        let theta = -(option_value_theta - option_value) / dt;
        self.lr_option.tree.option.t = original_t;

        let dv = 0.01;
        self.lr_option.tree.option.sigma += dv;
        self.lr_option.setup_parameters();
        let payoffs_vega = self.lr_option.tree.begin_tree_traversal();
        let option_value_vega = payoffs_vega[payoffs_vega.len() / 2];
        
        // Calculate vega as the change in option value divided by the change in volatility
        let vega = (option_value_vega - option_value) / dv;
        self.lr_option.tree.option.sigma -= dv;

        let dr = 0.01;
        self.lr_option.tree.option.r += dr;
        self.lr_option.setup_parameters();
        let payoffs_rho = self.lr_option.tree.begin_tree_traversal();
        let option_value_rho = payoffs_rho[payoffs_rho.len() / 2];
        
        // Calculate rho as the change in option value divided by the change in interest rate
        let rho = (option_value_rho - option_value) / dr;
        self.lr_option.tree.option.r -= dr;

        (option_value, delta, gamma, theta, vega, rho)
    }
}