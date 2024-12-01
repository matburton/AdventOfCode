
const INPUT: &str = include_str!("../input/day_15.txt");

const EXAMPLE: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

fn get_hash(text: &str) -> usize {

    text.chars().fold(0, |s, c| (s + c as usize) * 17 % 256)
}

#[test]
fn test_get_hash() { assert_eq!(get_hash("HASH"), 52); }

mod part_1 {

    use super::*;

    fn get_result(input: &str) -> usize { input.split(',').map(get_hash).sum() }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 1320); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 511215); }
}

mod part_2 {

    use super::*;

    struct Lens<'a> { label: &'a str, focal: usize }

    fn get_result(input: &str) -> usize {

        let mut lens_boxes: [Vec<Lens>; 256] =
            std::array::from_fn(|_| Vec::new());
       
        for texts in input.split(',').map(|t| t.split(['=', '-'])
                                               .collect::<Vec<_>>()) {

            let lens_box = &mut lens_boxes[get_hash(texts[0])];

            let lens_index = lens_box.iter().position(|l| l.label == texts[0]);

            if let Some(focal) = texts.get(1).and_then(|t| t.parse().ok()) {

                if let Some(index) = lens_index {

                    lens_box[index].focal = focal;
                }
                else { lens_box.push(Lens { label: texts[0], focal }); }
            }
            else if let Some(index) = lens_index { lens_box.remove(index); }
        }

        let score_box = |box_index, lens_box: &[Lens]|
            lens_box.iter()
                    .enumerate()
                    .map(|(i, l)| (box_index + 1) * (i + 1) * l.focal)
                    .sum::<usize>();

        lens_boxes.iter().enumerate().map(|(i, b)| score_box(i, b)).sum()
    }

    #[test]
    fn example() { assert_eq!(get_result(EXAMPLE), 145); }
    
    #[test]
    fn real() { assert_eq!(get_result(INPUT), 236057); }
}