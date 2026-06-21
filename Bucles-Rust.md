Los bucles tienen las siguiente nomenclatura:

```rust
for i in 0..5 {
    println!("{}", i);
}
```

Esto nos devolverá `01234`

Esto lo podemos usarpara muchos casos como por ejemplo sumar todos los números del 1 al 100:

```rust

let mut sum_numbers = 0;
for i in 1..=100 {
    sum_numbers += i;
}

println!("{}", sum_numbers);
```

Esto primero recorrerá todos los números entre 1 y 100 (incluyendo el 100 debido al signo ..=) y los sumará todos, luego imprimirá la variable sum_numbers

Si por alguna razón quieres crear un bucle sin usar una variable (i), deberías añadir un guion bajo al principio del nombre: \_i. Esto le dirá al compilador que está bien que no se use, y evitará que el programa genere una advertencia:

```rust
for _i in 0..5 {
    println!("Hello!");
}
```
