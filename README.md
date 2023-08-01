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

Agora em um navegador, acesse o endereço `localhost:7878`.
Se tudo der certo, você vai ver um arquivo JSON como resposta.
Caso queria emular uma rota que demora para responder, você pode acessar `localhost:7878/sleep`, isso fará o servidor esperar 5 segundos antes de responder.

## :heart: Agradecimentos

- [The Rust Programming Language](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
