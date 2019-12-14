use std::time::Duration;

#[derive(Debug,PartialEq, Eq, Hash)]
struct Chemical
{
    name: String,
    quantity: usize
}

#[derive(Debug,PartialEq, Eq, Hash)]
struct Rule
{
    result: Chemical,
    ingredients: Vec<Chemical>
}



fn parse_chemical(input: &str) -> Chemical {
    let quantity_and_name = input.split(" ").collect::<Vec<_>>();

    Chemical{
        name: quantity_and_name[1].to_string(),
        quantity: str::parse::<usize>(quantity_and_name[0]).unwrap()
    }
}

type Rules = std::collections::HashMap<String, Rule>;


fn main() {

    let input = "\
        9 ORE => 2 A\n\
        8 ORE => 3 B\n\
        7 ORE => 5 C\n\
        3 A, 4 B => 1 AB\n\
        5 B, 7 C => 1 BC\n\
        4 C, 1 A => 1 CA\n\
        2 AB, 3 BC, 4 CA => 1 FUEL\n\
    ";
    let input1 = "\
    171 ORE => 8 CNZTR\n\
    7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n\
    114 ORE => 4 BHXH\n\
    14 VRPVC => 6 BMBT\n\
    6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n\
    6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n\
    15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n\
    13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n\
    5 BMBT => 4 WPTQ\n\
    189 ORE => 9 KTJDG\n\
    1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n\
    12 VRPVC, 27 CNZTR => 2 XDBXC\n\
    15 KTJDG, 12 BHXH => 5 XCVML\n\
    3 BHXH, 2 VRPVC => 7 MZWV\n\
    121 ORE => 7 VRPVC\n\
    7 XCVML => 6 RJRHP\n\
    5 BHXH, 4 VRPVC => 5 LTCX \n\
    ";

    let input = "\
    157 ORE => 5 NZVS\n\
    165 ORE => 6 DCFZ\n\
    44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n\
    12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n\
    179 ORE => 7 PSHF\n\
    177 ORE => 5 HKGWZ\n\
    7 DCFZ, 7 PSHF => 2 XJWVT\n\
    165 ORE => 2 GPVTF\n\
    3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT\n\
    ";

    let input = "\
    1 ZQVND => 2 MBZM\n\
    2 KZCVX, 1 SZBQ => 7 HQFB\n\
    1 PFSQF => 9 RSVN\n\
    2 PJXQB => 4 FSNZ\n\
    20 JVDKQ, 2 LSQFK, 8 SDNCK, 1 MQJNV, 13 LBTV, 3 KPBRX => 5 QBPC\n\
    131 ORE => 8 WDQSL\n\
    19 BRGJH, 2 KNVN, 3 CRKW => 9 MQJNV\n\
    16 DNPM, 1 VTVBF, 11 JSGM => 1 BWVJ\n\
    3 KNVN, 1 JQRML => 7 HGQJ\n\
    1 MRQJ, 2 HQFB, 1 MQJNV => 5 VQLP\n\
    1 PLGH => 5 DMGF\n\
    12 DMGF, 3 DNPM, 1 CRKW => 1 CLML\n\
    1 JSGM, 1 RSVN => 5 TMNKH\n\
    1 RFJLG, 3 CFWC => 2 ZJMC\n\
    1 BRGJH => 5 KPBRX\n\
    1 SZBQ, 17 GBVJF => 4 ZHGL\n\
    2 PLGH => 5 CFWC\n\
    4 FCBZS, 2 XQWHB => 8 JSGM\n\
    2 PFSQF => 2 KNVN\n\
    12 CRKW, 9 GBVJF => 1 KRCB\n\
    1 ZHGL => 8 PJMFP\n\
    198 ORE => 2 XQWHB\n\
    2 BWVJ, 7 CFWC, 17 DPMWN => 3 KZCVX\n\
    4 WXBF => 6 JVDKQ\n\
    2 SWMTK, 1 JQRML => 7 QXGZ\n\
    1 JSGM, 1 LFSFJ => 4 LSQFK\n\
    73 KNVN, 65 VQLP, 12 QBPC, 4 XGTL, 10 SWMTK, 51 ZJMC, 4 JMCPR, 1 VNHT => 1 FUEL\n\
    1 BWVJ, 7 MBZM => 5 JXZT\n\
    10 CFWC => 2 DPMWN\n\
    13 LQDLN => 3 LBTV\n\
    1 PFZW, 3 LQDLN => 5 PJXQB\n\
    2 RSVN, 2 PFSQF => 5 CRKW\n\
    1 HGQJ, 3 SMNGJ, 36 JXZT, 10 FHKG, 3 KPBRX, 2 CLML => 3 JMCPR\n\
    126 ORE => 4 FCBZS\n\
    1 DNPM, 13 MBZM => 5 PLGH\n\
    2 XQWHB, 10 FCBZS => 9 LFSFJ\n\
    1 DPMWN => 9 PFZW\n\
    1 ZJMC, 3 TMNKH => 2 SWMTK\n\
    7 TZCK, 1 XQWHB => 5 ZQVND\n\
    4 CFWC, 1 ZLWN, 5 RSVN => 2 WXBF\n\
    1 BRGJH, 2 CLML => 6 LQDLN\n\
    26 BWVJ => 2 GBVJF\n\
    16 PJXQB, 20 SDNCK, 3 HQFB, 7 QXGZ, 2 KNVN, 9 KZCVX => 8 XGTL\n\
    8 PJMFP, 3 BRGJH, 19 MRQJ => 5 SMNGJ\n\
    7 DNPM => 2 SZBQ\n\
    2 JQRML, 14 SDNCK => 8 FHKG\n\
    1 FSNZ, 6 RFJLG, 2 CRKW => 8 SDNCK\n\
    2 CLML, 4 SWMTK, 16 KNVN => 4 JQRML\n\
    8 TZCK, 18 WDQSL => 2 PFSQF\n\
    1 LSQFK => 8 VTVBF\n\
    18 BRGJH, 8 ZHGL, 2 KRCB => 7 VNHT\n\
    3 TZCK => 4 DNPM\n\
    14 PFZW, 1 PFSQF => 7 BRGJH\n\
    21 PLGH, 6 VTVBF, 2 RSVN => 1 ZLWN\n\
    149 ORE => 2 TZCK\n\
    3 JSGM => 1 RFJLG\n\
    4 PFSQF, 4 DMGF => 3 MRQJ\n\
    ";


    let mut rules = Rules::new();

    for l in input.lines() {
        let split = l.split("=>").collect::<Vec<_>>();

        let ingredients = split[0].split(", ").map(|s| {
            parse_chemical(s)
        }).collect::<Vec<_>>();

        let result = parse_chemical(split[1].trim());

        rules.insert(result.name.clone(), Rule{result, ingredients});

        println!("Rules: {:?}", rules);
    }

    let mut required = std::collections::HashMap::<String, i32>::new();
    let mut ore_total = 0usize;

    required.insert("FUEL".to_string(), 1);


    loop {
        println!("Reqired chemicals: {:?}", required);
        let mut new_required = required.clone();
        for r in &required {
            if r.0 == "ORE" {
                ore_total += *r.1 as usize;
                continue;
            }

            let rule = &rules[r.0];

            let needed = *r.1;
            let multiplier = (needed as f32 / rule.result.quantity as f32).ceil() as usize;
            let can_produce =  multiplier * rule.result.quantity;

            *new_required.get_mut(r.0).unwrap() -= can_produce as i32;

            for ingredient in &rule.ingredients {
                if !new_required.contains_key(&ingredient.name) {
                    new_required.insert(ingredient.name.clone(), 0);
                }

                *new_required.get_mut(&ingredient.name).unwrap() += (ingredient.quantity * multiplier) as i32;
            }

        }
        required = new_required;
    }

}
