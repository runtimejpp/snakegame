    fn main(){
        let a = 10;
        let b = a;
        let c = 15;
        let d = add(a, b);

        println!("{}", d);

    }

    fn add(x: u32, y: u32) -> u32 {
        let sum = x + y;
      //  return sum;
        sum;
    

    }
