let g:Moves = { 'U': [-1, 0], 'R': [0, 1], 'D': [1, 0], 'L': [0, -1] }

let g:Keypad1 = [[1, 2, 3],
\                [4, 5, 6],
\                [7, 8, 9]]

function! Part1()
    execute "normal! gg0"
    let current_pos = [1, 1]
    let result = ""

    " For each line in input using GNU seq
    for c_row in split(system('seq 1 '.line('$')))
        let line = getline(c_row)
        for c_char in split(line, '\zs')
            let move_op = g:Moves[c_char]
            let current_pos[0] += move_op[0]
            let current_pos[1] += move_op[1]

            " Ensure to not go outside keypad
            if current_pos[0] > 2
                let current_pos[0] = 2
            elseif current_pos[0] < 0
                let current_pos[0] = 0
            elseif current_pos[1] > 2
                let current_pos[1] = 2
            elseif current_pos[1] < 0
                let current_pos[1] = 0
            endif
        endfor
        let result .= g:Keypad1[current_pos[0]][current_pos[1]]
    endfor
    echo result
endfunction

function! Part2()
    execute "normal! gg0"
endfunction
