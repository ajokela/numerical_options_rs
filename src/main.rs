use std::f64::consts::E;

struct StockOption {
    s0: f64,
    k: f64,
    r: f64,
    t: f64,
    n: usize,
    sts: Vec<Vec<f64>>,
    pu: f64,
    pd: f64,
    div: f64,
    sigma: f64,
    is_call: bool,
    is_european: bool,
}

impl StockOption {
    fn new(
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

    fn dt(&self) -> f64 {
        self.t / self.n as f64
    }

    fn df(&self) -> f64 {
        E.powf(-1.0 * (self.r - self.div) * self.dt())
    }
}

struct BinomialTreeOption {
    option: StockOption,
    u: f64,
    d: f64,
    qu: f64,
    qd: f64,
}

impl BinomialTreeOption {
    fn new(option: StockOption) -> Self {
        BinomialTreeOption {
            option,
            u: 0.0,
            d: 0.0,
            qu: 0.0,
            qd: 0.0,
        }
    }

    fn setup_parameters(&mut self) {
        self.u = 1.0 + self.option.pu;
        self.d = 1.0 - self.option.pd;
        self.qu = (E.powf((self.option.r - self.option.div) * self.option.dt()) - self.d)
            / (self.u - self.d);
        self.qd = 1.0 - self.qu;
    }

    fn init_stock_price_tree(&mut self) {
        self.option.sts = vec![vec![self.option.s0]];

        for _ in 0..self.option.n {
            let prev_branches = &self.option.sts[self.option.sts.len() - 1];
            let mut st = prev_branches
                .iter()
                .map(|&x| x * self.u)
                .collect::<Vec<_>>();
            st.push(prev_branches[prev_branches.len() - 1] * self.d);
            self.option.sts.push(st);
        }
    }

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

    fn begin_tree_traversal(&mut self) -> Vec<f64> {
        let payoffs = self.init_payoffs_tree();
        self.traverse_tree(payoffs)
    }

    fn price(&mut self) -> f64 {
        self.setup_parameters();
        self.init_stock_price_tree();
        let payoffs = self.begin_tree_traversal();
        payoffs[0]
    }
}

struct BinomialLROption {
    tree: BinomialTreeOption,
    p: f64,
}

impl BinomialLROption {
    fn new(tree: BinomialTreeOption) -> Self {
        BinomialLROption { tree, p: 0.0 }
    }

    fn setup_parameters(&mut self) {
        let odd_n = if self.tree.option.n % 2 == 0 {
            self.tree.option.n
        } else {
            self.tree.option.n + 1
        };

        let d1 = (self.tree.option.s0 / self.tree.option.k).ln()
            + ((self.tree.option.r
                - self.tree.option.div
                + (self.tree.option.sigma.powi(2) / 2.0))
                * self.tree.option.t)
                / (self.tree.option.sigma * self.tree.option.t.sqrt());

        let d2 = (self.tree.option.s0 / self.tree.option.k).ln()
            + ((self.tree.option.r
                - self.tree.option.div
                - (self.tree.option.sigma.powi(2) / 2.0))
                * self.tree.option.t)
                / (self.tree.option.sigma * self.tree.option.t.sqrt());

        let pbar = self.pp_2_inversion(d1, odd_n);
        self.p = self.pp_2_inversion(d2, odd_n);

        self.tree.u = 1.0 / self.tree.option.df() * pbar / self.p;
        self.tree.d = (1.0 / self.tree.option.df() - self.p * self.tree.u) / (1.0 - self.p);
        self.tree.qu = self.p;
        self.tree.qd = 1.0 - self.p;
    }

    fn pp_2_inversion(&self, z: f64, n: usize) -> f64 {
        let n = n as f64;
        let p = 0.5 + z.signum()
            * (0.25
                - 0.25
                    * E.powf(
                        -1.0
                            * ((z / (n + 1.0 / 3.0 + 0.1 / (n + 1.0))).powi(2))
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

struct BinomialLRWithGreeks {
    lr_option: BinomialLROption,
}

impl BinomialLRWithGreeks {
    fn new(lr_option: BinomialLROption) -> Self {
        BinomialLRWithGreeks { lr_option }
    }

    fn new_stock_price_tree(&mut self) {
        let u_over_d = self.lr_option.tree.u / self.lr_option.tree.d;
        let d_over_u = self.lr_option.tree.d / self.lr_option.tree.u;

        self.lr_option.tree.option.sts = vec![vec![
            self.lr_option.tree.option.s0 * u_over_d,
            self.lr_option.tree.option.s0,
            self.lr_option.tree.option.s0 * d_over_u,
        ]];

        for _ in 0..self.lr_option.tree.option.n {
            let prev_branches = &self.lr_option.tree.option.sts
                [self.lr_option.tree.option.sts.len() - 1];
            let mut st = prev_branches
                .iter()
                .map(|&x| x * self.lr_option.tree.u)
                .collect::<Vec<_>>();
            st.push(prev_branches[prev_branches.len() - 1] * self.lr_option.tree.d);
            self.lr_option.tree.option.sts.push(st);
        }
    }

    fn price(&mut self) -> (f64, f64, f64, f64, f64, f64) {
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

        let delta = dv / ds;

        let gamma = ((payoff_up - option_value) / ds_up
            - (option_value - payoff_down) / ds_down)
            / ((self.lr_option.tree.option.s0 + s_up) / 2.0
                - (self.lr_option.tree.option.s0 + s_down) / 2.0);

        let dt = self.lr_option.tree.option.t / self.lr_option.tree.option.n as f64;
        let theta = (payoffs[1] - option_value) / dt;

        let dv = 0.01;
        self.lr_option.tree.option.sigma += dv;
        self.lr_option.setup_parameters();
        let payoffs_vega = self.lr_option.tree.begin_tree_traversal();
        let option_value_vega = payoffs_vega[payoffs_vega.len() / 2];
        let vega = (option_value_vega - option_value) / dv;
        self.lr_option.tree.option.sigma -= dv;

        let dr = 0.01;
        self.lr_option.tree.option.r += dr;
        self.lr_option.setup_parameters();
        let payoffs_rho = self.lr_option.tree.begin_tree_traversal();
        let option_value_rho = payoffs_rho[payoffs_rho.len() / 2];
        let rho = (option_value_rho - option_value) / dr;
        self.lr_option.tree.option.r -= dr;

        (option_value, delta, gamma, theta, vega, rho)
    }
}

fn main() {
    // Example usage
    let stock_option = StockOption::new(50.0, 52.0, 0.05, 2.0, 300, 0.0, 0.0, 0.0, 0.3, false, false);
    let mut binomial_tree_option = BinomialTreeOption::new(stock_option);
    let price = binomial_tree_option.price();
    println!("Option price: {}", price);

    let binomial_lr_option = BinomialLROption::new(binomial_tree_option);
    let mut binomial_lr_with_greeks = BinomialLRWithGreeks::new(binomial_lr_option);
    let (price, delta, gamma, theta, vega, rho) = binomial_lr_with_greeks.price();
    println!("Option price: {}", price);
    println!("Delta: {}", delta);
    println!("Gamma: {}", gamma);
    println!("Theta: {}", theta);
    println!("Vega: {}", vega);
    println!("Rho: {}", rho);
}

