const char_to_num_chfen = (c) => {
    switch (c) {
        case 'B':
            return 11;
            break;
        case 'A':
            return 10;
            break;
        case '9':
            return 9;
            break;
        case '8':
            return 8;
            break;
        case '7':
            return 7;
            break;
        case '6':
            return 6;
            break;
        case '5':
            return 5;
            break;
        case '4':
            return 4;
            break;
        case '3':
            return 3;
            break;
        case '2':
            return 2;
            break;
        case '1':
            return 1;
            break;
        default:
            return -1;
            break;
    }
}

export const boardFromCHFEN = (chfen) => {
    const chfen_board = chfen.slice(0,-4);

    let board = [];
    for (let i = 0; i < 11; i++) {
        board[i] = [];
        for (let j = 0; j < 11; j++) {
            board[i][j] = '_';
        }
    }
    board[0][0] = 'x';
    board[0][10] = 'x';
    board[10][0] = 'x';
    board[10][10] = 'x';
    board[5][5] = 'x';
    
    let col = 0;
    let row = 0;
    for (let ind in chfen_board) {
        switch (chfen_board[ind]) {
            case '/':
                row++;
                col = 0;
                break;
            case 'b':
                board[row][col] = 'b';
                col++; 
                break;
            case 'w':
                board[row][col] = 'w';
                col++;
                break;
            case 'k':
                board[row][col] = 'k';
                col++;
                break;
            default:
                let n = char_to_num_chfen(chfen_board[ind]);
                if (n < 1 || n > 11) {
                    break;
                }
                col += n;
                break;
        }
    }

    return board;
}
