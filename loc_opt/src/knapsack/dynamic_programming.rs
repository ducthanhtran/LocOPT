use knapsack::utils::KnapsackResult;

pub fn run(profits: &Vec<f64>, weights: &Vec<f64>, capacity: &f64) -> KnapsackResult{
    assert!(profits.len() == weights.len());

    // Selection of largest boundary: either total profit or capacity. Thus we
    // compute the solution in O(Bn) where B = min(total_profit, capacity)
    let total_profit: f64 profits.iter().sum();
    if total_profit > *capacity {
        return _run_profit_based(profits, weights, capacity)
    } else {
        return _run_weight_based(profits, weights, capacity)
    }
}

fn _run_profit_based(profits: &Vec<f64>, weights &Vec<f64>, capacity: &f64) {

}

fn _run_weight_based(profits: &Vec<f64>, weights &Vec<f64>, capacity: &f64) {

}