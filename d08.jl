using Pipe

parse_input(input) = @pipe input |> chomp |> split .|> collect |> mapreduce(permutedims, vcat, _) |> parse.(Int32, _)


function get_ltor_visibility(input::Matrix{Int32})::BitMatrix
    (row_max, col_max) = size(input)
    visibility_mask = falses(row_max, col_max)

    for row_idx = 1:row_max
        prev_heighest = -1
        for col_idx = 1:col_max
            if input[row_idx, col_idx] > prev_heighest
                prev_heighest = input[row_idx, col_idx]
                visibility_mask[row_idx, col_idx] = true
            end
        end
    end

    visibility_mask
end

function get_visiblity_mask(input::Matrix{Int32})::BitMatrix
    ltor_mask = get_ltor_visibility(input)
    rtol_mask = @pipe input |> reverse(_, dims=2) |> get_ltor_visibility |> reverse(_, dims=2)
    ttob_mask = @pipe input |> permutedims |> get_ltor_visibility |> permutedims
    btot_mask = @pipe input |> permutedims |> reverse(_, dims=2) |> get_ltor_visibility |>
                      reverse(_, dims=2) |> permutedims

    ltor_mask .|| rtol_mask .|| ttob_mask .|| btot_mask
end

p1(input::Matrix{Int32})::Int32 = input |> get_visiblity_mask |> sum


function p2(input::Matrix{Int32})
    max_score = 0
    for row_idx = 1:size(input)[1], col_idx = 1:size(input)[2]
        left_score = 0
        for j = (col_idx-1):-1:1
            left_score += 1
            if input[row_idx, j] >= input[row_idx, col_idx]
                break
            end
        end

        right_score = 0
        for j = (col_idx+1):size(input)[2]
            right_score += 1
            if input[row_idx, j] >= input[row_idx, col_idx]
                break
            end
        end

        top_score = 0
        for i = (row_idx-1):-1:1
            top_score += 1
            if input[i, col_idx] >= input[row_idx, col_idx]
                break
            end
        end

        bottom_score = 0
        for i = (row_idx+1):size(input)[1]
            bottom_score += 1
            if input[i, col_idx] >= input[row_idx, col_idx]
                break
            end
        end

        score = left_score * right_score * top_score * bottom_score
        if score > max_score
            max_score = score
        end
    end

    max_score
end


input = read("inputs/d08", String) |> parse_input
test_input = """
             30373
             25512
             65332
             33549
             35390
             """ |> parse_input

@assert p1(test_input) == 21
@assert @show p1(input) == 1690

@assert p2(test_input) == 8
@assert @show p2(input) == 535680
