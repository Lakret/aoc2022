using Pipe

function parse_input(path)
    @pipe (
        read(path, String)
        |> chomp
        |> split(_, "\n")
        |> collect.(_)
        |> mapreduce(permutedims, vcat, _)
        |> findall(ch -> ch == '#', _)
        |> Set(_)
    )
end

# moves[n, :] is the nth condition
# moves[n, 1] is the actual move
moves::Matrix{CartesianIndex{2}} = [
    CartesianIndex(-1, 0) CartesianIndex(-1, -1) CartesianIndex(-1, 1)
    CartesianIndex(1, 0) CartesianIndex(1, -1) CartesianIndex(1, 1)
    CartesianIndex(0, -1) CartesianIndex(-1, -1) CartesianIndex(1, -1)
    CartesianIndex(0, 1) CartesianIndex(-1, 1) CartesianIndex(1, 1)
]

neighbours::Vector{CartesianIndex{2}} = collect(Set(moves))

function round(elfs::Set{CartesianIndex{2}}; start_move_idx::Int=1)::Set{CartesianIndex{2}}
    # first part of the round: all elfs with neighbours propose moves if possible
    proposals = Dict()
    destination_claims = Dict()

    for elf = elfs
        all_neighbours = Set(map(x -> elf + x, neighbours))
        if !isempty(intersect(all_neighbours, elfs))
            considred_moves = 0

            while considred_moves < 4
                curr_move_idx = mod1(start_move_idx + considred_moves, 4)
                neighbours_in_direction = Set(map(x -> elf + x, moves[curr_move_idx, :]))
                if isempty(intersect(neighbours_in_direction, elfs))
                    destination = elf + moves[curr_move_idx, 1]
                    proposals[elf] = destination
                    destination_claims[destination] = get(destination_claims, destination, 0) + 1
                    break
                end

                considred_moves += 1
            end
        end
    end

    # second part of the round: elfs with non-conflicting proposals move
    new_elfs = deepcopy(elfs)
    invalid_destinations = Set(keys(filter(x -> x[2] > 1, destination_claims)))
    valid_proposals = filter(proposal -> !(proposal[2] in invalid_destinations), proposals)
    for proposal = valid_proposals
        delete!(new_elfs, proposal[1])
        push!(new_elfs, proposal[2])
    end

    new_elfs
end

function rounds(elfs::Set{CartesianIndex{2}}, n::Int)
    curr_elfs = deepcopy(elfs)
    for start_move_idx = 1:n
        curr_elfs = round(curr_elfs, start_move_idx=start_move_idx)
    end
    curr_elfs
end

function visualize(elfs::Set{CartesianIndex{2}})
    for row = eachrow(collect(minimum(elfs):maximum(elfs)))
        for elf = row
            if elf ∈ elfs
                print("#")
            else
                print(".")
            end
        end
        println()
    end
end

function p1(elfs::Set{CartesianIndex{2}})::Int
    new_elfs = rounds(elfs, 10)
    (top_left, bottom_right) = minimum(new_elfs), maximum(new_elfs)
    prod(bottom_right.I .- top_left.I .+ (1, 1)) - length(new_elfs)
end

function p2(elfs::Set{CartesianIndex{2}})::Int
    curr_elfs = deepcopy(elfs)
    new_elfs = Set()
    curr_start_move_idx = 1

    while true
        new_elfs = round(curr_elfs, start_move_idx=curr_start_move_idx)

        if new_elfs == curr_elfs
            return curr_start_move_idx
        end

        curr_start_move_idx += 1
        curr_elfs = new_elfs
    end
end

test_elfs = parse_input("inputs/d23_test")
elfs = parse_input("inputs/d23")
elfs = test_elfs

@pipe test_elfs |> rounds(_, 10) |> visualize
@time @assert p1(test_elfs) == 110
@time @assert p1(elfs) == 4109
@time @assert p2(test_elfs) == 20
@time @assert p2(elfs) == 1055
