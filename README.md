# Numerical Options Pricing in Rust

This project is an implementation of various numerical options pricing models in Rust. It provides a comprehensive suite of tools for pricing financial options using popular numerical methods such as the binomial tree model and the Leisen-Reimer tree model.

The project is designed to be modular, efficient, and easy to use. It consists of several key components, including the `StockOption` struct, which represents the underlying stock option and its associated parameters, and the `BinomialTreeOption` struct, which implements the binomial tree option pricing model.

The binomial tree model is a widely used numerical method for pricing options. It discretizes the time to expiration of the option into a number of time steps and models the evolution of the stock price as a binomial tree. At each node of the tree, the stock price can move up or down by a certain factor, and the option price is calculated by working backward through the tree from the terminal payoffs.

In addition to the basic binomial tree model, this project also includes an implementation of the Leisen-Reimer tree model, which is an improvement over the standard binomial tree model. The Leisen-Reimer model uses a modified tree structure and probability calculations to achieve better accuracy and convergence properties.

The project also provides functionality for calculating various option price sensitivities, known as the "Greeks." These include delta (the sensitivity of the option price to changes in the underlying stock price), gamma (the sensitivity of delta to changes in the stock price), theta (the sensitivity of the option price to changes in time), vega (the sensitivity of the option price to changes in volatility), and rho (the sensitivity of the option price to changes in the risk-free interest rate).

The code is thoroughly documented using Rust's documentation comments, making it easy for users to understand and utilize the various components of the project. The documentation provides clear explanations of the structs, methods, and their purposes, along with details on input parameters and return values.

This project is suitable for a wide range of users, including financial professionals, researchers, and students interested in options pricing and numerical methods. It can be used as a educational tool to learn about options pricing models and their implementation in Rust, as well as a practical tool for pricing options in real-world scenarios.

Whether you are new to options pricing or an experienced practitioner, this project provides a robust and efficient framework for numerical options pricing in Rust. The modular design and comprehensive documentation make it easy to integrate into your own projects or to extend with new pricing models and features.