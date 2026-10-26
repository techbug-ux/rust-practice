fn main() {
    let ticker = "AAPL";
    let price = 187.50;
    let shares = 10;

    let total = price * shares as f64;
    println!("{} shares of {} cost ${}", shares, ticker, total);
}