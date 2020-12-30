#[derive(Debug)]
struct Centros {
    c_centro: String,
    n_centro: String,
}

#[derive(Debug)]
struct Ciclos {
    c_ciclo: String,
    n_ciclo: String,
}

fn main() {
    println!("Hello, world!");

    let mut v_centros = Vec::new();
    v_centros.push(Centros {
        c_centro: String::from("1"),
        n_centro: String::from("centro pepito 1"),
    });
    v_centros.push(Centros {
        c_centro: String::from("2"),
        n_centro: String::from("ciclo juanito 2"),
    });
    v_centros.push(Centros {
        c_centro: String::from("3"),
        n_centro: String::from("ciclo juanito 3"),
    });

    let mut v_ciclos = Vec::new();
    v_ciclos.push(Ciclos {
        c_ciclo: String::from("1"),
        n_ciclo: String::from("ciclo informatica 1"),
    });
    v_ciclos.push(Ciclos {
        c_ciclo: String::from("2"),
        n_ciclo: String::from("ciclo sanidad 2"),
    });
    v_ciclos.push(Ciclos {
        c_ciclo: String::from("3"),
        n_ciclo: String::from("ciclo peluqueria 3"),
    });

    let r_centros: Vec<&Centros> = v_centros.iter().filter(|x| x.n_centro.contains("juanito")).collect();
    let r_ciclos: Vec<&Ciclos> = v_ciclos.iter().filter(|x| pertenece_a(x, &r_centros)).collect();

    println!("{:?}", &r_centros);
    println!("{:?}", r_ciclos);
}

fn pertenece_a(x: &Ciclos, r_centros: &Vec<&Centros>) -> bool {
    let mut found = false;
    for centro in r_centros {
        if x.c_ciclo.contains(&centro.c_centro) {
            found = true;
        }
    }
    found
}