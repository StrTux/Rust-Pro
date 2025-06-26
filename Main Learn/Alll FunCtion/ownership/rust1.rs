fn main() {
    let s1 = String::from("world");
    let s2 = s1;

    println!("{}", s2);
}

// yaha  pe hua ye ki  mene s1 me value daali  thi  or fir baad me ussi si ki  value ko  
// s2  me daal  diya tha too  rust ownership  ye isko  move  kar deta  hay jis se kaam asaan ho jata hay thoda
