use godot::{prelude::*};
use rand::Rng;

struct SimTortaExtension;

#[gdextension]
unsafe impl ExtensionLibrary for SimTortaExtension {}

#[derive(GodotClass)]
#[class(base=Node)]
struct SimTorta {
    #[base]
    node: Base<Node>,
}

#[godot_api]
impl NodeVirtual for SimTorta {
    fn init(node: Base<Node>) -> Self {
        Self { node, }
    }
}

#[godot_api]
impl SimTorta {
    #[signal]
    fn mandar_output(output:String);
 
    #[func]
    fn empezar_simulacion(&mut self, gustos1:Array<f64>, gustos2:Array<f64>, t:u32, n:u32, info_perf_corte:bool, j2_malvado:bool) -> Array<VariantArray> {

        let mut resultados = Array::<VariantArray>::new();
        let mut ganancias = (0.0, 0.0);

        let j1 = Jugador { 
            gusto1: gustos1.get(0), 
            gusto2: gustos1.get(1), 
            gusto3: gustos1.get(2),
        };

        let j2 = Jugador { 
            gusto1: gustos2.get(0), 
            gusto2: gustos2.get(1), 
            gusto3: gustos2.get(2),
        };

        // Juego las partidas y me guardo los datos
        for i in 0..n {
            
            self.node.emit_signal("mandar_output".into(), &[Variant::from(format!("PARTIDA {:?}\n", i+1))]);
            
            let res_partida: (f64, f64) = self.jugar_partida(t, &j1, &j2, info_perf_corte, j2_malvado);
            ganancias = (ganancias.0 + res_partida.0, ganancias.1 + res_partida.1);

            resultados.push(array![res_partida.0.to_variant(), res_partida.1.to_variant(), ganancias.0.to_variant(), ganancias.1.to_variant()]);
        }

        self.node.emit_signal("mandar_output".into(), &[Variant::from("----------\n")]);
        self.node.emit_signal("mandar_output".into(), &[Variant::from(format!("TOTAL GANANCIA JUG 1: {:?}\n", ganancias.0))]);
        self.node.emit_signal("mandar_output".into(), &[Variant::from(format!("TOTAL GANANCIA JUG 2: {:?}\n", ganancias.1))]);
        
        resultados
    }

    //*
    #[func]
    fn testeo_prints(&mut self, n:u32) {

        for i in 0..n {
            self.node.emit_signal("mandar_output".into(), &[Variant::from(format!("TEST {:?}\n", i))]);
        }

    }
    //*/

    fn jugar_partida(&mut self, tam_torta:u32, j1:&Jugador, j2:&Jugador, info_perf_corte:bool, j2_malvado:bool) -> (f64, f64) {
        let torta = self.crear_torta(tam_torta);

        // print!("La torta creada es: {:?}\n", &torta);
        self.node.emit_signal("mandar_output".into(), &[Variant::from(format!("La torta creada es: {:?}\n", &torta))]);

        let corte = self.realizar_corte(&torta, j1, j2, info_perf_corte);

        // print!("La porcion cortada es: {:?}\n", &corte);
        self.node.emit_signal("mandar_output".into(), &[Variant::from(format!("La porcion cortada es: {:?}\n", &corte))]);

        let trozos_elegidos = if j2_malvado {
            self.elegir_trozos(&torta, corte, j1)
        } else {
            self.elegir_trozos(&torta, corte, j2)
        };

        // print!("Trozos elegidos: {:?}\n", &trozos_elegidos);
        self.node.emit_signal("mandar_output".into(), &[Variant::from(format!("Trozos elegidos: {:?}\n", &trozos_elegidos))]);

        let ganancia1:f64 = self.calcular_ganancia(&torta, &trozos_elegidos.0, j1);
        let ganancia2:f64 = self.calcular_ganancia(&torta, &trozos_elegidos.1, j2);

        self.node.emit_signal("mandar_output".into(), &[Variant::from(format!("Ganancia J1: {:?}\n", &ganancia1))]);
        self.node.emit_signal("mandar_output".into(), &[Variant::from(format!("Ganancia J2: {:?}\n", &ganancia2))]);

        (ganancia1, ganancia2)
    }

    // Crea una torta del tamaño especificado
    fn crear_torta(&self, tam:u32) -> Vec<u8> {
        let mut rng = rand::thread_rng();
        (0..tam).map(|_| rng.gen_range(1..4)).collect()
    }

    // Decide un par de indices de acuerdo a los gustos de un jugador
    // Retorna los indices que corresponden al corte hecho por el jugador
    fn realizar_corte(&self, torta:&Vec<u8>, j1:&Jugador, j2:&Jugador, info_perf:bool) -> (usize, usize) {
        if info_perf {
            self.mejor_corte_contemplando_dos_jugadores(torta, j1, j2)
        }
        else {
            self.mejor_corte_contemplando_un_jugador(torta, j1)
        }
    }

    // Corte para probar las otras funciones, no toma en cuenta los gustos de los jugadores
    fn corte_a_la_mitad(&self, torta:&Vec<u8>) -> (usize, usize) {
        (0, torta.len()/2)
    }

    // Realiza el corte en base a los gustos del jugador 1, contemplando los gustos del jugador 2
    fn mejor_corte_contemplando_dos_jugadores(&self, torta:&Vec<u8>, j1:&Jugador, j2:&Jugador) -> (usize, usize) {
        
        println!("CORTE CON INFO PERF");

        let tam_torta = torta.len();
        
        let matriz_ganancias_j1 = self.generar_matriz_ganancias_de_jugador(torta, j1);
        let matriz_ganancias_j2 = self.generar_matriz_ganancias_de_jugador(torta, j2);

        println!("Matriz 1: {:?}", matriz_ganancias_j1);
        println!("Matriz 2: {:?}", matriz_ganancias_j2);

        let mut mejor_indice: (usize, usize) = (0, 1);
        let mut mejor_ganancia = matriz_ganancias_j1[0][0];

        for fila in 0..tam_torta {
            for col in fila..tam_torta {
                
                if matriz_ganancias_j1[fila][col] >= mejor_ganancia && 1.0 - matriz_ganancias_j2[fila][col] >= 0.5 {
                    mejor_ganancia = matriz_ganancias_j1[fila][col];
                    mejor_indice = (fila, col+1);
                }
                else if 1.0 - matriz_ganancias_j1[fila][col] >= mejor_ganancia && matriz_ganancias_j2[fila][col] >= 0.5 {
                    mejor_ganancia = matriz_ganancias_j1[fila][col];
                    mejor_indice = (fila, col+1);
                }

            }
        }

        println!("Mejor indice: {:?}", mejor_indice);
        println!("Mejor ganancia: {:?}", mejor_ganancia);

        mejor_indice
    }
    
    fn mejor_corte_contemplando_un_jugador(&self, torta:&Vec<u8>, j1:&Jugador) -> (usize, usize) {
        
        println!("CORTE SIN INFO PERF");
        
        let tam_torta = torta.len();
        
        let matriz_ganancias_j1 = self.generar_matriz_ganancias_de_jugador(torta, j1);

        println!("Matriz 1: {:?}", matriz_ganancias_j1);

        let mut mejor_indice: (usize, usize) = (0, 1);
        let mut mejor_ganancia = matriz_ganancias_j1[0][0];

        for fila in 0..tam_torta {
            for col in fila..tam_torta {
                
                if matriz_ganancias_j1[fila][col] >= mejor_ganancia && 1.0 - matriz_ganancias_j1[fila][col] >= 0.5 {
                    mejor_ganancia = matriz_ganancias_j1[fila][col];
                    mejor_indice = (fila, col+1);
                }
                else if 1.0 - matriz_ganancias_j1[fila][col] >= mejor_ganancia && matriz_ganancias_j1[fila][col] >= 0.5 {
                    mejor_ganancia = matriz_ganancias_j1[fila][col];
                    mejor_indice = (fila, col+1);
                }

            }
        }

        println!("Mejor indice: {:?}", mejor_indice);
        println!("Mejor ganancia: {:?}", mejor_ganancia);

        mejor_indice
    }

    fn generar_matriz_ganancias_de_jugador(&self, torta:&Vec<u8>, jug:&Jugador) -> Vec<Vec<f64>> {
        let tam_torta = torta.len();

        let mut tabla = vec![vec![-1.0; tam_torta]; tam_torta];

        let cant_1 = cant_elemento(torta, 1u8) as f64;
        let cant_2 = cant_elemento(torta, 2u8) as f64;
        let cant_3 = cant_elemento(torta, 3u8) as f64;

        let proporciones = |x| {
            match x {
                1u8 => 1.0/cant_1,
                2u8 => 1.0/cant_2, 
                3u8 => 1.0/cant_3,
                _ => 0.0
            }
        };

        for fila in 0..tam_torta {
            for col in fila..tam_torta {

               // Caso base, la diagonal 
                if col == fila {
                    tabla[fila][col] = proporciones(torta[col]) * jug.gustos(torta[col]);
                }
                else {
                    // Caso inductivo
                    tabla[fila][col] = tabla[fila][col-1] + proporciones(torta[col]) * jug.gustos(torta[col]);
                }

            }
        }

        tabla

    }

    // Elije un trozo de la torta en base a los gustos del jugador
    // Retorna vectores con los indices de los objetos de la torta que corresponden a cada corte
    // y el indice elegido por el jugador
    fn elegir_trozos(&self, torta:&Vec<u8>, corte:(usize, usize) , j:&Jugador) -> (Vec<usize>, Vec<usize>) {
        let rango = corte.0 .. corte.1;

        let r1: Vec<usize> = rango.clone().into_iter().collect();
        let r2: Vec<usize> = (0 .. torta.len()).filter(|i| !&rango.contains(i)).collect();

        let ganancia1:f64 = self.calcular_ganancia(torta, &r1, j); // r1.iter().map(|i| j.gustos(torta[*i])).sum();
        let ganancia2:f64 = self.calcular_ganancia(torta, &r2, j); // r2.iter().map(|i| j.gustos(torta[*i])).sum();

        if ganancia1 > ganancia2 {
            (r2, r1)
        }
        else {
            (r1, r2)
        }
    }

    fn calcular_ganancia(&self, torta:&Vec<u8>, partes:&Vec<usize>, j:&Jugador) -> f64 {
        
        let cant_1_total = cant_elemento(torta, 1u8) as f64;
        
        let cant_2_total = cant_elemento(torta, 2u8) as f64;
        
        let cant_3_total = cant_elemento(torta, 3u8) as f64;
        
        let pedazo_del_jugador:Vec<u8> = torta.iter().enumerate()
                                    .filter(|&(i, _)| partes.contains(&i))
                                    .map(|(_, e)| *e).collect();
        
        let cant_1_jugador = cant_elemento(&pedazo_del_jugador, 1u8) as f64;
        
        let cant_2_jugador = cant_elemento(&pedazo_del_jugador, 2u8) as f64;
        
        let cant_3_jugador = cant_elemento(&pedazo_del_jugador, 3u8) as f64;

        let mut ganancia:f64 = 0f64;

        ganancia += if cant_1_total == 0.0 {0f64} else {(cant_1_jugador/cant_1_total) * j.gustos(1u8)};
        ganancia += if cant_2_total == 0.0 {0f64} else {(cant_2_jugador/cant_2_total) * j.gustos(2u8)};
        ganancia += if cant_3_total == 0.0 {0f64} else {(cant_3_jugador/cant_3_total) * j.gustos(3u8)};

        ganancia
    }
}

//

struct Jugador {
    gusto1: f64,
    gusto2: f64,
    gusto3: f64,
}

impl Jugador {
    fn gustos(&self, g:u8) -> f64 {
        match g {
            1u8 => self.gusto1,
            2u8 => self.gusto2,
            3u8 => self.gusto3,
            _ => 0f64,
        }
    }
}

fn cant_elemento(fuente:&Vec<u8>, elemento:u8) -> usize {
    fuente.iter().filter(|e| **e == elemento).count()
}
