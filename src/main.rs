// las referencias son punteros que apuntan a un dato que le pertenece a otro owner
//una manera las conscisa de describirlo es de la siguiente manera:
//una variable string tiene 3 componentes (ptr,len,capacity) y este ptr apunta a la localidad del heap donde se encuentran los datos
//una referencia a esta variable apunta a esos 3 componentes para localizar los datos, puede de esta manera ganar acceso
//pero limitado por que no es el owner, solo le "presto" (borrowing) su direccion.

//la accion de crear una referencia se conoce como borrowing, ya que la referencia no es el owner
//las referencias se indican con & antes del tipo (parametros) y antes del nombre (argumentos y variables).

fn main() {
    let s1 = String::from("This ref is rusty!");
    //no se mueve el valor de s1, por que estamos enviando una referencia de s1, un prestamo
    //las referencias no pueden ser modificadas ya que ellas no son el owner, por defecto son inmutables.
    let len = calculate_length(&s1);
    //s1 no mueve su valor por que se paso una referencia.
    //podemos seguir usando s1
    println!("la cadena {} tiene una logitud de {}", s1,len);

    //una referencia es mutable por defecto, lo que impide que se modifique la informacion
    //pero al igual que en las variables se puede marcar una referencia como mutable con &mut
    
    //el siguiente metodo recibe una referencia mutable, por que cambia su informacion (pero no el owner).
    //para que se pueda generar una referencia mutable, el owner debe ser mutable en su declaracion.
    let mut s2 = String::from("Rust");
    rustify(&mut s2);
    println!("{s2}");

    //las referencias impiden cambiar un valor y los problemas relacionados a la concurrencia, de ahi que sean mutables.
    //las referencias mutables permiten cambiar los datos, pero el owner se mantiene (respetando un unico owner).
    //para evitar los problemas de concurrencia, solo puede haber una referencia mutable a una variable en un mismo contexto.

    let s3 = String::from("una cadena");
    //se pueden generar multiples referencias inmutables
    let r1 = &s3;
    let r2 = &s3;

    //pero no se pueden declarar multiples referencias mutables, para evitar race condition
    let mut s4 = String::from("otra cadena");
    let r3 = &mut s4;
    let r4 = &mut s4; // no permitido, el error sale al usar la variable debido al NLL (non lexical lifetimes).

    //r4 es una referencia no permitida por que solo puede haber una referencia mutable del mismo owner en el mismo scope.
    //println!("{},{}",r3,r4); // no compila

    //tampoco se puede declarar una referencia mutable si existen referencias inmutables en el scope
    let mut s5 = String::from("Rust is cool");
    let r5 = &s5; //referencia inmutable
    let r6 = &mut s5; //referencia mutable, igualmente sujeto al NLL

    //r6 es una referencia no permitida ya que existe una referencia al mismo owner en el scope.
    //println!("{},{}",r5,r6); //no compila

    //en nuevo contexto si esta permitido generar referencias (siempre y cuando no sean simultaneas)
    let mut s6 = String::from("Rust is");
    {
        //nuevo scope
        let r7 = &mut s6; //si esta permitido
        r7.push_str(" cool");
    } //en este punto r7 sale del scope y se pueden crear nuevas referencias

    let r7 = &mut s5; //es valido por que r7 salio de scope y no puede generar una data race
    println!("el valor de r7 es {r7}");



    //no se puede dar una mezcla de referencias mutables e inmutables en el mismo scope para el mismo owner
    //por que esto generaria race data condition que es alguna delas siguientes situaciones:
    //- 2 o mas pointers acceden a la misma informacion al mismo tiempo
    //- Al menos 1 de los punteros esta siendo usado para escribir los datos.
    //- no existe un mecanismo para sincronizar el acceso a los datos.

    //dangling references

    //una referencia 'dangling' es una referencia que apunta a una locacion en memoria invalida o cuyo propietario o data sufrio cambios
    //y dicha referencia no se actualizo y apunta a nada o a un lugar que ya no contiene la informacion correcta.
    //lo mismo sucede con los dangling pointers

    //En Rust, el compilador se asegura que no exista este tipo de referencias, como en el siguiente ejemplo

    //let dangling_reference = dangle(); //El compilador de Rust no permite las dangling references.

    //la alternativa es retornar el valor y no la referencia, una operacion de move, donde se cambia el owner al nuevo scope.
    //En Rust las referencias deben de ser siempre validas, algo genial en comparacion con otros lenguajes.
}

//el siguiente metodo recibe una referencia de un string, por lo que el owner no pierde su ownership.
//el valor que reciba s no pasa a ser de su ownership, solo es una referencia que puede consultar.
//el owner original no pierde su ownership.
fn calculate_length(s: &String) -> usize{
    s.len() // el resultado de la funcion es un entero, este si se puede asignar a un nuevo owner desde donde se llamo la funcion.
}

//el parametro s es una referencia mutable, esto quiere decir que puede modificar su valor
fn rustify(s: &mut String) {
    s.push_str(" 🦀");
}

//El compilador indicara que se debe utilizar un lifetime especifier
//la referencia que se retorna no tiene un valor al cual haver borrow

//fn dangle() -> &String { //dangle retorna una referencia a un String
    //let s = String::from("soy temporal");
    //let r = &s; //obtiene una referencia de s
    //r solo es valida mientras este en el scope de s
    //al retornar y terminar el scope r apuntaria a una locacion invalida, pero rust lo detecta y no permite dangling references
    //r
//}