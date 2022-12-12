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
function connected_neighbours(grid::Matrix{Char}, idx::CartesianIndex)::Vector{CartesianIndex}
    curr_char = normalize_char(grid[idx])
    neighbours = []

    for x = [-1, 1]
        new_x = idx[1] + x
        if new_x >= 1 && new_x <= size(grid)[1]
            neighbour_idx = CartesianIndex(new_x, idx[2])
            if normalize_char(grid[neighbour_idx]) - curr_char <= 1
                push!(neighbours, neighbour_idx)
            end
        end
    end

    for y = [-1, 1]
        new_y = idx[2] + y
        if new_y >= 1 && new_y <= size(grid)[2]
            neighbour_idx = CartesianIndex(idx[1], new_y)
            if normalize_char(grid[neighbour_idx]) - curr_char <= 1
                push!(neighbours, neighbour_idx)
            end
        end
    end

    neighbours
end

function dijkstra(grid::Matrix{Char})::Tuple{
    Dict{CartesianIndex,CartesianIndex},
    Dict{CartesianIndex,Int},
    Union{CartesianIndex,Missing},
    Union{Int,Missing},
    CartesianIndex,
    Char,
    Int
}
    unvisited = Set([CartesianIndices(grid)...])
    start_idx = findall(x -> x == 'S', grid) |> first

    distances = @pipe collect(unvisited) |> map(x -> x => Inf, _)
    distances = PriorityQueue(distances)
    distances[start_idx] = 0

    best_signal_coords, best_signal_so_far, best_signal_distance = start_idx, 'a', 0

    final_distances = Dict(start_idx => 0)
    previous_nodes = Dict()

    while !isempty(unvisited)
        if isempty(distances)
            distances = PriorityQueue([k => v for (k, v) = collect(final_distances) if (k in unvisited) && v != Inf])
        end

        (curr_idx, cost) = dequeue_pair!(distances)
        while !(curr_idx in unvisited)
            distances[curr_idx] = cost
            (curr_idx, cost) = dequeue_pair!(distances)
        end
        @assert curr_idx in unvisited

        if cost == Inf
            return previous_nodes, final_distances, missing, missing, best_signal_coords, best_signal_so_far, best_signal_distance
        end

        for neighbour_idx = connected_neighbours(grid, curr_idx)
            if neighbour_idx in unvisited
                if distances[neighbour_idx] > (cost + 1)
                    distances[neighbour_idx] = cost + 1
                    previous_nodes[neighbour_idx] = curr_idx
                    final_distances[neighbour_idx] = cost + 1

                    if best_signal_so_far < normalize_char(grid[neighbour_idx])
                        best_signal_coords = neighbour_idx
                        best_signal_so_far = grid[neighbour_idx]
                        best_signal_distance = cost + 1
                    end

                    if grid[neighbour_idx] == 'E'
                        return previous_nodes, final_distances, neighbour_idx, cost + 1, neighbour_idx, 'E', cost + 1
                    end
                end
            end
        end

        delete!(unvisited, curr_idx)
    end

    previous_nodes, final_distances, missing, missing, best_signal_coords, best_signal_so_far, best_signal_distance
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

# excluding the start node
function reconstruct_path(
    previous_nodes::Dict{CartesianIndex,CartesianIndex}, to::CartesianIndex
)::Vector{CartesianIndex}
    curr = to
    path = []
    while in
        (previous_nodes, curr)
        push!(path, curr)
        curr = previous_nodes[curr]
    end
    reverse(path)
end

function visualize_path(grid::Matrix{Char}, path::Vector{CartesianIndex})
    for row_id = 1:size(grid)[1]
        for col_id = 1:size(grid)[2]
            idx = CartesianIndex(row_id, col_id)
            if idx in path
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

previous_nodes, final_distances, e_coords, e_cost, best_signal_coords, best_signal_so_far, best_signal_distance =
    dijkstra(test_grid)
@assert e_coords == CartesianIndex(3, 6)
@assert e_cost == 31
@assert best_signal_coords == e_coords
@assert best_signal_so_far == 'E'
@assert best_signal_distance == e_cost

previous_nodes, final_distances, e_coords, e_cost, best_signal_coords, best_signal_so_far, best_signal_distance =
    dijkstra(grid)
@assert e_coords == CartesianIndex(21, 59)
@assert e_cost == 408
@assert best_signal_coords == e_coords
@assert best_signal_so_far == 'E'
@assert best_signal_distance == e_cost

# visualize_paths(grid, previous_nodes)
# visualize_paths(grid, previous_nodes)
# e_coords = CartesianIndex(21, 59)

# 298 is too low
# 299 is too low
# 312 is too low