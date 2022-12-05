"""
Returns a 3-dimensional array of [pair, elf, min/max range boundary]
"""
function parse_input(inp::AbstractString)::Array{Int32,3}
    inp = split(inp, "\n")
    inp = split.(inp, ",")
    inp = hcat(inp...) |> permutedims
    inp = map(row -> parse.(Int32, row), split.(inp, "-"))

    X = zeros(Int32, size(inp)[1], 2, 2)
    for pair = axes(inp, 1),
        elf = axes(inp, 2),
        (idx, val) = enumerate(inp[pair, elf])

        X[pair, elf, idx] = val
    end
    X
end


function p1(X::Array{Int32,3})::Int32
    sum(
        # 1st elf's range fully contains the 2nd's
        ((X[:, 1, 1] .<= X[:, 2, 1]) .&& (X[:, 1, 2] .>= X[:, 2, 2])) .||
        # or the 2nd's fully contains the 1st
        ((X[:, 2, 1] .<= X[:, 1, 1]) .&& (X[:, 2, 2] .>= X[:, 1, 2]))
    )
end


function p2(X::Array{Int32,3})::Int32
    non_overlapping_count = sum(
        (X[:, 1, 2] .< X[:, 2, 1]) .||
        (X[:, 1, 1] .> X[:, 2, 2]) .||
        (X[:, 2, 2] .< X[:, 1, 1]) .||
        (X[:, 2, 1] .> X[:, 1, 2])
    )

    size(X, 1) - non_overlapping_count
end


inp = read("inputs/d04", String) |> chomp
inp = parse_input(inp)

test_inp =
    """
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    """ |> chomp
test_inp = parse_input(test_inp)

@assert p1(test_inp) == 2
@assert @show p1(inp) == 657

@assert p2(test_inp) == 4
@assert @show p2(inp) == 938
