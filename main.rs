 
 

use num_traits::sign::Signed; 
use fraction::BigFraction;
use num_bigint::BigInt; 

fn function(x:f64,log1:f64,log2:f64) -> f64 

{ 
    let y: f64 = (x*(log1))+x.ln()-((x-1.0)*log2);
    return y; 
} 

fn bisect(base:u32) -> f64 
{ 

    let mut x:f64 = base as f64; 
    let log1:f64=((base-1)as f64).ln();
    let log2:f64=(base as f64).ln();
    while function(x,log1,log2).is_positive() 
    { 
        x = x * 2.0; 
    } 

    while function(x,log1,log2).is_negative() 
    { 
        x-=1.0; 
    }
    x+=1.0;
    for i in 1..20
    {
        if function(x-(1.0/(2_i32.pow(i)as f64)),log1,log2).is_negative()
        {
            x=x-(1.0/(2_i32.pow(i) as f64));
        }
    }
    return x; 
} 

fn main() 

{ 

    const length:usize = 400; 

    let mut array: [f64; length] = [0.0; length]; 

    array[0] = 0.0; 
    array[1] = 0.0; 
    array[2] = 0.0; 
    array[3] = 0.0; 

    for base in (4..length) 

    { 

        array[base]=(bisect(base as u32)); 
        //println!("{} {},",bisect(base as u32),i); 
    } 
    for i in (1..length){println!("{} {},",array[i]-array[i-1],i);}
    

} 
