#[derive(Debug)]
struct Block {
    begin: usize,
    end: usize,
    step: usize,
    multiplier: i32,
    value: i32,
}

impl Block {
    fn advance(&self, data: &[i32]) -> Option<Self> {
        let new_begin = self.begin + self.step;
        if new_begin >= data.len() {
            return None;
        }

        let mut new_end = self.end + self.step + 1;

        if new_end > data.len() {
            new_end = data.len();
        }

        let value = if new_begin < self.end
        {
            let to_remove: i32 = data[self.begin..new_begin].iter().sum::<i32>() * self.multiplier;
            let to_add: i32 = data[self.end..new_end].iter().sum::<i32>() * self.multiplier;

            self.value - to_remove + to_add
        } else {
            data[new_begin..new_end].iter().sum::<i32>() * self.multiplier
        };

        Some(Block { begin: new_begin, end: new_end, step: self.step, multiplier: self.multiplier, value })
    }
}

fn main() {
    let tf = "59717238168580010599012527510943149347930742822899638247083005855483867484356055489419913512721095561655265107745972739464268846374728393507509840854109803718802780543298141398644955506149914796775885246602123746866223528356493012136152974218720542297275145465188153752865061822191530129420866198952553101979463026278788735726652297857883278524565751999458902550203666358043355816162788135488915722989560163456057551268306318085020948544474108340969874943659788076333934419729831896081431886621996610143785624166789772013707177940150230042563041915624525900826097730790562543352690091653041839771125119162154625459654861922989186784414455453132011498";

    let mut input = t2.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();

    let mut extended = Vec::<i32>::new();
    for _ in 0..10000 {
        extended.append(&mut input.clone());
    }

    input = extended;


    for oi in 0..100 {
        let mut after = Vec::<i32>::with_capacity(input.len());

        let mut blocks = Vec::<Block>::with_capacity(input.len());

        let mut mul = 1;
        for i in (0..input.len()).step_by(2) {
            blocks.push(Block { begin: i, end: i + 1, step: i + 1, multiplier: mul, value: input[i] * mul });
            mul *= -1;
        }

        for i in 0..input.len() {
            let value = blocks.iter().map(|b| b.value).sum::<i32>().abs() % 10;

            after.push(value);

            let mut new_blocks = Vec::<Block>::with_capacity(blocks.len() / 2);

            for b in blocks {
                if let Some(new_b) = b.advance(&input) {
                    new_blocks.push(new_b);
                } else {
                    break;
                }
            }

            blocks = new_blocks;
        }

        input = after;
    }

    println!("{:?}", &input[0308177..(0308177 + 8)])
}
