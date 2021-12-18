enum Chain {
    Ethereum,
    Solana,
    Binance,
}

struct ChainPerformance {
    transactionpersecond: Chain,
    performance: f64,
}

fn get_chain_details(_chain:ChainPerformance) {

match _chain.transactionpersecond {
    Chain::Ethereum => println!("Ethereum"),
    Chain::Solana => println!("Solana"),
    Chain::Binance => println!("Binance"),
}
println!("{:?}",_chain.performance);
}

fn main() {

    let ethereum = ChainPerformance{
        transactionpersecond: Chain::Ethereum,
        performance: 3500.0,
    };

    let binance = ChainPerformance{
        transactionpersecond: Chain::Binance,
        performance: 15000.0,
    };

    let solana = ChainPerformance{
        transactionpersecond: Chain::Solana,
        performance: 70000.0,
    };
    let solana = ChainPerformance{
        transactionpersecond: Chain::Solana,
        performance: 70000.0,
    };
    get_chain_details(ethereum);
    println!("\n");
    get_chain_details(solana);
    println!("\n");
    get_chain_details(binance);
    

}