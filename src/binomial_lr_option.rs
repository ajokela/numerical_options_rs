// binomial_lr_option.rs

use core::f64::consts::E;
use crate::binomial_tree_option::BinomialTreeOption;

/// Represents a binomial LR (Leisen-Reimer) option pricing model.
///
/// The Leisen-Reimer model is a modification of the binomial tree option pricing model
/// that improves the convergence speed and accuracy of the option price calculation.
///
/// # Example
///
/// ```
/// use binomial_lr_option::BinomialLROption;
/// use binomial_tree_option::BinomialTreeOption;
///
/// let option = BinomialTreeOption::new(...);
/// let mut lr_option = BinomialLROption::new(option);
/// lr_option.setup_parameters();
/// let price = lr_option.tree.calculate_price();
/// ```
pub struct BinomialLROption {
    /// The underlying binomial tree option.
    pub tree: BinomialTreeOption,
    /// The probability parameter used in the Leisen-Reimer model.
    ///
    /// This parameter is calculated based on the option parameters and is used to
    /// determine the up and down move probabilities in the binomial tree.
    pub p: f64,
}

impl BinomialLROption {
    /// Creates a new `BinomialLROption` with the given binomial tree option.
    ///
    /// The `p` parameter is initialized to 0.0 and will be calculated later using
    /// the `setup_parameters` method.
    ///
    /// # Arguments
    ///
    /// * `tree` - The binomial tree option representing the underlying asset and option parameters.
    pub fn new(tree: BinomialTreeOption) -> Self {
        BinomialLROption { tree, p: 0.0 }
    }

    /// Sets up the parameters for the binomial LR option pricing model.
    ///
    /// This method calculates the values of `p`, `u`, `d`, `qu`, and `qd` based on the
    /// option parameters stored in the `tree` field. These parameters are used to determine
    /// the probabilities and move factors in the binomial tree.
    ///
    /// The calculations are based on the Leisen-Reimer model, which uses a modified version
    /// of the Cox-Ross-Rubinstein (CRR) binomial tree model.
    pub fn setup_parameters(&mut self) {
        let odd_n = if self.tree.option.n % 2 == 0 {
            self.tree.option.n
        } else {
            self.tree.option.n + 1
        };

        let d1 = (self.tree.option.s0 / self.tree.option.k).ln()
            + ((self.tree.option.r - self.tree.option.div + (self.tree.option.sigma.powi(2) / 2.0))
                * self.tree.option.t)
                / (self.tree.option.sigma * self.tree.option.t.sqrt());

        let d2 = (self.tree.option.s0 / self.tree.option.k).ln()
            + ((self.tree.option.r - self.tree.option.div - (self.tree.option.sigma.powi(2) / 2.0))
                * self.tree.option.t)
                / (self.tree.option.sigma * self.tree.option.t.sqrt());

        let pbar = self.pp_2_inversion(d1, odd_n);
        self.p = self.pp_2_inversion(d2, odd_n);

        self.tree.u = 1.0 / self.tree.option.df() * pbar / self.p;
        self.tree.d = (1.0 / self.tree.option.df() - self.p * self.tree.u) / (1.0 - self.p);
        self.tree.qu = self.p;
        self.tree.qd = 1.0 - self.p;
    }

    /// Calculates the pp 2 inversion used in the Leisen-Reimer model.
    ///
    /// This function is a helper method used in the `setup_parameters` method to calculate
    /// the values of `p` and `pbar`. It approximates the inverse of the cumulative standard
    /// normal distribution function using a modified version of the Beasley-Springer-Moro algorithm.
    ///
    /// # Arguments
    ///
    /// * `z` - The z-score (number of standard deviations from the mean).
    /// * `n` - The number of periods in the binomial tree.
    ///
    /// # Returns
    ///
    /// The approximate value of the cumulative standard normal distribution function at `z`.
    fn pp_2_inversion(&self, z: f64, n: usize) -> f64 {
        let n = n as f64;
        let p = 0.5
            + z.signum()
                * (0.25
                    - 0.25
                        * E.powf(
                            -1.0 * ((z / (n + 1.0 / 3.0 + 0.1 / (n + 1.0))).powi(2))
                                * (n + 1.0 / 6.0),
                        ))
                .sqrt();

        if p.is_nan() {
            if z < 0.0 {
                0.0
            } else {
                1.0
            }
        } else {
            p
        }
    }
}
