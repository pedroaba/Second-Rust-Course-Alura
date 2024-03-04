#[allow(dead_code)]
fn matrix() {
    let _matrix: [[f32; 3]; 2] = [
        [0.0, 1.2,  0.1],
        [1.3, 0.3, 1.4]
    ];

    for row in _matrix {
        for item in row {
            println!("{item}")
        }
    }
}

fn is_week_end(weekday: WeekDay) -> bool {
    match weekday {
        WeekDay::Sunday | WeekDay::Saturday => true,
        _ => false
    }
}

#[allow(dead_code)]
enum WeekDay {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday
}

#[allow(dead_code)]
enum Color {
    Red,
    Blue,
    Green,
    RgbColor(u8, u8, u8),
    CymkColor{cyan: u8, magenta: u8, yellow: u8, black: u8}
}

#[allow(dead_code)]
fn colors() {
    let color = Color::CymkColor {cyan: 70, magenta: 90, yellow: 80, black: 80};

    println!("Cor = {}", match color {
        Color::Red => "Vermelho",
        Color::Green => "Verde",
        Color::Blue => "Azul",
        Color::RgbColor(0, 0, 0) |
            Color::CymkColor{cyan: _, magenta: _, yellow: _, black: 255} => "Preta",
        Color::RgbColor(_, _, _) => "RGB desconhecido",
        Color::CymkColor{cyan: _, magenta: _, yellow: _, black: _} => "CYMK desconhecido"
    })
}

fn optional_content() {
    let content_file = read_file(String::from(""));

    match &content_file {
        Some(value) => println!("{}", value),
        None => println!("Arquivo não existe!")
    }

    println!("{:?}", &content_file);

    if let Some(value) = content_file {
        println!("{}", value);
    }
}

fn read_file(_: String) -> Option<String> {
    Some(String::from("Conteúdo do arquivo"))
}

fn vectors() {
    let mut grades: Vec<f32> = Vec::with_capacity(4);
    grades.push(120.0);
    grades.push(120.0);
    grades.push(120.0);

    println!("{:?}", grades);
    println!("Capacidade = {}", grades.capacity());

    grades.push(6.8);
    println!("{:?}", grades);
    println!("Capacidade = {}", grades.capacity());

    println!("Nota 6 = {}", match grades.get(7) {
        Some(&n) => n,
        None => 0.0
    });

    // while let Some(grade) = grades.pop() {
    //     println!("Valor removido = {}", grade);
    // }

    for grade in &grades {
        println!("Nota = {}", grade);
    }
}

struct Conta {
    titular: Titular,
    saldo: f64
}

impl Conta {
    fn sacar(&mut self, valor: f64) {
        self.saldo -= valor
    }
}

struct Titular {
    nome: String,
    sobrenome: String
}

fn conta_corrente() {
    let titular = Titular{ nome: String::from("Pedro"), sobrenome: String::from("Augusto") };
    let mut conta: Conta = Conta{
        titular,
        saldo: 120.0
    };

    conta.sacar(60.0);

    println!("Dados da conta: Titular = {} {}, Saldo = {}", conta.titular.nome, conta.titular.sobrenome, conta.saldo);
}

fn main() {
    let grades: [f32; 4] = [10.0, 8.0, 9.5, 6.0];
    let integer: usize = 0;

    println!("{}", grades[integer]);

    for index in 0..grades.len() {
        println!("Nota {}: {}", index + 1, grades[index]);
    }

    println!("É fim de semana? {}", is_week_end(WeekDay::Friday));
    optional_content();
    vectors();

    conta_corrente();
}
