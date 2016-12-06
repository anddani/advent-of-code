let g:Moves = { 'U': [-1, 0], 'R': [0, 1], 'D': [1, 0], 'L': [0, -1] }

let g:Keypad1 = [[1, 2, 3],
\                [4, 5, 6],
\                [7, 8, 9]]

let g:Keypad2 = [[ "",  "", "1",  "",  ""],
\                [ "", "2", "3", "4",  ""],
\                ["5", "6", "7", "8", "9"],
\                [ "", "A", "B", "C",  ""],
\                [ "",  "", "D",  "",  ""]]

function! Part1()
    execute "normal! gg0"
    let c_pos = [1, 1]
    let result = ""

    " For each line in input using GNU seq
    for c_row in split(system('seq 1 '.line('$')))
        let line = getline(c_row)
        for c_char in split(line, '\zs')
            let move_op = g:Moves[c_char]
            let new_row = c_pos[0] + move_op[0]
            let new_col = c_pos[1] + move_op[1]

            if new_row >= 0 && new_row <= 2 && new_col >= 0 && new_col <= 2
                let c_pos[0] = new_row
                let c_pos[1] = new_col
            endif
        endfor
        let result .= g:Keypad1[c_pos[0]][c_pos[1]]
    endfor
    echo result
endfunction

function! Part2()
    execute "normal! gg0"
    let c_pos = [2, 0]
    let result = ""

    " For each line in input using GNU seq
    for c_row in split(system('seq 1 '.line('$')))
        let line = getline(c_row)
        for c_char in split(line, '\zs')
            let move_op = g:Moves[c_char]
            let new_row = c_pos[0] + move_op[0]
            let new_col = c_pos[1] + move_op[1]

            if new_col >= 0 && new_col <= 4 && new_row >= 0 && new_row <= 4
                if g:Keypad2[new_row][new_col] != ""
                    let c_pos[0] = new_row
                    let c_pos[1] = new_col
                endif
            endif
        endfor
        let result .= g:Keypad2[c_pos[0]][c_pos[1]]
    endfor
    echo result
endfunction
