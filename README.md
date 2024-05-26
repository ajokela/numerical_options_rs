# Numerical Options Pricing in Rust

This project is an implementation of various numerical options pricing models in Rust. It provides a comprehensive suite of tools for pricing financial options using popular numerical methods such as the binomial tree model and the Leisen-Reimer tree model.

The project is designed to be modular, efficient, and easy to use. It consists of several key components, including the `StockOption` struct, which represents the underlying stock option and its associated parameters, and the `BinomialTreeOption` struct, which implements the binomial tree option pricing model.

The binomial tree model is a widely used numerical method for pricing options. It discretizes the time to expiration of the option into a number of time steps and models the evolution of the stock price as a binomial tree. At each node of the tree, the stock price can move up or down by a certain factor, and the option price is calculated by working backward through the tree from the terminal payoffs.

In addition to the basic binomial tree model, this project also includes an implementation of the Leisen-Reimer tree model, which is an improvement over the standard binomial tree model. The Leisen-Reimer model uses a modified tree structure and probability calculations to achieve better accuracy and convergence properties.

The project also provides functionality for calculating various option price sensitivities, known as the "Greeks." These include delta (the sensitivity of the option price to changes in the underlying stock price), gamma (the sensitivity of delta to changes in the stock price), theta (the sensitivity of the option price to changes in time), vega (the sensitivity of the option price to changes in volatility), and rho (the sensitivity of the option price to changes in the risk-free interest rate).

The code is thoroughly documented using Rust's documentation comments, making it easy for users to understand and utilize the various components of the project. The documentation provides clear explanations of the structs, methods, and their purposes, along with details on input parameters and return values.

This project is suitable for a wide range of users, including financial professionals, researchers, and students interested in options pricing and numerical methods. It can be used as a educational tool to learn about options pricing models and their implementation in Rust, as well as a practical tool for pricing options in real-world scenarios.

## Importing the Library

To use the Numerical Options Pricing Rust library in your Python code, you need to import the `numerical_options_rs` module:

```python
import numerical_options_rs
```

## Calculating Option Price and Greeks

The library provides a function called `calculate_option_price_and_greeks` that calculates the option price and Greeks using the binomial LR (Leisen-Reimer) model. Here's how you can use it:

```python
option_price, delta, gamma, theta, vega, rho = numerical_options_rs.calculate_option_price_and_greeks(
    s0, k, r, t, n, pu, pd, div, sigma, options_type, is_am
)
```

### Parameters

The `calculate_option_price_and_greeks` function takes the following parameters:

- `s0` (float): The initial stock price.
- `k` (float): The strike price of the option.
- `r` (float): The risk-free interest rate.
- `t` (float): The time to expiration of the option (in years).
- `n` (int): The number of time steps in the binomial tree.
- `pu` (float): The probability of an up move in the binomial tree.
- `pd` (float): The probability of a down move in the binomial tree.
- `div` (float): The continuous dividend yield of the underlying asset.
- `sigma` (float): The volatility of the underlying asset.
- `options_type` (str): The type of the option, either "call" or "put".
- `is_am` (bool): A boolean indicating whether the option is American-style (True) or European-style (False).

### Return Value

The `calculate_option_price_and_greeks` function returns a tuple containing the following values:

- `option_price` (float): The calculated option price.
- `delta` (float): The option's delta (rate of change of option price with respect to the underlying asset price).
- `gamma` (float): The option's gamma (rate of change of delta with respect to the underlying asset price).
- `theta` (float): The option's theta (rate of change of option price with respect to time).
- `vega` (float): The option's vega (sensitivity of option price to changes in volatility).
- `rho` (float): The option's rho (sensitivity of option price to changes in the risk-free interest rate).

### Error Handling

If the `options_type` parameter is not set to either "call" or "put", the function will raise a `PyValueError` exception.

## Example Usage

Here's an example of how to use the `calculate_option_price_and_greeks` function:

```python
import numerical_options_rs

s0 = 100.0
k = 110.0
r = 0.05
t = 1.0
n = 100
pu = 0.2
pd = 0.2
div = 0.0
sigma = 0.3
options_type = "call"
is_am = False

option_price, delta, gamma, theta, vega, rho = numerical_options_rs.calculate_option_price_and_greeks(
    s0, k, r, t, n, pu, pd, div, sigma, options_type, is_am
)

print(f"Option Price: {option_price:.4f}")
print(f"Delta: {delta:.4f}")
print(f"Gamma: {gamma:.4f}")
print(f"Theta: {theta:.4f}")
print(f"Vega: {vega:.4f}")
print(f"Rho: {rho:.4f}")
```

This example calculates the option price and Greeks for a European call option with the given parameters. The results are then printed to the console.

