
# Upjar [![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)

Ferramenta para realizar a criação ou atualização, via linha de commando, de modulo java dentro do sankhya.


## Instalação

Instale fazendo o dowload de uma das [releases](https://github.com/Luisricardo2825/upjar/releases), extraia o arquivo em algum diretorio e adicione o diretorio criado a variavel `PATH` do sistema.



    
## Uso/Exemplos
  Crie um arquivo de configuração com a exstensão `.json`, como descrito  em [example](https://github.com/Luisricardo2825/upjar/blob/master/example/build.jsonc)
  ```json
{
  "url": "String", // Url da base
  "user": "String", // usuário login
  "password": "String", // Senha login
  "jarFilePath": "String", // Caminho do jar a ser enviado
  "resourceId": "String", // identificador do modulo a ser criado/atualizado
  "resourceDesc": "String" // Descrição do modulo a ser criado. OBS: Só é utilizado durante a CRIAÇÃO, não atualizando a descrição do modulo já existente
}
  ```
Após a criação do arquivo, copie o caminho do arquivo de configuração e utilize o comando: 
```shell
upjar <caminho>
```

## Parametros do arquivo de configuração

| Parâmetro   | Tipo       | Descrição                           |
| :---------- | :--------- | :---------------------------------- |
| `url` | `string` | **Obrigatório**. Url da base |
| `user`      | `string` | **Obrigatório**. Usuário para login |
| `password`      | `string` | **Obrigatório**. Senha para login |
| `jarFilePath`      | `string` | **Obrigatório**. Caminho do jar a ser enviado |
| `resourceId`      | `string` | **Obrigatório**. identificador do modulo|
| `resourceDesc`      | `string` | **Obrigatório**. Descrição do modulo a ser criado.|

## Autores

- Ricardo Alves ([@Luisricardo2825](https://github.com/Luisricardo2825))
