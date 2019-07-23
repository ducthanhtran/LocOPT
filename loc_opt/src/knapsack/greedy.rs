use knapsack::utils::KnapsackResult;

pub fn run(profits: &Vec<f64>, weights: &Vec<f64>, capacity: &f64) -> KnapsackResult {
    assert!(profits.len() == weights.len());
    
    // Compute quotients p_i / w_i and sort these.
    let mut quotients = Vec::with_capacity(profits.len());
    let iter = profits.iter().zip(weights.iter());
    for (j, (profit, weight)) in iter.enumerate() {
        quotients.push((j, profit / weight));
    }
    quotients.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());

    // Select items with highest quotient value until capacity is reached.
    let mut selected_items: Vec<usize> = Vec::new();
    let mut current_weight: f64 = 0.0;
    let mut current_profit: f64 = 0.0;
    let mut last_index: usize = 0;
    for (index, q) in quotients.iter().rev().enumerate() {
        if current_weight + weights[q.0] < *capacity {
            selected_items.push(q.0);
            current_weight += weights[q.0];
            current_profit += profits[q.0];
        } else {
            last_index = index;
            break;
        }
    }

    // Check whether next item fits capacity and yields better profits. This
    // guarantees the 2-approximation.
    if weights[last_index] < *capacity && profits[last_index] > current_profit {
        selected_items.clear();
        selected_items.push(last_index);
        current_weight = weights[last_index];
        current_profit = profits[last_index];
    }

    return KnapsackResult {
        selected_items,
        weight: current_weight,
        profit: current_profit
    };
}
