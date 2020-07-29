use roll::tokenize;
use roll::parse;
use roll::evaluate;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)
        .expect("Failed to read input");

    let tokenized = tokenize(input.trim().to_string());

    println!("{:?}", tokenized);

    let parsed = parse(tokenized);

    println!("{:#?}", parsed);

    let evaluated = evaluate(parsed.unwrap());

    println!("{:?}", evaluated);
}
