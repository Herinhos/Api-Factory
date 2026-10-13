use rand::RngExt;
use std::error::Error;
use std::io;
use std::io::Write;

// ----- utils -----

/// Flush stdout macro.
/// No error handling, unwraps.
#[macro_export]
macro_rules! ioflush {
    () => {
        io::stdout().flush().unwrap();
    };
}

/// Input macro with/without echo.
/// No error handling, unwraps.
#[macro_export]
macro_rules! input {
    ($msg:expr) => {{
        print!("{}", $msg);
        ioflush!();
        io::stdin().read_line(&mut SHELL_INPUT_BUFFER).unwrap();
        SHELL_INPUT_BUFFER.trim().to_string()
    }};

    () => {{
        io::stdin().read_line(&mut SHELL_INPUT_BUFFER).unwrap();
        SHELL_INPUT_BUFFER.trim().to_string()
    }};
}

fn get_shell_input(input_buffer: &mut String) -> io::Result<&str> {
    io::stdout().flush()?;
    input_buffer.clear();
    io::stdin().read_line(input_buffer)?;
    Ok(input_buffer.trim())
}

fn output_to_shell(message: &str) {
    println!("> {}", message);
    ioflush!();
}

fn get_random_in_range(from: usize, to: usize) -> usize {
    rand::rng().random_range(from..=to)
}

// ----- data -----

/// Representa um lançamento de dados com um número especificado de vezes e lados.
struct DiceRoll {
    times: usize,
    sides: usize,
}
/// Implementações da estrutura DiceRoll.
///
/// Preste atenção que apenas o mínimo necessário de validação é feita.
impl DiceRoll {
    /// Cria uma instância de DiceRoll a partir de uma string.
    /// Rust meio avançado aqui, mas a lógica é simples:
    /// dividimos a string em duas partes usando 'd' ou 'D' como separador,
    /// onde antes do 'd' ou 'D' temos o número de vezes e depois temos o número de lados.
    ///
    /// Por padrão a função aceita qualquer string e só realiza o mais básico de validação.
    /// No caso, apenas valida se a string é divisível por 'd' ou 'D'.
    fn from_string(input: &str) -> Result<Self, Box<dyn Error>> {
        // se não for possível dividir a string em duas partes usando 'd' ou 'D' como separador,
        // retorna um erro.
        if let Some((times, sides)) = input.split_once(['d', 'D']) {
            // se não for possível converter as partes em números, retorna um erro.
            // o '?' aqui significa que pode dar erro e esse erro está sobre o controle dessa função.

            // "d20 == 1d20"
            let times = if times.is_empty() {
                1
            } else {
                times.parse::<usize>()?
            };
            let sides = sides.parse::<usize>()?;

            if sides == 0 {
                return Err("Invalid Input. Input implies dice has 0 amount of sides.".into());
            }

            // se tudo estiver correto, retorna uma instância de DiceRoll.
            Ok(Self { times, sides })
        } else {
            // se qualquer parte for inválida (ou melhor se qualquer erro ocorrer durante),
            // retorna a string abaixo como um Erro.
            // Isso dá mais controle sobre como os erros são tratados dentro da aplicação.
            Err("Invalid format".into())
        }
    }

    /// Rola todos os dados dentro do
    fn roll_all(&self) -> usize {
        let mut result = 0;
        for _ in 0..self.times {
            result += get_random_in_range(1, self.sides);
        }
        result
    }
}

// ----- "API" -----

/// Aqui é onde a definição de API se encaixa.
///
/// Mesmo sendo apenas uma função simples de parsing e filtragem de dados, ela conecta
/// as partes de input do usuário, manipulação de dados e output para o usuário.
fn parse_dice_input(input_buffer: &str) -> Result<(), Box<dyn Error>> {
    // Mais uma vez verificamos se o input está nos padrões da notação.
    // essa parte é sensível ao tipo de estrutura dos dados que serão recebidos
    // e da arquitetura geral da aplicação.
    // pensem que num tráfego grande de dados essas validações podem custar muito mais tempo.

    let input = input_buffer.trim();

    if input.is_empty() {
        return Err("Input is empty".into());
    }
    if !input.contains(['d', 'D']) {
        return Err("Invalid Format".into());
    }

    let dice = DiceRoll::from_string(input)?;

    let total = dice.roll_all();

    output_to_shell(format!("{}", total).as_str());

    Ok(())
}

// ----- main -----

fn main() {
    let mut input_string: String = String::new();

    println!("----- Transferência de Dados -----");
    println!("----- Digite Sair ou sair para sair do programa -----");

    loop {
        print!("> Digite uma expressão em notação de Dados: \n> ");
        let input = get_shell_input(&mut input_string).expect("Error!");

        match input {
            "sair" => {
                break;
            }
            "Sair" => {
                break;
            }
            _ => {}
        }

        parse_dice_input(input).expect("Error!");
    }
}
