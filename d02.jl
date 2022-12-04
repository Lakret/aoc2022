function parse_input()
    inp = read("inputs/d02", String) |> chomp
    map(x -> split(x), split(inp, "\n"))
end
    
decipher = Dict(
    "A" => "Rock",
    "B" => "Paper",
    "C" => "Scissors",
    "X" => "Rock",
    "Y" => "Paper",
    "Z" => "Scissors"
)

sign_points = Dict("Rock" => 1, "Paper" => 2, "Scissors" => 3)
win_signs = Dict("Scissors" => "Rock", "Paper" => "Scissors", "Rock" => "Paper")

function score_move(move)
    deciphered_move = map(x -> decipher[x], move)

    round_outcome =
        if deciphered_move[1] == deciphered_move[2]
            3 # draw
        elseif win_signs[deciphered_move[1]] == deciphered_move[2]
            6 # win
        else
            0 # loss
        end

    round_outcome + sign_points[deciphered_move[2]]
end

p1(inp) = map(score_move, inp) |> sum


p2_round_points = Dict("X" => 0, "Y" => 3, "Z" => 6)
lose_signs = map(reverse, collect(win_signs)) |> Dict

function score_move2(move)
    opponent_sign = decipher[move[1]]
    outcome = p2_round_points[move[2]]

    sign =
        if outcome == 3
            opponent_sign
        elseif outcome == 6
            win_signs[opponent_sign]
        else
            lose_signs[opponent_sign]
        end

    outcome + sign_points[sign]
end

p2(inp) = map(score_move2, inp) |> sum


test_input = [["A", "Y"], ["B", "X"], ["C", "Z"]]
inp = parse_input()

@assert p1(test_input) == 15
@assert @show p1(inp) == 10595

@assert p2(test_input) == 12
@assert @show p2(inp) == 9541
