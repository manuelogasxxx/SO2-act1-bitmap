//Coded by Manuel Fernández Mercado 
use std::io;
const T_MEMORY: u32 = 1024;
const CHUNK_SIZE: u32 = 4;

struct Process{
    size: u32,
    begin: u32,
} 
//important functions using bit manipulation

fn is_power_of_two(n:u32)->bool{
    return n>0 && (n&(n-1))==0;
}

fn divide_module(a:u32,b:u32)->(u32,u32){
    let n=b.trailing_zeros();
    let quotient= a>>n;
    let residue= a & (b - 1);
    return (quotient,residue);
}


/*check if the given values of memory and chunks are 
pawers of two and if T_MEMORY is less than CHUN_SIZE*/ 
fn memory_check()->bool{
    if !is_power_of_two(T_MEMORY) || !is_power_of_two(CHUNK_SIZE) || T_MEMORY<CHUNK_SIZE {
        return false;
    }
    return true;
}

//shows the entire bitmap 
fn show_memory(v: &Vec<u32>){
    for row in v{
        println!("{:032b}",row);
    }
}

/*take an u32 number of chunks as input and find the number 
of 32-bit rows to represent that cuantity*/ 
fn construct_memory(chunks:u32)->Vec<u32>{
    let mut result: Vec<u32> = Vec::new();
    if chunks<=32{
        result.push(0);
        return result;
    }
    let rows: u32 = chunks>>5;
    for i in 0..rows{
        result.push(0);
    }
    return result;
}


//va a devolver el bit en el que se va a encontrar el inicio del proceso
fn find_space(v: &Vec<u32>, size:u32)->Option<u32>{
    let mut mask: u32 =1;
    let mut size_counter: u32=0;
    let mut bit_counter: u32=0;
    
    for i in 0..v.len(){
        mask=1;
        for j in 0..32{ //checar el ancho de los bits
            if mask & v[i] == 0  {
                size_counter+=1;
            }
            else{
                size_counter=0;
            }
            bit_counter+=1;
            
            if size_counter==size{
                return Some(bit_counter-size+1);//
            }
            mask<<=1;//
        }
    }
    return None;
}


fn fill_memory(v: &mut Vec<u32>, size: u32, begin: u32) {
    let (row, bit_index) = divide_module(begin, 32); // Determine starting row and bit index
    let mut size_counter: u32 = 0; // Tracks the number of bits set
    let mut flag = false; // Indicates when we've reached the starting bit
    let mut end = false; // Ends processing when all bits are set
    
    for i in row as usize..v.len() {
        let mut mask: u32 = if i == row as usize { 1 << bit_index } else { 1 }; // Initialize mask
        let start_bit = if i == row as usize { bit_index } else { 0 }; // Start bit for each row
        
        for j in bit_index as usize..32 {
            if size_counter < size {
                v[i] |= mask; // Set the bit
                size_counter += 1;
            } else {
                end = true;
                break;
            }
            mask <<= 1; // Shift mask to the next bit
        }
        if end {
            break;
        }
    }
}


//segunda función para la memoria

fn fill_memory2(v:&mut Vec<u32>, size:u32, begin:u32){
	let (row, bit_index) = divide_module(begin, 32); // Determine starting row and bit index
	let mut size_counter: u32 = 0; // Tracks the number of bits set
	let mut flag = false; // Indicates when we've reached the starting bit
	let mut end = false; // Ends processing when all bits are set
	    
	for i in row as usize..v.len() {
		let mut mask: u32 = if i == row as usize { 1 << bit_index } else { 1 }; // Initialize mask
	        
	        for j in bit_index as usize..32 {
	            if size_counter < size {
	                v[i] |= mask;
	                size_counter += 1;
	            } else {
	                end = true;
	                break;
	            }
	            mask <<= 1; // Shift mask to the next bit
	        }
	        if end {
	            break;
	        }
	    }
}

fn clear_memory(v: & mut Vec<u32>, size:u32, begin:u32){
    let ( row,bit_index)=divide_module(begin,32);
    let mut mask: u32 =1;
    let mut size_counter: u32=0;
    let mut bit_counter: u32=1;
    let mut flag = false;
    let mut end = false;
    
    for i in row as usize..v.len(){
        mask=1;
        for j in 0..32{ //checar el ancho de los bits
            
            
            if bit_counter==bit_index{ //ya está en la posición deseada
                flag=true;
            }
            if flag && size_counter<size {
                v[i] &= !mask;
                size_counter+=1;
            }
            
            if size_counter==size {
                end=true;
                break;
            }
			bit_counter+=1;
            mask<<=1;
            //
        }
        if end{
            break;
        }
    }
}

fn clear_memory2(v: & mut Vec<u32>, size:u32, begin:u32){
	let (row,bit_index) = divide_module(begin,32);
	let mut size_counter:u32 =0;
	let mut flag = false;
	let mut end = false;

	for i in row as usize..v.len(){
		let mut mask: u32 = if i == row as usize { 1 << bit_index } else { 1 };
		for j in bit_index as usize..32{
			if size_counter < size {
				v[i] &= !mask;
				size_counter += 1;
				} else {
					end = true;
					break;
				}
			mask <<= 1; // Shift mask to the next bit
		}
		if end {
			break;
			}
		}
		
}

//funciones para el menú

/*fn add_process(v:& mut Vec<Process>){
    println!("Ingrese el tamaño del proceso");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let choice: u32 = input.trim().parse().expect("Please enter a number!");


}*/

fn read_input(sign:&str)->u32{
    loop {
        println!("{}",sign);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim().parse::<u32>() {
            Ok(size) => {
                // Aquí agregarías el proceso con el tamaño válido
                return size; // Convertimos a i32 para el tipo de retorno
            }
            Err(_) => println!("Entrada inválida. Por favor, ingrese un número."),
        }
    }

}

fn main(){
    if !memory_check() {
        println!("Memory Error!!!");
        return;
    }
    
    let chunks: u32 = T_MEMORY/CHUNK_SIZE;
    let mut processes: Vec<Process> = Vec::new();
    let mut rows = construct_memory(chunks); //hacer que rows sea de u32
    println!("el total de chunks es::{chunks}");
    
    loop{

    	//clearscreen::clear().expect("failed to clear screen");
        println!("1. Cargar proceso");
        println!("2. Eliminar proceso");
        println!("3. salir");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        let choice: u32 = input.trim().parse().expect("Please enter a number!");

        match choice {
            1 => {
                let aux = read_input("ingrese el tamaño del proceso");
                match find_space(&rows, aux) {
                    Some(index) => println!("Se encontró un espacio de {} ceros comenzando en el índice {}", aux, index),
                    None => {
                        println!("No se encontró un espacio de {} ceros", aux);
                        break;
                    },
                }
                

            },
            2 => println!("Opción inválida13"),
            3 => break,
            _ => println!("Opción inválida"),
        }
    }
    println!("Mostrando la memoria\n");
    show_memory(&rows);
    }
    /*let aux:u32 = 10;
    match find_space(&rows, aux) {
        Some(index) => println!("Se encontró un espacio de {} ceros comenzando en el índice {}", aux, index),
        None => println!("No se encontró un espacio de {} ceros", aux),
    }*/
    
   // let (a,b)=divide_module(145,32);
	/*
    println!("-------->");
	//index starts at zero
    fill_memory(& mut rows,10,32);
    //clear_memory(& mut rows,24,10

    show_memory(&rows);
    clear_memory2(&mut rows, 5,32);
    println!("-------->");
    show_memory(& rows);
	
    //show_memory(&rows);*/


    //maybe find space isn't working fine with the new functions so we need to try it out
