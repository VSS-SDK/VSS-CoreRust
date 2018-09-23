# VSS-CoreRust [![License: GPL v3](https://img.shields.io/badge/License-GPL%20v3-blue.svg)][gpl3][![Build Status](https://api.travis-ci.com/VSS-SDK/VSS-CoreRust.svg?branch=master)][travis]

[![Trello](https://img.shields.io/badge/Trello-SDK-blue.svg)][vss-sdk-trello]
[![Slack](https://img.shields.io/badge/Slack-Channel-551a8b.svg)][slack]

O VSS-CoreRust é a versão em Rust do VSS-Core.
Esse crate, contém interfaces de comunicação, modelos que representam o domínio do problema de
futebol de robôs e métodos uteis. Os pacotes trafegam utilizando ZeroMQ e são serializados
utilizando Protobuf.

Mais informações podem ser encontradas em [VSS-SDK][vss-sdk].

## Desenvolvimento
Compilando
```
$ cargo build
```

Testando
```
$ cargo test
```

## Exemplos
Exemplo em C++: [VSS-SampleCpp][samplecpp]

# Licença

Esse código está sob licença [GNU GENERAL PUBLIC LICENSE Version 3][gpl3], cujo uma cópia em texto pode
ser encontrada em [LICENSE.txt](LICENSE.txt).

Você pode utilizar esse código. Caso o faça, nos informe.

[gpl3]: http://www.gnu.org/licenses/gpl-3.0/
[vss-sdk]: https://vss-sdk.github.io
[samplecpp]: https://github.com/SIRLab/VSS-SampleCpp
[travis]: https://travis-ci.com/VSS-SDK/VSS-CoreRust
[vss-sdk-trello]: https://trello.com/b/b4dVV6ug/vss-sdk
[slack]: https://vss-sdk.slack.com
