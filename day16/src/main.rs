

fn generate_pattern(n: usize) -> Vec<i32> {
   let mut result = Vec::<i32>::new();

   for _ in 0..=n  {
      result.push(0);
   }

   for _ in 0..=n  {
      result.push(1);
   }

   for _ in 0..=n  {
      result.push(0);
   }

   for _ in 0..=n  {
      result.push(-1);
   }


   result
}

fn get_pattern_value(n: i32, i: i32) -> i32 {
   let v = (i+1) % (4 * (n+1));

   if v >= n && v < (2 * n) {
      return 1;
   }

   if v >= (3 * n) {
      return -1;
   }

   0
}

fn main() {

   let t1 = "03036732577212944063491565474664";
   let tf = "59717238168580010599012527510943149347930742822899638247083005855483867484356055489419913512721095561655265107745972739464268846374728393507509840854109803718802780543298141398644955506149914796775885246602123746866223528356493012136152974218720542297275145465188153752865061822191530129420866198952553101979463026278788735726652297857883278524565751999458902550203666358043355816162788135488915722989560163456057551268306318085020948544474108340969874943659788076333934419729831896081431886621996610143785624166789772013707177940150230042563041915624525900826097730790562543352690091653041839771125119162154625459654861922989186784414455453132011498";

   let mut input = tf.chars().map(|c| c.to_digit(10).unwrap() as i32).collect::<Vec<_>>();

   let mut extended =  Vec::<i32>::new();
   for _ in 0..10000 {
      extended.append(&mut input.clone());
   }

   input = extended;



   for i in 0..100 {

      //println!("Before {:?}", input);

      let mut after = Vec::<i32>::new();
      for i in 0..input.len() {
         let mut pattern = generate_pattern(i);
         let mut pattern_cycle = pattern.iter().cycle();
         pattern_cycle.next();

         let now = std::time::SystemTime::now();

         let value = input.iter().zip(pattern_cycle).map(|(&l,&r)| l as i32 * r as i32).sum::<i32>().abs() % 10;

         println!("Elapsed {}", now.elapsed().unwrap().as_millis());

         after.push(value);

         println!("{}/{}",i, input.len());
      }

      println!("After {:?}", after);
       input = after;
   }



}
