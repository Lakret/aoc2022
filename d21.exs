defmodule D21 do
  def parse_input(path) do
    File.read!(path)
    |> String.trim_trailing()
    |> String.split("\n")
    |> Enum.map(fn x ->
      [name, op] = String.split(x, ": ")

      case Integer.parse(op) do
        {num, _} when is_integer(num) ->
          {name, num}

        :error ->
          [a, op, b] = String.split(op, " ")
          {name, {op, a, b}}
      end
    end)
    |> Map.new()
  end

  def eval_op("+", a, b), do: a + b
  def eval_op("-", a, b), do: a - b
  def eval_op("*", a, b), do: a * b
  def eval_op("/", a, b), do: div(a, b)

  def eval_once(input, env \\ %{}) do
    Enum.reduce(input, env, fn
      {name, num}, env when is_integer(num) ->
        Map.put(env, name, num)

      {name, {op, a, b}}, env ->
        if Map.has_key?(env, a) and Map.has_key?(env, b) do
          {a, b} = {env[a], env[b]}
          Map.put(env, name, eval_op(op, a, b))
        else
          env
        end
    end)
  end

  def eval(input, stop_monkey, env \\ %{}) do
    if Map.has_key?(env, stop_monkey) do
      env[stop_monkey]
    else
      env = eval_once(input, env)
      eval(input, stop_monkey, env)
    end
  end

  def build_relation_once(input, env \\ %{}) do
    Enum.reduce(input, env, fn
      {"humn", _num}, env ->
        Map.put(env, "humn", :humn)

      {name, num}, env when is_integer(num) ->
        Map.put(env, name, num)

      {name, {op, a, b}}, env ->
        if Map.has_key?(env, a) and Map.has_key?(env, b) do
          {a, b} = {env[a], env[b]}

          if name == "root" do
            Map.put(env, name, {"=", a, b})
          else
            if is_integer(a) && is_integer(b) do
              Map.put(env, name, eval_op(op, a, b))
            else
              Map.put(env, name, {op, a, b})
            end
          end
        else
          env
        end
    end)
  end

  def build_relation(input, env \\ %{}) do
    if Map.has_key?(env, "root") do
      env["root"]
    else
      env = build_relation_once(input, env)
      build_relation(input, env)
    end
  end

  @rev_op %{"+" => "-", "*" => "/", "-" => "+", "/" => "*"}

  def infer(input) do
    {"=", left, right} = D21.build_relation(input)

    cond do
      is_integer(left) -> infer(input, left, right)
      is_integer(right) -> infer(input, right, left)
      true -> raise("cannot reduce a branch to an integer")
    end
  end

  # TODO: something is still off here
  def infer(input, num, :humn) when is_integer(num), do: num

  def infer(input, num, {op, left, right}) when is_integer(left) and is_integer(right),
    do: infer(input, num, eval_op(op, left, right))

  def infer(input, num, {op, left, right}) when is_integer(left) do
    num = eval_op(op, left, num)
    infer(input, num, right)
  end

  def infer(input, num, {op, left, right}) when is_integer(right) do
    num = eval_op(@rev_op[op], num, right)
    infer(input, num, left)
  end

  def topological_order(input, stop_monkey) do
    {no_deps, graph} =
      Enum.reduce(input, {MapSet.new(), %{}}, fn
        {name, val}, {no_deps, graph} when is_number(val) ->
          {MapSet.put(no_deps, name), graph}

        {name, {_op, name1, name2}}, {no_deps, graph} ->
          {no_deps, Map.put(graph, name, [name1, name2])}
      end)

    topological_order(no_deps, graph, stop_monkey, [])
  end

  defp topological_order(no_deps, graph, stop_monkey, ordering) do
    ordering = ordering ++ MapSet.to_list(no_deps)

    if MapSet.size(no_deps) == 0 || stop_monkey in no_deps do
      ordering
    else
      {new_no_deps, graph} =
        Enum.reduce(graph, {MapSet.new(), %{}}, fn {name, dependencies}, {new_no_deps, graph} ->
          case Enum.filter(dependencies, fn dep -> dep not in no_deps end) do
            [] -> {MapSet.put(new_no_deps, name), graph}
            dependencies -> {new_no_deps, Map.put(graph, name, dependencies)}
          end
        end)

      topological_order(new_no_deps, graph, stop_monkey, ordering)
    end
  end

  def p1(input) do
    eval(input, "root") |> round()
  end

  def p2(input) do
    {_, monkey1, monkey2} = input["root"]
    input = Map.delete(input, "root")

    Enum.reduce_while(1..10_000, nil, fn humn, acc ->
      if rem(humn, 1000) == 0 do
        IO.inspect(humn, label: :iteration)
      end

      input = Map.put(input, "humn", humn)

      if eval(input, monkey1) == eval(input, monkey2) do
        {:halt, humn}
      else
        {:cont, acc}
      end
    end)
  end

  def contains?(num, _name) when is_integer(num), do: false
  def contains?({_op, name, _}, name) when is_atom(name) or is_binary(name), do: true
  def contains?({_op, _, name}, name) when is_atom(name) or is_binary(name), do: true
  def contains?({_op, left, right}, name), do: contains?(left, name) || contains?(right, name)
end

import ExUnit.Assertions

input = D21.parse_input("inputs/d21")
test_input = D21.parse_input("inputs/d21_test")

assert D21.p1(test_input) == 152
assert D21.p1(input) == 291_425_799_367_130

:timer.tc(fn -> D21.p1(test_input) end)
:timer.tc(fn -> D21.p1(input) end)

D21.build_relation(test_input)
{"=", left, right} = D21.build_relation(test_input)
assert D21.contains?(left, :humn) == true
assert D21.contains?(right, :humn) == false
assert D21.infer(test_input) == 301

{"=", left, right} = D21.build_relation(input)
assert D21.contains?(left, :humn) == true
assert D21.contains?(right, :humn) == false

D21.topological_order(test_input, "root")
test_input["root"]
D21.topological_order(test_input, "pppw")
D21.topological_order(test_input, "sjmn")

D21.topological_order(input, "root") |> length()
input["root"]
D21.topological_order(input, "pgtp") |> length()
D21.topological_order(input, "vrvh") |> length()
D21.topological_order(input, "humn") |> length()

D21.eval(input, "pgtp")
D21.eval(input, "vrvh")

D21.p2(test_input) == 301
# 30 seconds for 1000 iterations => way too slow
D21.p2(input)
