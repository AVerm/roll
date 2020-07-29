use roll::tokenize;
use roll::parse;
use roll::evaluate;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    let input = args.join(" ");

    let tokenized = tokenize(input.trim().to_string());
    let parsed = parse(tokenized);
    let evaluated = match parsed {
        Ok(parse_tree) => evaluate(parse_tree),
        Err(error) => {
            eprintln!("{}", error);
            return;
        },
    };
    match evaluated {
        Ok(answer) => println!("{}", answer),
        Err(error) => eprintln!("{}", error),
    }
}
