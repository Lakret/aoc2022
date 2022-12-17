using Pipe

# Part 1 idea:
# 1. preproccess the graph with FW to find shortest distances between all nodes
# 2. all candidate for optimality paths can be expressed as a permutation of non-zero flow valves -
# we can generate them using breadth-first search modification
# 3. for each permutation, compute the score using the shortest distances to speed up computation for no change
# moments
# + use vectors and integer vertex ids

struct Valve{KeyId}
    flow::Int64
    connections::Vector{<:KeyId}
end

DictGraph{KeyId} = Dict{KeyId,Valve{<:KeyId}}
Path{KeyId} = Vector{<:KeyId}

function parse_input(path)::DictGraph{AbstractString}
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

VecGraph = Vector{Valve{<:Int}}

function index_graph(graph::DictGraph{AbstractString})::Tuple{VecGraph,Dict{AbstractString,Int}}
    id_to_idx = keys(graph) |> collect
    sort!(id_to_idx)
    @show id_to_idx = id_to_idx |> enumerate .|> reverse |> Dict
    vec_graph = Vector{Valve}(undef, length(id_to_idx))

    for (id, valve) = graph
        idx = id_to_idx[id]
        connections = map(id -> id_to_idx[id], valve.connections)
        vec_graph[idx] = Valve(valve.flow, connections)
    end

    vec_graph, id_to_idx
end

test_graph, test_graph_id_to_idx = index_graph(test_graph)
graph, graph_id_to_idx = index_graph(graph)

function fw(graph::VecGraph)::Tuple{Matrix{Int},Matrix{Int}}
    n_vertices = length(graph)
    dist = fill(Inf, (n_vertices, n_vertices))
    paths = fill(0, (n_vertices, n_vertices))

    for (vertex_id, valve) = enumerate(graph)
        dist[vertex_id, vertex_id] = 0
        paths[vertex_id, vertex_id] = vertex_id

        for next_vertex_id = valve.connections
            dist[vertex_id, next_vertex_id] = 1
            paths[vertex_id, next_vertex_id] = next_vertex_id
        end
    end

    for k = 1:n_vertices
        for i = 1:n_vertices
            for j = 1:n_vertices
                if dist[i, j] > dist[i, k] + dist[k, j]
                    dist[i, j] = dist[i, k] + dist[k, j]
                    paths[i, j] = paths[i, k]
                end
            end
        end
    end

    convert.(Int64, dist), paths
end

test_graph_dist, test_graph_paths = fw(test_graph)
graph_dist, graph_paths = fw(graph)

function get_non_zero_flow_valves(graph::VecGraph)::Vector{Int}
    @pipe enumerate(graph) |> collect |> filter(id_and_v -> id_and_v[2].flow > 0, _) |> first.(_)
end

test_graph_non_zero_flow_valves = get_non_zero_flow_valves(test_graph)
graph_non_zero_flow_valves = get_non_zero_flow_valves(graph)

mutable struct State
    current_id::Int
    to_open::Vector{Int}

    minute::Int
    flow::Int
    released_pressure::Int
end

"""
Opens the current valve `state.current_id`,
updating the state up to the end of the open valve minute.
Remove the opened valve from the set of valves yet to open.
"""
function open_valve!(state::State, graph::VecGraph)
    state.minute += 1
    state.released_pressure += state.flow
    state.flow += graph[state.current_id].flow

    deleteat!(state.to_open, findall(id -> id == state.current_id, state.to_open))
    return
end

"""
Moves from `state.current_id` valve to `dest_id` valve,
updating the state up to the end of the arrival minute.

Uses `graph_dist` from Floyd-Warshall to avoid recomputing / searching for the path
and to apply changes to `released_pressure`` and `minute`` in one step.
"""
function move_to!(state::State, graph_dist::Matrix{Int}, dest_id::Int)
    steps = graph_dist[state.current_id, dest_id]
    state.minute += steps
    state.released_pressure += steps * state.flow

    state.current_id = dest_id
end

get_score(state::State, max_minutes::Int) = state.released_pressure + (max_minutes - state.minute) * state.flow

function p1(graph::VecGraph; start_id::Int=1, max_minutes=30)
    graph_dist, _graph_paths = fw(graph)
    non_zero_flow_valves = get_non_zero_flow_valves(graph)

    scores = Set()
    queue = [State(start_id, deepcopy(non_zero_flow_valves), 0, 0, 0)]

    while !isempty(queue)
        state = popfirst!(queue)

        if state.minute >= max_minutes
            # time expired
            push!(scores, state.released_pressure + (max_minutes - state.minute) * state.flow)
        elseif isempty(state.to_open)
            # no more valves to open => compute the rest of the time and save the score
            push!(scores, get_score(state, max_minutes))
        else
            # otherwise, add all reachable next open valve candidates
            remaining_time = max_minutes - state.minute

            more_valves_will_be_opened = false
            for next_id = state.to_open
                # doesn't make sense to run to something we cannot reach
                if graph_dist[state.current_id, next_id] < remaining_time
                    new_state = deepcopy(state)
                    more_valves_will_be_opened = true

                    move_to!(new_state, graph_dist, next_id)
                    open_valve!(new_state, graph)

                    push!(queue, new_state)
                end
            end

            # if no more valves can be opened, we calculate the score
            if !more_valves_will_be_opened
                push!(scores, get_score(state, max_minutes))
            end
        end
    end

    scores |> maximum
end

@assert p1(test_graph, max_minutes=3) == 20
@time @assert @show p1(test_graph) == 1651
@time @assert @show p1(graph) == 2056


mutable struct State2
    current_ids::Vector{Int}
    to_open::Vector{Int}

    minutes::Vector{Int}
    flows::Vector{Int}
    released_pressure::Vector{Int}
end

function open_valve!(state::State2, graph::VecGraph, actor::Int)
    vertex_id = state.current_ids[actor]

    state.minutes[actor] += 1
    state.released_pressure[actor] += state.flows[actor]
    state.flows[actor] += graph[vertex_id].flow

    deleteat!(state.to_open, findall(id -> id == vertex_id, state.to_open))
    return
end

function move_to!(state::State2, graph_dist::Matrix{Int}, dest_id::Int, actor::Int)
    steps = graph_dist[state.current_ids[actor], dest_id]
    state.minutes[actor] += steps
    state.released_pressure[actor] += steps * state.flows[actor]

    state.current_ids[actor] = dest_id
end

function get_score(state::State2, max_minutes::Int)
    sum(state.released_pressure .+ (max_minutes .- state.minutes) .* state.flows)
end

function p2(graph::VecGraph; start_id::Int=1, max_minutes=26)
    graph_dist, _graph_paths = fw(graph)
    non_zero_flow_valves = get_non_zero_flow_valves(graph)

    scores = Set()
    queue = [State2([start_id, start_id], deepcopy(non_zero_flow_valves), [0, 0], [0, 0], [0, 0])]

    while !isempty(queue)
        state = popfirst!(queue)

        if all(state.minutes .>= max_minutes)
            push!(scores, get_score(state, max_minutes))
        elseif isempty(state.to_open)
            # no more valves to open => compute the rest of the time and save the score
            push!(scores, get_score(state, max_minutes))
        else
            # otherwise, add all reachable next open valve candidates for each actor
            more_valves_will_be_opened = false

            # you
            remaining_time = max_minutes - state.minutes[1]
            for next_id = state.to_open
                # doesn't make sense to run to something we cannot reach
                if graph_dist[state.current_ids[1], next_id] < remaining_time
                    new_state = deepcopy(state)
                    more_valves_will_be_opened = true

                    move_to!(new_state, graph_dist, next_id, 1)
                    open_valve!(new_state, graph, 1)

                    # elephant
                    remaining_time = max_minutes - state.minutes[2]
                    for next_id = new_state.to_open
                        if graph_dist[state.current_ids[2], next_id] < remaining_time
                            new_new_state = deepcopy(new_state)

                            move_to!(new_new_state, graph_dist, next_id, 2)
                            open_valve!(new_new_state, graph, 2)

                            push!(queue, new_new_state)
                        end
                    end
                end
            end

            # if no more valves can be opened, we calculate the score
            if !more_valves_will_be_opened
                push!(scores, get_score(state, max_minutes))
            end
        end
    end

    scores |> maximum
end

@time @assert @show @time p2(test_graph, max_minutes=26) == 1707
# @time @assert @show p2(graph, max_minutes=26) == 2056

# julia> @time p2(test_graph, max_minutes=26) == 1707
#   0.001917 seconds (25.95 k allocations: 1.969 MiB)

# julia> @time p2(test_graph, max_minutes=26)
#   0.008686 seconds (27.06 k allocations: 2.054 MiB)


# julia> @time p2(graph, max_minutes=20)
#  12.970624 seconds (72.10 M allocations: 5.537 GiB)
# julia> @time p2(graph, max_minutes=20)
#  10.247361 seconds (72.10 M allocations: 5.537 GiB, 25.32% gc time)


# julia> @time p2(graph, max_minutes=20)
#  10.026532 seconds (68.51 M allocations: 5.269 GiB, 38.86% gc time)
# 1460

# julia> @time p2(graph, max_minutes=20)
#   8.816874 seconds (68.51 M allocations: 5.269 GiB, 37.07% gc time)

# julia> @time p2(graph, max_minutes=20)
#   7.926491 seconds (68.51 M allocations: 5.269 GiB, 35.35% gc time)

# julia> @time p2(graph, max_minutes=20)
#   0.998779 seconds (8.94 M allocations: 687.624 MiB, 21.92% gc time)


# julia> @time p2(graph, max_minutes=26)
# 350.881363 seconds (1.33 G allocations: 89.751 GiB, 44.14% gc time)

# 2502 is too low for part2

# TODO: mirror positions optimization failed, so it seems we need to do some "reduce allocations" fun