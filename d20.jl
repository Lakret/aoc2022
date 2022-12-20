using Pipe

mutable struct CircularDoubleLinkedList
    # actual values in the list; maps "virtual position" to the corresponding value
    # that virtual position (i.e., the original position in the list) works like an id of the node.
    nodes::Vector{Int64}
    # maps virtual position for each node with the next node's virtual position/id
    # i.e., if next[id1] == id2, and next[id2] == id3, it means that id1 -> id2 -> id3
    next::Vector{Int64}
    # same as next, but points to the previous node instead
    prev::Vector{Int64}

    function CircularDoubleLinkedList(values::Vector{Int64})
        nodes = deepcopy(values)
        next = [2:length(nodes); 1]
        prev = [length(nodes); 1:(length(nodes)-1)]

        new(nodes, next, prev)
    end
end


function parse_input(path)::CircularDoubleLinkedList
    values = @pipe read(path, String) |> chomp |> split .|> parse(Int64, _)
    CircularDoubleLinkedList(values)
end


function move(list::CircularDoubleLinkedList, pos::Int64)
    if list.nodes[pos] == 0
        return
    end

    delta = list.nodes[pos] % (length(list.nodes) - 1)

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
    for _ in 1:(n%length(list.nodes))
        pos = list.next[pos]
    end
    list.nodes[pos]
end


function p1(list::CircularDoubleLinkedList)
    list = deepcopy(list)

    for pos = 1:length(list.nodes)
        move(list, pos)
    end

    zero_pos = findfirst(x -> x == 0, list.nodes)
    map(idx -> next_by_n(list, zero_pos, idx), [1000, 2000, 3000]) |> sum
end


function p2(list::CircularDoubleLinkedList)
    list = deepcopy(list)
    list.nodes = list.nodes .* 811589153

    for _ in 1:10
        for pos = 1:length(list.nodes)
            move(list, pos)
        end
    end

    zero_pos = findfirst(x -> x == 0, list.nodes)
    map(idx -> next_by_n(list, zero_pos, idx), [1000, 2000, 3000]) |> sum
end


input = parse_input("inputs/d20")
test_input = parse_input("inputs/d20_test")

@assert p1(test_input) == 3
@time @assert @show p1(input) == 1591

@assert p2(test_input) == 1623178306
@time @assert @show p2(input) == 14579387544492
