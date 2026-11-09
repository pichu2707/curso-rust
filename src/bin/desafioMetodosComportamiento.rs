// Crea una estructura BankAccount con un solo campo: balance (i32).
//
// Añade un bloque de implementación para BankAccount con tres métodos:
//
// deposit: recibe &mut self y un amount: i32, y añade la cantidad al saldo
// withdraw: recibe &mut self y un amount: i32, y resta la cantidad del saldo
// get_balance: recibe &self y devuelve el saldo actual como i32
// Recibirás tres entradas:
//
// Primera línea: el saldo inicial (i32)
// Segunda línea: la cantidad depositada (i32)
// Tercera línea: la cantidad retirada (i32)
// Crea una instancia mutable de BankAccount con el saldo inicial, después deposita la cantidad indicada, retira la cantidad indicada e imprime el saldo final.
//
// Formato de salida esperado:
//
// {final_balance}

use std::io;

// TODO: Define la struct BankAccount aquí


// TODO: Añade el bloque de implementación para BankAccount con los métodos deposit, withdraw y get_balance


fn main() {
    let mut input = String::new();
    
    // Lee el saldo inicial
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let initial_balance: i32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    
    // Lee la cantidad del depósito
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let deposit_amount: i32 = input.trim().parse().expect("Invalid number");
    
    input.clear();
    
    // Lee la cantidad del retiro
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let withdraw_amount: i32 = input.trim().parse().expect("Invalid number");
    
    // TODO: Crea una instancia mutable de BankAccount con initial_balance
    // TODO: Llama a deposit con deposit_amount
    // TODO: Llama a withdraw con withdraw_amount
    // TODO: Imprime el saldo final usando get_balance
} 
