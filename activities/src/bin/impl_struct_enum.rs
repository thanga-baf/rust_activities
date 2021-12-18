// define a txn_details
// define a chain
// define a prefered chain to add the chain details

enum Chain {
    Ethereum,
    Solana,
    Polygon,
}

struct Txndetails {
    blocktime: i32,
    count: i32,
}

struct MyChain{
    name: Chain,
    details: Txndetails,
}

impl Chain {
    fn print(&self){
        match self{
            Chain::Ethereum => println!("Its ethereum"),
            Chain::Solana => println!("its solana"),
            Chain::Polygon => println!("its polygon"),
        }
    }
}


impl Txndetails {
    fn print(&self){
        println!("blocktime {:?}",self.blocktime);
        println!("count {:?}",self.count);
    }
}

impl MyChain {
    fn create(name:Chain,details:Txndetails) -> Self {
        Self {
            name,
            details,
        }
    }

    fn print(&self){
        self.name.print();
        self.details.print();
    }
}


fn main() {
    let solana_details = Txndetails{
        blocktime: 1,
        count: 45000,
    };
    let thanga_chain = MyChain::create(Chain::Solana,solana_details);
    thanga_chain.print();

    let ethereum_details = Txndetails{
        blocktime:10,
        count:3000,
    };
    let ethereum_chain = MyChain::create(Chain::Ethereum,ethereum_details);
    ethereum_chain.print();
}