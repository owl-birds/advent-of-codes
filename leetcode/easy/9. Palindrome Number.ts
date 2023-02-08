function isPalindrome(x: number): boolean {
    if (x < 0) return false;
    // reversing the num without converting it into an number
    let num = x;
    let new_num = 0;
    while (num>0){
        new_num = new_num * 10 + num%10;
        num = Math.floor(num/10);
    }
    return new_num === x;
};
