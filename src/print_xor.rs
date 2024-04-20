pub fn print (){
    println!("
           An example of how XOR encryption and decryption works

             Input A   0 | 0 | 1 | 1 | 0 | 0 | 1 | - First key
                       ---------------------------
             Input B   0 | 1 | 0 | 1 | 1 | 1 | 0 | - Second Key
                       ---------------------------
             Result    0 | 1 | 1 | 0 | 1 | 1 | 1 |
                       |   |   |   |   |   |   |
                       |   |   |   |   |   |   |
                       |   |   |   |   |   |   |
             Result    0 | 1 | 1 | 0 | 1 | 1 | 1 | - Result
                       ---------------------------
             Input B   0 | 1 | 0 | 1 | 1 | 1 | 0 | - Second Key
                       ---------------------------
             Input A   0 | 0 | 1 | 1 | 0 | 0 | 1 | - First key

                           ");
}