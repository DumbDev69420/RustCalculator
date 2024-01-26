use std::alloc::System;
use std::marker::PhantomData;
use std::{char, string};
use std::io::{stdin, stdout, Write};

enum RechenArt{
    Plus,
    Minus,
    Geteilt,
    Mal,
    None
}

struct RechenDaten{
    numb1: String,
    numb2: String,
    rechen_zeichen: char,
    rechen_Art: RechenArt,
    ergebniss: f64,
    error: bool,
}



fn is_Num(input:char, )->bool{
    return (input >= '0' && input <= '9');
}

fn is_Special(input:char)->bool{
    return (input == '-' || input == '*' || input == '/' || input == '+');
}

fn get_Mode(input:char)->RechenArt{

    let mut rechenArtOutput: RechenArt;

    match input {
        '-' =>{
            rechenArtOutput = RechenArt::Minus;
        },
        '+' =>{
            rechenArtOutput = RechenArt::Plus;
        },
        '*' =>{
            rechenArtOutput = RechenArt::Mal;
        },
        '/' =>{
            rechenArtOutput = RechenArt::Geteilt;
        },

        _ =>{
            rechenArtOutput = RechenArt::None;
        }
    }

    return rechenArtOutput;
}

fn calculate_Data(daten:&mut RechenDaten)->bool{
    let convertedNumb1:f64 = daten.numb1.trim().parse::<f64>().unwrap();
    let convertedNumb2:f64 = daten.numb2.trim().parse::<f64>().unwrap();

    match daten.rechen_Art {

        RechenArt::Geteilt =>{
            daten.ergebniss = convertedNumb1 / convertedNumb2;
        }
        RechenArt::Mal =>{
            daten.ergebniss = convertedNumb1 * convertedNumb2;
        }
        RechenArt::Minus =>{
            daten.ergebniss = convertedNumb1 - convertedNumb2;
        }
        RechenArt::Plus =>{
            daten.ergebniss = convertedNumb1 + convertedNumb2;
        }
        RechenArt::None =>{
            println!("Couldn't Calculate Data since passed Data was unusable!\n");
            return false;
        }
    }

    return true;
}

fn CopyNumb(input:String)->RechenDaten {

    let mut rechenDaten = RechenDaten {numb1: String::new(), numb2: String::new(), 
    rechen_zeichen: 'n',rechen_Art: RechenArt::None, ergebniss: 0.0, error:false };

    let mut wroteCheckNumb:bool = true;
    let mut wroteNum: bool = false;
    let mut wasNonNumb:bool = false;
    let mut state:i32 = 0;



    let Size: usize = input.len();

    for char in input.chars().into_iter() {

        if(char == '.' || is_Num(char)){
            if(char == '.'){wroteCheckNumb = false;}
            else {
                wroteCheckNumb = true;
            }

            wroteNum = true;
            if(wasNonNumb){
                if(!wroteCheckNumb){rechenDaten.numb1.push('0'); wroteCheckNumb = true}
                wasNonNumb = false;
                state += 1;
            }

            if(state == 0){
                rechenDaten.numb1.push(char);
            }
            else {
                rechenDaten.numb2.push(char);
            }
        }
        else {
            wasNonNumb = true;

            if(is_Special(char)){
                rechenDaten.rechen_zeichen = char;
                rechenDaten.rechen_Art = get_Mode(char);
            }
        }
    }

    if(!wroteNum || state == 0){
        rechenDaten.error = true;
    }

    if(!wroteCheckNumb){rechenDaten.numb2.push('0'); wroteCheckNumb = true}

    //debug stuff
    //println!("1: {}, 2: {}, sign: {}", rechenDaten.numb1, rechenDaten.numb2, rechenDaten.rechen_zeichen);

    return rechenDaten;
}


fn main() {
    while true {
        
    
    println!("Hello, Enter your Operation!: ");
    //let new_T:Taschenrechner = {RechenArt::Plus};

    let mut inpt: String = String::new();

    let _ = stdout().flush();

    stdin().read_line(&mut inpt);

    if(inpt.contains("exit")){break;}
    
    let mut output: RechenDaten = CopyNumb(inpt);

    if(output.error){
        println!("Couldn't Calculate Data since passed Data was unusable!\n");
    }
    else {
        if(calculate_Data(&mut output)){
            println!("\n Output: {}", output.ergebniss);
        } 
    }

   }

}
