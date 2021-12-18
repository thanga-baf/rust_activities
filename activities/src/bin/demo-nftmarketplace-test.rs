enum MarketPlace {
    Opensea,
    Solsea,
    Solanart,
}

struct NFTdetails {
    details: MarketPlace,
    count: i32,
}

fn get_nft(_details: NFTdetails){
    match _details.details{
        MarketPlace::Opensea => println!("opensea"),
        MarketPlace::Solsea => println!("solsea"),
        MarketPlace::Solanart => println!("solanart"),
    }
    println!("{:?}",_details.count);
}

fn main() {
    let opensea_nft = NFTdetails {
        details: MarketPlace::Opensea,
        count: 100
    };

    get_nft(opensea_nft);

    let solsea_nft = NFTdetails {
        details: MarketPlace::Solsea,
        count: 3600
    };

    get_nft(solsea_nft);

}