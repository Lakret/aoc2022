using Pipe

struct Valve
    flow::Int64
    connections::Vector{AbstractString}
end

Graph = Dict{AbstractString,Valve}
Path = Vector{AbstractString}

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
# graph = parse_input("inputs/d16")
graph = test_graph


# julia> @time scores = p1(graph, "AA"; max_minutes=20)
#   3.042289 seconds (10.30 M allocations: 618.137 MiB)

function p1(graph::Graph, start_id::AbstractString; max_minutes=6)
    to_explore = [(curr=start_id, minute=0, opened=[], total_flow=0, released_so_far=0)]
    scores = Set()
    # TODO: non-zero valves count
    max_opened = 6

    while !isempty(to_explore)
        curr, minute, opened, total_flow, released_so_far = popfirst!(to_explore)

        # if all valves were opened, skip to the end
        if length(opened) == max_opened
            released_so_far += (max_minutes - minute) * total_flow
            push!(scores, released_so_far)
            continue
        end

        if minute < max_minutes
            flow = graph[curr].flow
            if flow > 0 && curr ∉ opened
                minute += 1
                released_so_far += total_flow
                opened = [curr; opened]

                minute += 1
                total_flow += flow
                released_so_far += total_flow
            else
                minute += 1
                released_so_far += total_flow
            end

            for neighbour = graph[curr].connections
                push!(
                    to_explore,
                    (curr=neighbour, minute=minute, opened=opened, total_flow=total_flow,
                        released_so_far=released_so_far)
                )
            end
        else
            # avoid double counting because of the eager minute addition when opening a valve
            if minute > max_minutes
                released_so_far -= total_flow
            end
            push!(scores, released_so_far)
        end
    end

    scores |> maximum
end

# graph = test_graph
# paths = p1(graph, "AA")
# 6 - 206 paths
# 7 - 471 paths
# @time paths = bfs(graph, "AA", max_steps=8)
# 1111
# @time paths = bfs(graph, "AA", max_steps=9)
# 2537-element
# 10 - 5970

# all_paths = Set()
# for valve_id = keys(graph)
#     union!(all_paths, bfs(graph, valve_id))
# end


# function score_path(path, graph)
#     minute, total_flow, released_pressure = 0, 0, 0

#     for valve_id = path
#         flow = graph[valve_id].flow
#         if flow > 0
#             @show minute += 1
#             @show released_pressure += total_flow

#             @show minute += 1
#             @show total_flow += flow
#             @show released_pressure += total_flow
#         else
#             @show minute += 1
#             @show released_pressure += total_flow
#         end
#     end

#     released_pressure += (30 - minute) * total_flow
#     return (released_pressure=released_pressure, total_flow=total_flow)
# end

# #  map(path -> score_path(path, graph), paths)
