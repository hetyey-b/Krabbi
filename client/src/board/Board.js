import React from 'react';
import axios from 'axios';

import empty from '../images/PH_empty.png';
import black from '../images/PH_black.png';
import corner from '../images/PH_corner.png';
import king from '../images/PH_king.png';
import white from '../images/PH_white.png';

const BACKEND_URL = `${process.env.REACT_APP_SERVER_URL}:${process.env.REACT_APP_SERVER_PORT}`;

const BOARD = [
    ['X','.','.','B','B','B','B','B','.','.','X'],
    ['.','.','.','.','.','B','.','.','.','.','.'],
    ['.','.','.','.','.','.','.','.','.','.','.'],
    ['B','.','.','.','.','W','.','.','.','.','B'],
    ['B','.','.','.','W','W','W','.','.','.','B'],
    ['B','B','.','W','W','K','W','W','.','B','B'],
    ['B','.','.','.','W','W','W','.','.','.','B'],
    ['B','.','.','.','.','W','.','.','.','.','B'],
    ['.','.','.','.','.','.','.','.','.','.','.'],
    ['.','.','.','.','.','B','.','.','.','.','.'],
    ['X','.','.','B','B','B','B','B','.','.','X'],
]

const tile_to_img = (tile) => {
    switch (tile) {
        case 'X':
            return corner;
            break;
        case '.':
            return empty;
            break;
        case 'T':
            return corner;
            break;
        case 'W':
            return white;
            break;
        case 'K':
            return king;
            break;
        case 'B':
            return black;
            break;
        default:
            return empty;
            break;
    }
}

const Board = ({playerName, setPlayerName}) => {
    const [tiles, setTiles] = React.useState([[]]);

    React.useEffect(() => {

    }, []);

    const handleAbortOnClick = () => {
        setPlayerName("");
    };

    return(
        <div className="flex items-center flex-col">
            <div
                className="flex w-full justify-between bg-amber-600 px-4 py-2"
            >
                <a
                    className="cursor-pointer"
                    onClick={handleAbortOnClick}
                >
                    {"<< Abort game"}
                </a>
                <span className="font-bold">{playerName}</span>
            </div>
            <div className="max-w-[330px] text-center m-2 columns-11 gap-0">
                {
                    BOARD.map((row, x) => {
                        return row.map((tile, y) => 
                            <img 
                                key={`${tile}-${tile}-${x}-${y}`}
                                className="mr-0 ml-0 h-[30px] w-[30px] m-0"
                                src={tile_to_img(tile)}
                            />
                        );
                    }) 
                }
            </div>
        </div>
    )

}

export default Board;

