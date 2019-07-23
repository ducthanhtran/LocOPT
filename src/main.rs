extern crate loc_opt;

fn main() {
    println!("A");
    
    loc_opt::run();

    let profits = vec![20.0, 3.0, 5.0, 3.0];
    let weights = vec![5.0, 3.0, 10.0, 0.3];
    let capacity: f64 = 10.0;
    let result = loc_opt::knapsack::greedy::run(&profits, &weights, &capacity);
    println!("profit {}, weight {}", result.profit, result.weight);
    println!("{:?}", result.selected_items);
}