fn main() {
    // boolean true false
    let varboll: bool = true;
    println!("boolean: {}", varboll);
    //chareter
    let varchar: char = 'a';
    println!("char: {}", varchar);
    //integer
    let varint8: i8 = 127; 
     let varint16: i16 = 32767; 
      let varint32: i32 = 2147483647; 
       let varint64: i64 = 127;  
       let varint128: i128 = 127;  
    println!("integer i8 : {} integer i16 : {} integer i32: {} integer i64: {} integer i128: {}", varint8,varint16,varint32,varint64,varint128);

    let varuint8: u8 = 255; 
     let varuint16: u16 = 65535; 
      let varuint32: u32 = 4294967295; 
       let varuint64: u64 = 127;  
       let varuint128: u128 = 127;  

        println!("uinteger i8 : {} uinteger i16 : {} uinteger i32: {} uinteger i64: {} uinteger i128: {}", varuint8,varuint16,varuint32,varuint64,varuint128);


    //floot
    let varfloot32:f32=32.56;    
    let varfloot64:f64=32.56;
    println!("floot 32 : {}  floot 64 :{}",varfloot32,varfloot64);

    //size
    let var_num:isize=68986;
    let  var_unum:usize =89989 ;
    println!("isize :{var_num} usize :{var_unum}");

    // string
    let var_str: &str="shadweb";
    println!("string: {var_str}");

}
