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
Returns (
    previous_nodes,
    final_distances,
    dest_char_coords,
    dest_char_cost,
    best_signal_coords,
    best_signal_so_far,
    best_signal_distance
).
"""
function dijkstra(grid::Matrix{Char}; start_char::Char='S', dest_char::Char='E', inverted::Bool=false)::Tuple{
    Dict{CartesianIndex,CartesianIndex},
    Dict{CartesianIndex,Int},
    Union{CartesianIndex,Missing},
    Union{Int,Missing}
}
    unvisited = Set([CartesianIndices(grid)...])
    start_idx = findall(x -> x == start_char, grid) |> first

    distances = @pipe collect(unvisited) |> map(x -> x => Inf, _)
    distances = PriorityQueue(distances)
    distances[start_idx] = 0

    final_distances = Dict(start_idx => 0)
    previous_nodes = Dict()

    while !isempty(unvisited)
        if isempty(distances)
            println("used it")
            distances = PriorityQueue([k => v for (k, v) = collect(final_distances) if (k in unvisited) && v != Inf])
        end

        (curr_idx, cost) = dequeue_pair!(distances)
        while !(curr_idx in unvisited)
            distances[curr_idx] = cost
            (curr_idx, cost) = dequeue_pair!(distances)
        end
        @assert curr_idx in unvisited
        @assert cost != Inf

        for neighbour_idx = connected_neighbours(grid, curr_idx, inverted=inverted)
            if neighbour_idx in unvisited
                if distances[neighbour_idx] > (cost + 1)
                    distances[neighbour_idx] = cost + 1
                    previous_nodes[neighbour_idx] = curr_idx
                    final_distances[neighbour_idx] = cost + 1

                    if grid[neighbour_idx] == dest_char
                        return previous_nodes, final_distances, neighbour_idx, cost + 1
                    end
                end
            end
        end

        delete!(unvisited, curr_idx)
    end

    previous_nodes, final_distances, missing, missing
end


function visualize_paths(grid::Matrix{Char}, previous_nodes::Dict{CartesianIndex,CartesianIndex})
    for row_id = 1:size(grid)[1]
        for col_id = 1:size(grid)[2]
            idx = CartesianIndex(row_id, col_id)
            if haskey(previous_nodes, idx)
                print(grid[idx])
            else
                print(".")
            end
        end
        println()
    end
end

test_grid = parse_input("inputs/d12_test")
grid = parse_input("inputs/d12")

previous_nodes, final_distances, dest_coords, dest_cost = dijkstra(test_grid)
@assert dest_coords == CartesianIndex(3, 6)
@assert dest_cost == 31
@assert test_grid[dest_coords] == 'E'

previous_nodes, final_distances, dest_coords, dest_cost = dijkstra(grid)
@assert dest_coords == CartesianIndex(21, 59)
@assert dest_cost == 408
@assert grid[dest_coords] == 'E'

previous_nodes, final_distances, dest_coords, dest_cost = dijkstra(
    test_grid, start_char='E', dest_char='a', inverted=true
)
@assert dest_cost == 29
@assert dest_coords == CartesianIndex(5, 1)
@assert grid[dest_coords] == 'a'

previous_nodes, final_distances, dest_coords, dest_cost = dijkstra(grid, start_char='E', dest_char='a', inverted=true)
@assert dest_cost == 399
@assert dest_coords == CartesianIndex(34, 1)
@assert grid[dest_coords] == 'a'
