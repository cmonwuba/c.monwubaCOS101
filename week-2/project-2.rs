fn main() {
   let qToshiba = 2.0;
   let aToshiba = 450_000.0;
   let qMac = 1.0;
   let aMac = 1_500_000.0;
   let qHp = 3.0;
   let aHp = 750_000.0;
   let qDell = 3.0;
   let aDell = 2_850_000.0;
   let qAcer = 1.0;
   let aAcer = 250_000.0;

   let sum = (qToshiba*aToshiba)+(qMac*aMac)+(qHp*aHp)+(qDell*aDell)+(qAcer*aAcer);
   let average = sum/(qToshiba+qMac+qHp+qDell+qAcer);
   println!("The sum is {} and the Average is {}", sum,average);
}