pub fn main(){
    let x = u32::checked_add(u32::MAX,1);
    println!("{:?}",x);

    let y = u32::wrapping_add(u32::MAX,1);
    println!("{:?}",y);

    let z = u32::saturating_add(u32::MAX,1);
    println!("{:?}",z);

    let w = u32::overflowing_add(5,1);
    println!("{:?}",w);
}