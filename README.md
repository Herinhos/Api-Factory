# API Factory

## Um guia e template compreensivo para você montar suas APIs Python com as ferramentas mais novas do mercado

 Este guia e template por enquanto só dizem respeito a ==FastAPI com SQLAlchemy==.

 Estaremos usando o gerenciador de pacotes uv quando possível, assim como as outras ferramentas da Astral, como o Ruff e o Ty.
 Use a seção [pré-requisitos](#prerequisites) para se referir a instalação das ferramentas necessárias.

### Prerequisitos e como obte-los {#prerequisites}

 Se certifique que tenha o Python instalado em sua máquina. Baixe a versão mais nova no link abaixo:

 `https://www.python.org/downloads/`

 Apenas siga as instruções do instalador. Você pode verificar a instalação do Python com o comando abaixo no terminal:

 ```cmd
 python --version
 ```

 Agora baixe e instale o uv. Para aqueles que trabalham com proxies recomendo utilizar o winget:

 ```cmd
 winget install astral-sh.uv --source winget
 ```

 Você pode verificar a instalação com:

 ```cmd
 uv --version
 ```

 Após ter realizado todos os passos acima você poder seguir em frente com o [setup padrão](#fastapi)

### Opcional

 Tendo cumprido os passos acima, ~teoricamente~ não há necessidade de instalar mais nenhuma ferramenta, porém é possivel deixar o Ruff e o Ty instalado globalmente com:

 ```cmd
 uv tool install ruff@latest
 ```

 e

 ```cmd
 uv tool install ty@latest
 ```

### Setup padrão: FastAPI com SQLAlchemy {#fastapi}

 Clone este repositório no local que desejar.

 Depois use o comando abaixo dentro da pasta do repositorio clonado.

 ``` cmd
 uv sync
 ```

 E pronto! Você pode começar a contruir sua API da forma que precisar!
