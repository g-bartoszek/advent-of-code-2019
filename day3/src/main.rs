fn main() {
    let input =
        "R1004,U520,R137,D262,L403,U857,R50,U679,R788,D98,L717,D1,R367,U608,L125,U703,L562,D701,L718,U357,R742,D860,R557,D117,R950,U546,L506,U836,R951,D460,L38,U893,L1,D217,R262,D950,R239,U384,R971,D289,R323,U878,L525,U687,L831,U523,R94,D33,L879,D318,R633,D775,R879,D351,L120,D8,R31,U49,R328,D598,L380,D160,R261,D716,R459,U533,L444,U412,L326,U93,L193,D621,R236,U769,L319,D885,L559,U509,L62,U321,L667,D505,R556,U159,L5,U126,L262,D946,L168,U491,L56,D831,R926,U926,R562,D270,R785,U436,R852,D629,R872,U716,R549,U435,R462,U191,R318,U91,L637,D682,R647,D53,L789,D725,R312,D366,L287,U29,R85,D657,R88,U300,R795,U378,R800,D391,L594,U791,R205,U352,L510,D975,R47,D311,R319,U579,R214,D112,R996,U874,R328,D578,R37,U689,L543,U16,L580,D230,L714,D58,L580,D658,R218,U535,R149,U996,L173,D316,L90,D372,L364,U700,L60,D70,L250,U276,R580,U505,L682,U943,R336,U847,R810,U963,R874,D740,R732,D328,R926,D447,R638,D102,R696,U211,L594,D354,R384,U81,L884,U916,L168,U759,R631,D702,L598,D382,L647,U642,R537,U53,R897,U954,R263,U445,L41,D91,L51,D338,R219,U269,L689,D172,R627,D287,L440,D504,L253,D252,R815,D108,L282,U835,L243,U638,R910,D306,R755,D202,R69,D862,L537,D947,L180,D835,L111,U832,R939,D449,R180,U105,R892,D837,L153,U215,L695,U957,R923,U496,R608,U739,L711,U700,L838,D117,R479,U852,R795,D955,L386,D70,R728,D40,R580,U777,L877,U284,R414,D300,R105,D372,L317,D91,R653,U920,R956,D496,L543,D363,R374,D283,L696,U466,R467,D878,R660,U590,L962,U619,R991,U848,L648,D191,R459,U125,L998,U19,L214,U947,R188,U103,R916\n\
    L1008,U717,R288,D770,R270,U514,R109,D538,L719,U179,R466,D792,R421,U723,L22,U705,L284,U14,L478,U367,R727,U880,R620,D46,R377,U897,L731,U840,L910,D385,L257,U311,L596,D991,L668,D730,L707,D816,R47,U948,R84,D700,R299,U707,R261,D928,R358,D504,R309,U369,R931,U20,L940,U326,L362,D52,R98,D475,L907,D918,R931,D468,R279,D586,R592,U973,R753,D365,R694,U278,R934,U712,R441,U996,L989,D693,L211,D561,R105,D425,R53,U168,L451,U865,L585,D412,L857,U988,R724,U774,R295,U588,R329,D810,L698,D118,R277,U193,R309,U933,R186,D535,R409,U322,L849,U606,R590,U892,L542,D237,R475,D920,R679,U602,L477,D634,L988,D540,L323,U791,L375,U625,L621,U567,L943,U512,L239,D90,L66,U151,R83,U435,R612,D865,L177,U368,R326,U574,L241,U197,R499,U419,R297,U207,L311,D243,L559,D281,R513,U748,L884,U207,R71,D441,R133,D993,L4,D977,L669,U523,L564,U186,R477,U737,L685,U338,L456,U939,R774,U674,L97,D827,R237,D451,R618,D143,R750,U196,L559,D178,L693,D916,R334,U231,L651,U249,R620,U283,L387,U352,L915,U959,L693,U909,R320,U119,L617,U177,L993,D265,R667,U204,R59,D601,L579,U483,R155,D484,L44,D751,R915,U510,L552,U308,R505,U394,R585,U872,L617,U202,R928,U941,R235,U768,R666,D547,L244,D270,R353,D612,R384,U430,L685,D536,R103,U147,R794,D621,L52,U96,L557,D455,L635,D58,R265,U545,R938,D266,L173,U746,L672,D237,R286,U131,R487,U837,R394,D702,R49,U579,L699,U819,L448,D223,L982,D906,L397,U807,L737,D223,L791,D965,R436,U29,R908,D273,R194,U91,R232,U591,L336,D70,R467,U505,L341,U989,R278,U387,L442,U950,R487,D384,L534,D514,L433,U627,R381,U54,L847,U231,L590";
    println!("{}", find_intersection(input));
}


#[derive(Copy, Clone, Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
struct Edge {
    from: Point,
    to: Point,
    distance_from_central_port: usize,
}

impl Edge {
    fn is_vertical(&self) -> bool {
        self.from.x == self.to.x
    }

    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }
}

type Wire = Vec<Edge>;
type Intersection = (Point, usize);

fn is_between(value: i32, begin: i32, end: i32) -> bool {
    (value > begin && value < end) || (value > end && value < begin)
}

fn intersections(lhs: &Wire, rhs: &Wire) -> Vec<Intersection> {
    let mut result = Vec::<Intersection>::new();

    for l in lhs {
        for r in rhs {
            if l.is_vertical() && r.is_horizontal() {
                if is_between(l.from.x, r.from.x, r.to.x) && is_between(r.from.y, l.from.y, l.to.y) {
                    println!("Found intersection: {:?} {:?}", l, r);
                    result.push((Point::new(l.from.x, r.from.y),
                                 l.distance_from_central_port + (r.from.y - l.from.y).abs() as usize + r.distance_from_central_port + (l.from.x - r.from.x).abs() as usize));
                }
            } else if l.is_horizontal() && r.is_vertical() {
                if is_between(l.from.y, r.from.y, r.to.y) && is_between(r.from.x, l.from.x, l.to.x) {
                    println!("Found intersection: {:?} {:?}", l, r);
                    result.push((Point::new(r.from.x, l.from.y),
                                 l.distance_from_central_port + (r.from.x - l.from.x).abs() as usize + r.distance_from_central_port + (l.from.y - r.from.y).abs() as usize));
                }
            }
        }
    }

    result
}

fn parse_input(paths: &str) -> Vec<Wire> {
    paths.lines().map(|path| {
        path.split(",").scan(
            (Point::new(0, 0), 0usize),
            |(ref mut current_position, ref mut distance_from_central_port), wire| {
                let (to, distance) = match (&wire[0..1], str::parse::<i32>(&wire[1..])) {
                    ("R", Ok(distance)) => (Point { x: current_position.x + distance, y: current_position.y }, distance),
                    ("L", Ok(distance)) => (Point { x: current_position.x - distance, y: current_position.y }, distance),
                    ("U", Ok(distance)) => (Point { x: current_position.x, y: current_position.y + distance }, distance),
                    ("D", Ok(distance)) => (Point { x: current_position.x, y: current_position.y - distance }, distance),
                    (_, _) => { panic!("Invalid input!") }
                };
                let edge = Edge { from: *current_position, to, distance_from_central_port: *distance_from_central_port };
                *current_position = to;
                *distance_from_central_port += distance as usize;
                Some(edge)
            }).collect::<Vec<_>>()
    }).collect()
}

fn find_intersection(paths: &str) -> i32 {
    let wires = parse_input(paths);

    let intersections = intersections(&wires[0], &wires[1]);
    println!("Wires: {:?}", wires);
    println!("Intersections: {:?}", intersections);

    intersections.iter().map(|p| p.1 as i32).min().unwrap()
}


#[cfg(test)]
mod test {
    use super::{find_intersection, intersections, Point};
    use crate::{Edge, Intersection};

    //#[test]
    //fn acceptance_first_part() {
    //    assert_eq!(6, find_intersection(
    //        "R8,U5,L5,D3\n\
    //        U7,R6,D4,L4"));
    //    assert_eq!(159, find_intersection(
    //        "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
    //        U62,R66,U55,R34,D71,R55,D58,R83"));
    //    assert_eq!(135, find_intersection("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\n\
    //                                         U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"));
    //}

    #[test]
    fn acceptance_second_part() {
        assert_eq!(610, find_intersection(
            "R75,D30,R83,U83,L12,D49,R71,U7,L72\n\
                    U62,R66,U55,R34,D71,R55,D58,R83"));
    }

    #[test]
    fn finds_intersections() {
        assert_eq!(vec![(Point::new(5, 5), 10)], intersections(
            &vec![Edge { from: Point::new(0, 5), to: Point::new(10, 5), distance_from_central_port: 0 }],
            &vec![Edge { from: Point::new(5, 0), to: Point::new(5, 10), distance_from_central_port: 0 }]));

        assert_eq!(vec![(Point::new(5, 3), 8)], intersections(
            &vec![Edge { from: Point::new(0, 3), to: Point::new(10, 3), distance_from_central_port: 0 }],
            &vec![Edge { from: Point::new(5, 0), to: Point::new(5, 10), distance_from_central_port: 0 }]));

        assert_eq!(vec![(Point::new(3, 3), 6), (Point::new(7, 3), 10)], intersections(
            &vec![Edge { from: Point::new(0, 3), to: Point::new(10, 3), distance_from_central_port: 0 }],
            &vec![
                Edge { from: Point::new(3, 0), to: Point::new(3, 10), distance_from_central_port: 0 },
                Edge { from: Point::new(7, 0), to: Point::new(7, 10), distance_from_central_port: 0 }
            ]));

        assert_eq!(Vec::<Intersection>::new(), intersections(
            &vec![Edge { from: Point::new(8, 5), to: Point::new(3, 5), distance_from_central_port: 0 }],
            &vec![Edge { from: Point::new(0, 0), to: Point::new(0, 7), distance_from_central_port: 0 }]));
    }
}
