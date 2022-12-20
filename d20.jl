using Pipe

mutable struct CircularDoubleLinkedList
    nodes::Vector{Int64}
    # maps virtual position in the linked list for each node with the next node
    # i.e., if next[id1] == id2, and next[id2] == id3, it means that id1 -> id2 -> id3
    next::Vector{Int64}
    # same as next, but points to the previous node
    prev::Vector{Int64}
    first_idx::Int64

    function CircularDoubleLinkedList(values::Vector{Int64})
        nodes = deepcopy(values)
        next = [2:length(nodes); 1]
        prev = [length(nodes); 1:(length(nodes)-1)]
        first_idx = 1

        new(nodes, next, prev, first_idx)
    end
end

function parse_input(path)::CircularDoubleLinkedList
    values = @pipe read(path, String) |> chomp |> split .|> parse(Int64, _)
    CircularDoubleLinkedList(values)
end

function walk_once(list::CircularDoubleLinkedList, start_pos::Int64)::Vector{Int64}
    res = [list.nodes[start_pos]]

    pos = list.next[start_pos]
    while pos != start_pos
        push!(res, list.nodes[pos])
        pos = list.next[pos]
    end

    res
end

function Base.show(io::IO, list::CircularDoubleLinkedList)
    curr_order = walk_once(list, list.first_idx)
    print(
        io,
        "CircularDoubleLinkedList(first_pos=$(list.first_idx), next=$(list.next), prev=$(list.prev), nodes=$(list.nodes))):\n\t$curr_order"
    )
end

function move(list::CircularDoubleLinkedList, pos::Int64)
    if list.nodes[pos] == 0
        return
    end

    delta = list.nodes[pos] # % length(list.nodes)
    list.first_idx += delta

    if delta > 0
        # ... -> [curr_prev] -> (curr) -> [curr_next] -> [curr_next_next] -> ...
        # after move of (curr) forward by delta==1 becomes
        # ... -> [curr_prev] -> [curr_next] -> (curr) -> [curr_next_next] -> ...
        for _ in 1:delta
            curr_prev = list.prev[pos]
            curr_next = list.next[pos]
            curr_next_next = list.next[curr_next]

            list.next[pos] = curr_next_next
            list.prev[pos] = curr_next
            list.prev[curr_next_next] = pos
            list.next[curr_next] = pos
            list.prev[curr_next] = curr_prev
            list.next[curr_prev] = curr_next
        end
    else
        # negative values move in the opposite direction, so after 1 step this:
        # ... -> [curr_prev_prev] -> [curr_prev] -> (curr) -> [curr_next] -> ...
        # becomes
        # ... -> [curr_prev_prev] -> (curr) -> [curr_prev] -> [curr_next] -> ...
        for _ in 1:abs(delta)
            curr_next = list.next[pos]
            curr_prev = list.prev[pos]
            curr_prev_prev = list.prev[curr_prev]

            list.next[pos] = curr_prev
            list.prev[pos] = curr_prev_prev
            list.prev[curr_prev] = pos
            list.next[curr_prev_prev] = pos
            list.next[curr_prev] = curr_next
            list.prev[curr_next] = curr_prev
        end
    end
end

function next_by_n(list::CircularDoubleLinkedList, pos::Int64, n::Int64)::Int64
    pos = pos
    for _ in 1:n #(n%length(list.nodes))
        pos = list.next[pos]
    end
    list.nodes[pos]
end

function p1(list::CircularDoubleLinkedList)
    list = deepcopy(list)

    for pos = 1:length(list.nodes)
        move(list, pos)
    end

    @show zero_pos = findfirst(x -> x == 0, list.nodes)
    @show n_1000 = next_by_n(list, zero_pos, 1000)
    @show n_2000 = next_by_n(list, zero_pos, 2000)
    @show n_3000 = next_by_n(list, zero_pos, 3000)
    n_1000 + n_2000 + n_3000
end


input = parse_input("inputs/d20")
test_input = parse_input("inputs/d20_test")

# 13447 is too high
# 13343 is too high
@time @assert @show p1(test_input) == 3
@time p1(input)
