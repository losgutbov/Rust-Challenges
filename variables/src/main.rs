//NECESSÁRIO USAR O let PARA CRIAR UMA VARIÁVEL QUE POR PADRÃO É IMUTÁVEL, PARA TORNAR A VARIÁVEL MUTÁVEL É NECEESSÁRIO ADICIONAR O mut.
// MESMO COM O mut SÓ É POSSÍVEL MUDAR A VALOR DA VARIÁVEL PARA OUTRO DO MESMO TIPO, OU SEJA, INT PARA INT E NÃO INT PARA STRING.
// É POSSÍVEL USAR O SHADOW, QUE É COMO UMA TÉCNICA PARA USAR O MESMO NOME DA VARIÁVEL, MAS COM OUTROS VALORES, MAS NESSE CASO É OUTRO ESPAÇO DE MEMÓRIA, SÓ O NOME É O MESMO.
fn main() {
    let mut x = 5;
    println!("O valor de x é: {x}");
    x = 6;
    println!("O valor de x é: {x}");

    // SHADOWING
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("O valor de y no escopo interno é: {y}");
    }

    println!("O valor de y é: {y}");
}
