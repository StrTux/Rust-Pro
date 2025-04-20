💡 Breakdown aur Explanation (Hinglish mein)
🔹 fn main() { ... }
fn ka matlab hota hai function.

main() Rust ka starting point hota hai — yaani jab program run karega to sabse pehle main() function hi chalega.

{ ... } ke andar likha gaya code main function ka body hota hai, jisme hum batate hain ki program kya kare.

🔹 let ans: u32 = sum(1, 2);
let ka matlab hota hai variable banaya.

ans variable ka naam hai.

: u32 ka matlab hai ki ans variable unsigned 32-bit integer type ka hai (yaani sirf positive numbers aur zero store karega).

sum(1, 2) ek function call hai jisme hum 1 aur 2 bhej rahe hain as input (arguments).

sum function dono numbers ka sum (jod) karke return karega, aur wo value ans mein store hogi.

🧠 Example:

rust
Copy
Edit
let ans: u32 = sum(1, 2); // yeh 3 return karega
🔹 println!("{}", ans);
Yeh ek print statement hai.

println! Rust ka special function hai jo screen pe output dikhata hai.

"{}" ek placeholder hai jahan ans ki value aayegi.

Agar ans = 3, to output hoga:

Copy
Edit
3
🔹 fn sum(a: u32, b: u32) -> u32 { return a + b; }
Yeh ek custom function hai jiska naam hai sum.

fn likhne ka matlab hai "function define kar rahe ho".

a aur b function ke parameters (inputs) hain — dono u32 type ke hain.

-> u32 ka matlab hai ki yeh function ek u32 value return karega.

return a + b; ka matlab hai — yeh function a aur b ko jod ke result wapas karega.

🧠 Socha jaaye to:

rust
Copy
Edit
sum(1, 2) => 1 + 2 => 3
🎯 Summary (Chhoti Kahani Jaise):
Aapne ek function banaya sum jo 2 number ko jodta hai.

Fir aapne main() function mein usse call kiya with values 1 aur 2.

Uska result ans variable mein store kiya.

Aur finally println! se result ko screen pe print kiya.



fn main() {
    let ans: u32 = sum(1, 2);
    println!("{}", ans);
}

fn sum(a: u32, b: u32) -> u32 {
    return a + b;
}



iska ans  jo hay  3 ayega 