import React from 'react';
import axios from 'axios';

import empty from '../images/PH_empty.png';
import black from '../images/PH_black.png';
import corner from '../images/PH_corner.png';
import king from '../images/PH_king.png';
import white from '../images/PH_white.png';

import {boardFromCHFEN} from '../util/boardFromCHFEN';

const BACKEND_URL = `${process.env.REACT_APP_SERVER_URL}:${process.env.REACT_APP_SERVER_PORT}`;

const tile_to_img = (tile) => {
    switch (tile) {
        case 'x':
            return corner;
            break;
        case '_':
            return empty;
            break;
        case 'w':
            return white;
            break;
        case 'k':
            return king;
            break;
        case 'b':
            return black;
            break;
        default:
            return empty;
            break;
    }
}
const Board = ({playerName, gameId, setGameId}) => {
    const [board, setBoard] = React.useState([[]]);

    React.useEffect(() => {
        const get_board = async () => {
            let response = await axios({
                method: "POST",
                url: `${BACKEND_URL}/get_board`, 
                headers: {
                    'Accept': 'application/json',
                    'Content-Type': 'application/json;charset=UTF-8'
                },
                data: {
                    game_id: gameId,
                },
            });

            setBoard(boardFromCHFEN(response.data));
        };

        get_board();
    }, []);

    const handleTileOnClick = (x,y) => {
        alert(`Clicked on tile ${x}, ${y}`);
    }

    return(
        <div>
            <div className="flex items-center justify-between text-black font-bold py-2 px-4 bg-amber-500 w-full h-[50px]">
                <a
                    className="cursor-pointer hover:text-slate-700"
                    onClick={() => {setGameId("")}}
                >
                    {"<< Abort game"}
                </a>
                <span>{playerName}</span>
            </div>

            <div className="text-center w-full">
                <div
                    className="max-w-[330px] mx-4 my-2 grid grid-cols-11"
                >
                    {
                    board.map((row, x) => {
                        return row.map((tile, y) => 
                            <img 
                                key={`${tile}-${tile}-${x}-${y}`}
                                className="mr-0 ml-0 h-[30px] w-[30px] m-0"
                                src={tile_to_img(tile)}
                                onClick={() => handleTileOnClick(x,y)}
                            />
                        );
                    }) 
                }
                </div>
            </div>
        </div>
    )

}

export default Board;

