// One byte can be used to encode any integer between 'O' and inclusive:
// A) 16
// B) 256
// C) 128
// D) 255
// E) None of the above
// Solution: Correct option is D)

// One byte can be used to encode any integer between 'O' and inclusive is a 255

// Sometimes you hear a group of four bits called a nibble. The largest number 
// you can represent with 8 bits is 11111111, or 255 in decimal notation.
// Since 00000000 is the smallest, you can represent 256 things with a byte.
// (Remember, a bite is just a pattern. A bit is a binary digit.
// So a byte can hold 2 (binary) ^ 8 numbers ranging from 0 to 2^8-1 = 255.
// It's the same as asking why a 3 digit decimal number can represent values 
// 0 through 999, which is answered in the same manner (10^3 - 1). 
// Originally bytes weren't always 8 bits though.
