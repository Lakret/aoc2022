using Pipe

function parse_input(path)
    @pipe read(path, String) |> chomp |> split(_, "\n")
end

to_digits::Dict{Char,Int64} = Dict(
    '2' => 2,
    '1' => 1,
    '0' => 0,
    '-' => -1,
    '=' => -2,
)

to_snafu::Dict{Int64,Char} = begin
    @pipe to_digits |> collect |> map(kv -> reverse(kv), _) |> Dict
end

function snafu_to_num(snafu::AbstractString)
    res = 0
    for (power, ch) = (snafu |> reverse |> enumerate)
        res += to_digits[ch] * (5^(power - 1))
    end
    res
end

function num_to_snafu(num::Int64)::String
    res::Vector{Char} = []
    remaining = num
    while remaining != 0
        (remaining, r) = divrem(remaining, 5)
        if r >= 3
            remaining += 1
            r -= 5
        end
        push!(res, to_snafu[r])
    end
    String(res |> reverse)
end

p1(input)::String = input .|> snafu_to_num |> sum |> num_to_snafu

test_input = parse_input("inputs/d25_test")
input = parse_input("inputs/d25")

@assert test_input[1] |> snafu_to_num == 1747
@assert test_input[2] |> snafu_to_num == 906
@assert test_input[end] |> snafu_to_num == 37
@assert num_to_snafu(1747) == test_input[1]
@assert num_to_snafu(906) == test_input[2]
@assert num_to_snafu(37) == test_input[end]
@assert p1(test_input) == "2=-1=0"
@time @assert @show p1(input) == "2=001=-2=--0212-22-2"
