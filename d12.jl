using Pipe
using DataStructures

parse_input(path)::Matrix{Char} =
    @pipe read(path, String) |> chomp |> split .|> collect |> mapreduce(permutedims, vcat, _)

function normalize_char(char::Char)::Char
    if char == 'S'
        'a'
    elseif char == 'E'
        'z'
    else
        char
    end
end

# note: we should allow going to at max 1 higher elevations, BUT we may drop from any elevation!
function connected_neighbours(grid::Matrix{Char}, idx::CartesianIndex; inverted::Bool=false)::Vector{CartesianIndex}
    curr_char = normalize_char(grid[idx])
    neighbours = []

    for x = [-1, 1]
        new_x = idx[1] + x
        if new_x >= 1 && new_x <= size(grid)[1]
            neighbour_idx = CartesianIndex(new_x, idx[2])
            neighbour_char = normalize_char(grid[neighbour_idx])
            if (!inverted && (neighbour_char - curr_char <= 1)) || (inverted && (curr_char - neighbour_char <= 1))
                push!(neighbours, neighbour_idx)
            end
        end
    end

    for y = [-1, 1]
        new_y = idx[2] + y
        if new_y >= 1 && new_y <= size(grid)[2]
            neighbour_idx = CartesianIndex(idx[1], new_y)
            neighbour_char = normalize_char(grid[neighbour_idx])
            if (!inverted && (neighbour_char - curr_char <= 1)) || (inverted && (curr_char - neighbour_char <= 1))
                push!(neighbours, neighbour_idx)
            end
        end
    end

    neighbours
end

"""
Returns a tuple of:

    - `previous_nodes` - a dictionary mapping each reachable vertex's coords
    to it's predecessor's coords on the shortest path from `start_char`.
    - `dest_char_pos` - the coords of the `dest_char` or `missing` if it's not reachable
    - `dest_char_distance` - the distance to the `dest_char` from `start_char`
"""
function dijkstra(grid::Matrix{Char}; start_char::Char='S', dest_char::Char='E', inverted::Bool=false)::Tuple{
    Dict{CartesianIndex,CartesianIndex},
    Union{CartesianIndex,Missing},
    Union{Int,Missing}
}
    unvisited = Set([CartesianIndices(grid)...])
    start_idx = findall(x -> x == start_char, grid) |> first

    distances = @pipe collect(unvisited) |> map(x -> x => Inf, _)
    distances = PriorityQueue(distances)
    distances[start_idx] = 0

    previous_nodes = Dict()

    while !isempty(unvisited)
        (curr_idx, cost) = dequeue_pair!(distances)
        @assert curr_idx in unvisited
        @assert cost != Inf

        for neighbour_idx = connected_neighbours(grid, curr_idx, inverted=inverted)
            if neighbour_idx in unvisited
                if distances[neighbour_idx] > (cost + 1)
                    distances[neighbour_idx] = cost + 1
                    previous_nodes[neighbour_idx] = curr_idx

                    if grid[neighbour_idx] == dest_char
                        return previous_nodes, neighbour_idx, cost + 1
                    end
                end
            end
        end

        delete!(unvisited, curr_idx)
    end

    previous_nodes, missing, missing
end

function visualize_paths(
    grid::Matrix{Char},
    previous_nodes::Dict{CartesianIndex,CartesianIndex},
    start_char::Char
)
    start_coords = findall(x -> x == start_char, grid) |> first

    for row_id = 1:size(grid)[1]
        for col_id = 1:size(grid)[2]
            idx = CartesianIndex(row_id, col_id)
            if haskey(previous_nodes, idx)
                print(grid[idx])
            elseif idx == start_coords
                print("🏃")
            else
                print(".")
            end
        end
        println()
    end
end

test_grid = parse_input("inputs/d12_test")
grid = parse_input("inputs/d12")

previous_nodes, dest_coords, dest_cost = dijkstra(test_grid)
@assert dest_coords == CartesianIndex(3, 6)
@assert dest_cost == 31
@assert test_grid[dest_coords] == 'E'

previous_nodes, dest_coords, dest_cost = dijkstra(grid)
@assert dest_coords == CartesianIndex(21, 59)
@assert dest_cost == 408
@assert grid[dest_coords] == 'E'

println("P1 ans: $dest_cost.")
visualize_paths(grid, previous_nodes, 'S')
println()

previous_nodes, dest_coords, dest_cost = dijkstra(test_grid, start_char='E', dest_char='a', inverted=true)
@assert dest_cost == 29
@assert dest_coords == CartesianIndex(5, 1)
@assert grid[dest_coords] == 'a'

previous_nodes, dest_coords, dest_cost = dijkstra(grid, start_char='E', dest_char='a', inverted=true)
@assert dest_cost == 399
@assert dest_coords == CartesianIndex(34, 1)
@assert grid[dest_coords] == 'a'

println("P2 ans: $dest_cost.")
visualize_paths(grid, previous_nodes, 'E')
println()
