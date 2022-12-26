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

  defp eval_once(input, env) do
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

  def p1(input) do
    eval(input, "root") |> round()
  end

  def p2(input) do
    {"=", left, right} = D21.build_relation(input)

    cond do
      is_integer(left) -> infer(input, left, right)
      is_integer(right) -> infer(input, right, left)
      true -> raise("cannot reduce a branch to an integer")
    end
  end

  @rev_op %{"+" => "-", "*" => "/", "-" => "+", "/" => "*"}

  @doc false
  def build_relation(input, env \\ %{}) do
    if Map.has_key?(env, "root") do
      env["root"]
    else
      env = build_relation_once(input, env)
      build_relation(input, env)
    end
  end

  # simplifies the expression by evaluating branches that don't contain :humn;
  # since both test and input have only one branch that contains :humn, the resulting
  # tree will have a form of `{"=", left, right}` where one of `left` or `rigth` is guaranteed to be an integer
  defp build_relation_once(input, env) do
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

  # progressively reverse operations keeping `num` as the current number to which the last argument should be equal
  defp infer(_input, num, :humn) when is_integer(num), do: num

  defp infer(input, num, {op, left, right}) when is_integer(left) do
    num =
      if op in ["-", "/"] do
        eval_op(op, left, num)
      else
        eval_op(@rev_op[op], num, left)
      end

    infer(input, num, right)
  end

  defp infer(input, num, {op, left, right}) when is_integer(right) do
    num = eval_op(@rev_op[op], num, right)
    infer(input, num, left)
  end
end

import ExUnit.Assertions

input = D21.parse_input("inputs/d21")
test_input = D21.parse_input("inputs/d21_test")

assert D21.p1(test_input) == 152
p1_ans = D21.p1(input)
assert p1_ans == 291_425_799_367_130
IO.puts("p1_ans = #{p1_ans}")

assert D21.p2(test_input) == 301
p2_ans = D21.p2(input)
assert p2_ans == 3_219_579_395_609
IO.puts("p2_ans = #{p2_ans}")
