// binomial_tree_option.rs

use core::f64::consts::E;
use crate::stock_option::StockOption;

/// Represents a binomial tree option pricing model.
pub struct BinomialTreeOption {
    /// The underlying stock option.
    pub option: StockOption,
    /// The up factor in the binomial tree.
    pub u: f64,
    /// The down factor in the binomial tree.
    pub d: f64,
    /// The risk-neutral probability of an up move.
    pub qu: f64,
    /// The risk-neutral probability of a down move.
    pub qd: f64,
}

impl BinomialTreeOption {
    /// Creates a new `BinomialTreeOption` instance with the given stock option.
    ///
    /// # Arguments
    ///
    /// * `option` - The underlying stock option.
    ///
    /// # Returns
    ///
    /// A new `BinomialTreeOption` instance.
    pub fn new(option: StockOption) -> Self {
        BinomialTreeOption {
            option,
            u: 0.0,
            d: 0.0,
            qu: 0.0,
            qd: 0.0,
        }
    }

    /// Sets up the parameters for the binomial tree option pricing model.
    ///
    /// This method calculates the up factor, down factor, and risk-neutral probabilities
    /// based on the underlying stock option parameters.
    #[allow(dead_code)]
    pub fn setup_parameters(&mut self) {
        self.u = 1.0 + self.option.pu;
        self.d = 1.0 - self.option.pd;
        self.qu = (E.powf((self.option.r - self.option.div) * self.option.dt()) - self.d)
            / (self.u - self.d);
        self.qd = 1.0 - self.qu;
    }

    /// Initializes the stock price tree for the binomial option pricing model.
    ///
    /// This method constructs the stock price tree based on the initial stock price,
    /// up factor, and down factor.
    #[allow(dead_code)]
    fn init_stock_price_tree(&mut self) {
        self.option.sts = vec![vec![self.option.s0]];
        for _ in 0..self.option.n {
            let prev_branches = &self.option.sts[self.option.sts.len() - 1];
            let mut st = prev_branches.iter().map(|&x| x * self.u).collect::<Vec<_>>();
            st.push(prev_branches[prev_branches.len() - 1] * self.d);
            self.option.sts.push(st);
        }
    }

    /// Initializes the payoff tree for the binomial option pricing model.
    ///
    /// This method calculates the payoffs at the terminal nodes of the binomial tree
    /// based on the stock prices and the option type (call or put).
    ///
    /// # Returns
    ///
    /// A vector containing the payoffs at the terminal nodes of the binomial tree.
    fn init_payoffs_tree(&self) -> Vec<f64> {
        if self.option.is_call {
            self.option.sts[self.option.n]
                .iter()
                .map(|&x| (x - self.option.k).max(0.0))
                .collect()
        } else {
            self.option.sts[self.option.n]
                .iter()
                .map(|&x| (self.option.k - x).max(0.0))
                .collect()
        }
    }

    /// Checks for early exercise opportunity at a given node in the binomial tree.
    ///
    /// This method compares the payoffs with the intrinsic values at a given node
    /// and returns the updated payoffs after considering early exercise.
    ///
    /// # Arguments
    ///
    /// * `payoffs` - The payoffs at the current node.
    /// * `node` - The index of the current node in the binomial tree.
    ///
    /// # Returns
    ///
    /// A vector containing the updated payoffs after considering early exercise.
    fn check_early_exercise(&self, payoffs: &[f64], node: usize) -> Vec<f64> {
        if self.option.is_call {
            payoffs
                .iter()
                .zip(self.option.sts[node].iter())
                .map(|(&p, &s)| p.max(s - self.option.k))
                .collect()
        } else {
            payoffs
                .iter()
                .zip(self.option.sts[node].iter())
                .map(|(&p, &s)| p.max(self.option.k - s))
                .collect()
        }
    }

    /// Traverses the binomial tree backward to calculate the option price.
    ///
    /// This method starts from the terminal payoffs and works backward through the tree,
    /// calculating the option price at each node based on the risk-neutral probabilities
    /// and the discount factor. It also checks for early exercise opportunities if the
    /// option is American-style.
    ///
    /// # Arguments
    ///
    /// * `payoffs` - The payoffs at the terminal nodes of the binomial tree.
    ///
    /// # Returns
    ///
    /// A vector containing the option prices at each node of the binomial tree.
    fn traverse_tree(&self, mut payoffs: Vec<f64>) -> Vec<f64> {
        for i in (0..self.option.n).rev() {
            payoffs = payoffs
                .windows(2)
                .map(|w| (w[0] * self.qu + w[1] * self.qd) * self.option.df())
                .collect();
            if !self.option.is_european {
                payoffs = self.check_early_exercise(&payoffs, i);
            }
        }
        payoffs
    }

    /// Begins the traversal of the binomial tree to calculate the option price.
    ///
    /// This method initializes the payoffs at the terminal nodes and then traverses
    /// the tree backward to calculate the option price at each node.
    ///
    /// # Returns
    ///
    /// A vector containing the option prices at each node of the binomial tree.
    pub fn begin_tree_traversal(&mut self) -> Vec<f64> {
        let payoffs = self.init_payoffs_tree();
        self.traverse_tree(payoffs)
    }

    /// Calculates the price of the option using the binomial tree model.
    ///
    /// This method sets up the parameters, initializes the stock price tree,
    /// and traverses the tree to calculate the option price.
    ///
    /// # Returns
    ///
    /// The calculated price of the option.
    #[allow(dead_code)]
    pub fn price(&mut self) -> f64 {
        self.setup_parameters();
        self.init_stock_price_tree();
        let payoffs = self.begin_tree_traversal();
        payoffs[0]
    }
}
