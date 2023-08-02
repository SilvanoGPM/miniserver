<h1 align="center">Miniserver</h1>

<p align="center">Servidor web simples implementado com Rust</p>

## ⚙️ Requisitos

- Rust 1.63.0 ou superior.
- Cargo 1.63.0 ou superior.

## 🤖 Utilização

Para executar o projeto utilize o comando:

```
cargo run
```

Agora em um navegador, acesse o endereço `localhost:7878` e você deve receber um JSON como resposta de sucesso.
<br />
Caso queria simular uma atraso no lado do servidor, acesse o endereço `localhost:7878/sleep`, você deve receber o mesmo JSON porém com um atraso de 5 segundos.
<br />
Qualquer outro endereço deve retornar um HTML.

## :heart: Agradecimentos

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
