# Advent of Code 2022

[Live Streams](https://www.youtube.com/watch?v=XmybJ1GlHUk&list=PLoSY6azqHO7AsDMT2WSXb68tGvvKRO8Wj)

## How to Install Livebook

I use Elixir 1.14.2 here. I recommend using [asdf](https://asdf-vm.com/guide/getting-started.html)
and the [Elixir asdf plugin](https://github.com/asdf-vm/asdf-elixir) to install it.

To install Elixir's Livebook (given Elixir 1.14+ installation):

```sh
mix do local.rebar --force, local.hex --force
mix escript.install hex livebook
# if using asdf:
# asdf reshim elixir
```

Running:

```sh
livebook server
```

## Julia

I'm using Julia 1.8.3 here.

You can download Julia [here](https://julialang.org/downloads/). After that, put it in your `PATH`, and you're done.

## Python

I use Python 3.10 + pipenv for managing Python dependencies.

[Install Instructions](https://pipenv.pypa.io/en/latest/#install-pipenv-today).

To create a venv:

```sh
pipenv sync --dev
```

You can get the path to the created venv with `pipenv --venv`.

If you're using VS Code, use `Python: Select Interpreter` from the command pallete to select the venv you've just
created.
