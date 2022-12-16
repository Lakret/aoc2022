using Pipe

struct Valve
    flow::Int64
    connections::Vector{AbstractString}
end

Graph = Dict{AbstractString,Valve}

function parse_input(path)::Graph
    graph = Dict()
    re = r"^Valve (?<id>\w+) has flow rate=(?<flow>\d+); tunnels? leads? to valves? (?<connections>(\w+(?:, )?)+)$"
    matches = @pipe read(path, String) |> chomp |> split(_, "\n") |> match.(re, _)

    for m = matches
        graph[m["id"]] = Valve(parse(Int64, m["flow"]), split(m["connections"], ", "))
    end
    graph
end

test_graph = parse_input("inputs/d16_test")
graph = parse_input("inputs/d16")


#     for each edge (u, v) do
#         dist[u][v] ← w(u, v)  // The weight of the edge (u, v)
#     for k from 1 to |V|
#         for i from 1 to |V|
#             for j from 1 to |V|
#                 if dist[i][j] > dist[i][k] + dist[k][j]
#                     dist[i][j] ← dist[i][k] + dist[k][j]
#                 end if

# graph = test_graph
function fw(graph::Graph)
    idx_to_vertex_id = keys(graph) |> enumerate |> Dict
    vertex_id_to_idx = collect(idx_to_vertex_id) .|> reverse |> Dict
    num_vertices = length(idx_to_vertex_id)
    dist = fill(Inf, (num_vertices, num_vertices))
    paths = fill(0, (num_vertices, num_vertices))

    for (vertex_id, valve) = graph
        idx = vertex_id_to_idx[vertex_id]

        # always open non-zero valves
        # if valve.flow == 0
        dist[idx, idx] = 0
        paths[idx, idx] = idx
        # else
        #     dist[idx, idx] = 1
        # end

        for connection_vertex_id = valve.connections
            connection_vertex_idx = vertex_id_to_idx[connection_vertex_id]
            dist[idx, connection_vertex_idx] = -valve.flow
            paths[idx, connection_vertex_idx] = connection_vertex_idx
        end
    end

    for k = 1:num_vertices
        for i = 1:num_vertices
            for j = 1:num_vertices
                if dist[i, j] > dist[i, k] + dist[k, j]
                    dist[i, j] = dist[i, k] + dist[k, j]
                    paths[i, j] = paths[i, k]
                end
            end
        end
    end

    dist, paths, idx_to_vertex_id
end

function reconstruct_path(paths::Matrix{Any}, idx_to_vertex_id::Dict{Any,Any}, from_idx)::Vector[Any]
    path = []
    id = idx_to_vertex_id[from_idx]
    while !(id in path)
        push!(path, id)
        next_idx = paths[from_idx]
        id = idx_to_vertex_id[next_idx]
    end
    path
end