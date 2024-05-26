// main.rs

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

mod stock_option;
mod binomial_tree_option;
mod binomial_lr_option;
mod binomial_lr_with_greeks;

use stock_option::StockOption;
use binomial_tree_option::BinomialTreeOption;
use binomial_lr_option::BinomialLROption;
use binomial_lr_with_greeks::BinomialLRWithGreeks;

/// Calculates the option price and Greeks using the binomial LR (Leisen-Reimer) model.
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
/// * `options_type` - The type of the option, either "call" or "put".
/// * `is_am` - A boolean indicating whether the option is American-style (true) or European-style (false).
///
/// # Returns
///
/// A tuple containing the following values:
/// - `option_price`: The calculated option price.
/// - `delta`: The option's delta (rate of change of option price with respect to the underlying asset price).
/// - `gamma`: The option's gamma (rate of change of delta with respect to the underlying asset price).
/// - `theta`: The option's theta (rate of change of option price with respect to time).
/// - `vega`: The option's vega (sensitivity of option price to changes in volatility).
/// - `rho`: The option's rho (sensitivity of option price to changes in the risk-free interest rate).
///
/// # Errors
///
/// Returns a `PyValueError` if the `options_type` is not "call" or "put".
#[pyfunction]
fn calculate_option_price_and_greeks(
    s0: f64,
    k: f64,
    r: f64,
    t: f64,
    n: usize,
    pu: f64,
    pd: f64,
    div: f64,
    sigma: f64,
    options_type: &str,
    is_am: bool,
) -> PyResult<(f64, f64, f64, f64, f64, f64)> {
    let is_put = match options_type {
        "call" => false,
        "put" => true,
        _ => return Err(PyValueError::new_err("Invalid options_type. Must be 'call' or 'put'.")),
    };

    let stock_option = StockOption::new(s0, k, r, t, n, pu, pd, div, sigma, is_put, is_am);
    let binomial_tree_option = BinomialTreeOption::new(stock_option);
    let binomial_lr_option = BinomialLROption::new(binomial_tree_option);
    let mut binomial_lr_with_greeks = BinomialLRWithGreeks::new(binomial_lr_option);

    Ok(binomial_lr_with_greeks.price())
}

/// The Python module definition for the Rust library.
///
/// This function defines the name of the Python module and the functions exposed to Python.
#[pymodule]
fn libnumerical_options_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_option_price_and_greeks, m)?)?;
    Ok(())
}
