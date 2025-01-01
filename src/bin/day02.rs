use nom::Err;
use nom::error::Error;
use adventofcode2015::build_main_res;

type E<'a> = Err<Error<&'a str>>;

mod parse {
    use nom::character::complete::{char, digit1, newline};
    use nom::combinator::{map, map_res};
    use nom::IResult;
    use nom::multi::separated_list1;

    fn number(input: &str) -> IResult<&str, usize> {
        map_res(digit1, |s: &str| s.parse::<usize>())(input)
    }

    pub fn boxes(input: &str) -> IResult<&str, Vec<[usize; 3]>> {
        separated_list1(
            newline,
            map(
                separated_list1(
                    char('x'),
                    number
                ),
                |vec| [vec[0], vec[1], vec[2]]
            )
        )(input)
    }
}

fn part1(input: &str) -> Result<usize, E> {
    let boxes = parse::boxes(input)?.1;
    let result = boxes.into_iter()
        .map(|mut arr| {
            arr.sort();
            let [l, w, h] = arr;
            2*l*w + 2*w*h + 2*h*l + l * w
        })
        .sum();

    Ok(result)
}

fn part2(input: &str) -> Result<usize, E> {
    let boxes = parse::boxes(input)?.1;
    let result = boxes.into_iter()
        .map(|mut arr| {
            arr.sort();
            let [l, w, h] = arr;
            2 * l + 2 * w + l * w * h
        })
        .sum();

    Ok(result)

}

build_main_res!("day02.txt", "Part 1" => part1, "Part 2" => part2);