fn gcd(a: int, b: int) -> int
  while(0 != b){
    set b = a % b
  end
  b
end

fn main()
    print(gcd(10, 2))
end
