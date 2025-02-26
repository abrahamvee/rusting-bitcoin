//FF13 = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12}

struct FieldElement{
    num: u64,    
    prime: u64,
}

impl FieldElement{
    fn new(num: u64, prime: u64) -> Self{
        assert!(num < prime, "Num must be in range from 0 to p-1");
        Self{num, prime}
    }

    fn display(&self){
        println!("Field Element: {}, {}", self.num, self.prime);
    }

    fn equals(&self, other: &Self)-> bool{
        self.num == other.num && self.prime == other.prime
    }

    //Addition: 15 mod 13 = 2
    fn add(&self, other: &Self)-> Self{
        assert!(self.prime == other.prime, "Elements must be on the same field");
        Self::new((self.num + other.num).rem_euclid(self.prime), self.prime)
    }

    //Subtraction: 7 - 8 = -1 
    fn sub(&self, other: &Self)-> Self{
        assert!(self.prime == other.prime, "Elements must be on the same field");
        if self.num < other.num{
            Self::new((self.num + self.prime - other.num).rem_euclid(self.prime), self.prime)
        }else{
            Self::new((self.num - other.num).rem_euclid(self.prime), self.prime)
        }  
    }

    //Multiplication
    fn mult(&self, other: &Self)-> Self{
        assert!(self.prime == other.prime, "Elements must be on the same field");
        Self::new((self.num * other.num).rem_euclid(self.prime), self.prime)
    }

    //Exponentiation
    fn exp(&self, exponent: i32) -> Self{
        //FLT: a^(p-1) = 1 mod p using this we can make the exponent n = exp mod(p-1)
        let n = exponent.rem_euclid((self.prime - 1) as i32) as u32;
        Self::new(self.num.pow(n).rem_euclid(self.prime), self.prime)
    }

    //"Division"
    // a/b -> a*b^(-1)
    // Modular inverse: b*b^(-1) = 1 
    fn div(&self, other: &Self)-> Self{
        assert!(self.prime == other.prime, "Elements must be on the same field");
        let inverse = other.exp(-1);
        Self::new((self.num * inverse.num).rem_euclid(self.prime), self.prime)
    }  
}

fn main() {

    let a = FieldElement{
        num: 7,
        prime: 13
    };

    println!("Field Element: {}, {}", a.num, a.prime);

    let b = FieldElement::new(8, 13);
    b.display();

    let c = a.add(&b);
    c.display();

    let d = a.div(&b);
    d.display();

}
